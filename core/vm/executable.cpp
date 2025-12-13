/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include "executable.hpp"

#include <cassert>
#include <iomanip>
#include <iostream>
#include <libassert/assert.hpp>
#include <limits>
#include <unordered_map>

#include "diagnostics.hpp"
#include "ir/tree.hpp"
#include "module/manager.hpp"
#include "module/module.hpp"
#include "sema/const_value.hpp"
#include "sema/type.hpp"
#include "support/ansi.hpp"
#include "support/bit.hpp"
#include "vm/instruction.hpp"

void via::detail::set_null_dst_trap(
    via::Executable& exe, const std::optional<uint16_t>& dst) noexcept {
  DEBUG_ASSERT_VAL(dst,
                   "destination register must not be null in this context");
}

template <>
void via::Executable::lower_expr<via::ir::ExprConstant>(
    const ir::ExprConstant* ir_expr_constant, std::optional<uint16_t> dst) {
  detail::set_null_dst_trap(*this, dst);

  const ConstValue& cvalue = ir_expr_constant->value;

  switch (cvalue.kind()) {
    case NIL:
      push_instruction(OpCode::LOADNIL, {*dst});
      break;
    case BOOL:
      push_instruction(
          cvalue.unwrap<BOOL>() ? OpCode::LOADTRUE : OpCode::LOADFALSE, {*dst});
      break;
    case INT: {
      int64_t integer = cvalue.unwrap<INT>();
      if (integer <= std::numeric_limits<int32_t>::max() &&
          integer >= std::numeric_limits<int32_t>::min()) {
        uint16_t b, c;
        int32_t val32 = static_cast<int32_t>(integer);  // preserve sign
        unpack_halves(static_cast<uint32_t>(val32), b, c);
        push_instruction(OpCode::LOADINT, {*dst, b, c});
        break;
      }
      [[fallthrough]];
    }
    default:
      push_constant(cvalue);
      push_instruction(OpCode::LOADK, {*dst, (uint16_t)constant_id()});
      break;
  }
}

template <>
void via::Executable::lower_expr<via::ir::ExprSymbol>(
    const ir::ExprSymbol* ir_expr_symbol, std::optional<uint16_t> dst) {
  detail::set_null_dst_trap(*this, dst);

  auto& frame = m_stack.top();
  if (auto lref = frame.get_local(ir_expr_symbol->symbol)) {
    push_instruction(OpCode::GETLOCAL, {*dst, lref->id});
    return;
  }

  UNREACHABLE("unimplemented ir symbol lookup");
}

template <>
void via::Executable::lower_expr<via::ir::ExprModuleAccess>(
    const ir::ExprModuleAccess* ir_expr_module_access,
    std::optional<uint16_t> dst) {
  detail::set_null_dst_trap(*this, dst);

  push_instruction(OpCode::GETIMPORT,
                   {
                       *dst,
                       static_cast<uint16_t>(ir_expr_module_access->mod_id),
                       static_cast<uint16_t>(ir_expr_module_access->key_id),
                   });
}

template <>
void via::Executable::lower_expr<via::ir::ExprBinary>(
    const ir::ExprBinary* ir_expr_binary, std::optional<uint16_t> dst) {
  detail::set_null_dst_trap(*this, dst);

  uint16_t opid = static_cast<uint16_t>(ir_expr_binary->op);
  uint16_t rlhs = m_reg_state.alloc(), rrhs = m_reg_state.alloc();

  lower_expr(ir_expr_binary->lhs, rlhs);
  lower_expr(ir_expr_binary->rhs, rrhs);

  if (opid >= static_cast<uint16_t>(BinaryOp::ADD) &&
      opid <= static_cast<uint16_t>(BinaryOp::MOD)) {
    /* TODO: Check if rhs is constexpr, in which case increment base by one
     * for K instructions*/
    uint16_t base = static_cast<uint16_t>(OpCode::IADD) +
                    static_cast<uint16_t>(ir_expr_binary->op);

    if (ir_expr_binary->lhs->type.unwrap()->is_integral()) {
      if (ir_expr_binary->rhs->type.unwrap()->is_float()) {
        base += 2;  // FP mode
        push_instruction(OpCode::TOFLOAT, {rlhs, rlhs});
      }
    } else {
      base += 2;  // FP mode

      if (ir_expr_binary->rhs->type.unwrap()->is_integral()) {
        push_instruction(OpCode::TOFLOAT, {rrhs, rrhs});
      }
    }

    push_instruction(static_cast<OpCode>(base), {*dst, rlhs, rrhs});
  } else if (opid >= static_cast<uint16_t>(BinaryOp::AND) &&
             opid <= static_cast<uint16_t>(BinaryOp::OR)) {
    /* TODO: Check if rhs is constexpr, in which case increment base by one
     * for K instructions*/
    uint16_t base = static_cast<uint16_t>(OpCode::AND) +
                    static_cast<uint16_t>(ir_expr_binary->op);
    push_instruction(static_cast<OpCode>(base), {*dst, rlhs, rrhs});
  } else if (opid >= static_cast<uint16_t>(BinaryOp::BAND) &&
             opid <= static_cast<uint16_t>(BinaryOp::BSHR)) {
    /* TODO: Check if rhs is constexpr, in which case increment base by one
     * for K instructions*/
    uint16_t base = static_cast<uint16_t>(OpCode::BAND) +
                    static_cast<uint16_t>(ir_expr_binary->op);
    push_instruction(static_cast<OpCode>(base), {*dst, rlhs, rrhs});
  }

  push_instruction(OpCode::FREE2, {rlhs, rrhs});
  m_reg_state.free_all(rlhs, rrhs);
}

template <>
void via::Executable::lower_expr<via::ir::ExprCall>(
    const ir::ExprCall* ir_expr_call, std::optional<uint16_t> dst) {
  uint16_t callee = m_reg_state.alloc();
  auto args = ir_expr_call->args;
  std::reverse(args.begin(), args.end());

  for (const auto& arg : args) {
    lower_expr(arg, callee);
    push_instruction(OpCode::PUSH, {callee});
  }

  lower_expr(ir_expr_call->callee, callee);
  push_instruction(OpCode::CALL, {callee});
  push_instruction(OpCode::FREE1, {callee});
  m_reg_state.free(callee);

  if (dst.has_value()) {
    push_instruction(OpCode::GETTOP, {*dst});
  }
}

template <>
void via::Executable::lower_expr<via::ir::ExprCast>(
    const ir::ExprCast* ir_expr_cast, std::optional<uint16_t> dst) {
  using enum BuiltinKind;

  detail::set_null_dst_trap(*this, dst);
  lower_expr(ir_expr_cast->expr, dst);

  if (ir_expr_cast->cast == ir_expr_cast->expr->type) {
    // Redundant cast
    return;
  }

  auto& type_ctx = m_module->manager().type_context();

  if TRY_COERCE (const BuiltinType, cast_bultin_type,
                 ir_expr_cast->cast.unwrap()) {
    if TRY_COERCE (const BuiltinType, expr_builtin_type,
                   ir_expr_cast->expr->type.unwrap()) {
      static std::unordered_map<const Type*, OpCode> cast_rules = {
          {type_ctx.instance<BuiltinType>(BuiltinKind::INT), OpCode::TOINT},
          {type_ctx.instance<BuiltinType>(BuiltinKind::FLOAT), OpCode::TOFLOAT},
          {type_ctx.instance<BuiltinType>(BuiltinKind::BOOL), OpCode::TOBOOL},
          {type_ctx.instance<BuiltinType>(BuiltinKind::STRING),
           OpCode::TOSTRING},
      };

      if (auto it = cast_rules.find(cast_bultin_type); it != cast_rules.end()) {
        push_instruction(it->second, {*dst, *dst});
      } else {
        UNREACHABLE("unmapped builtin cast directive");
      }
    }
  }
}

void via::Executable::lower(const ir::Expr* expr, std::optional<uint16_t> dst) {
#define VISIT_EXPR(TYPE) \
  if TRY_COERCE (const TYPE, _INNER, expr) return lower_expr<TYPE>(_INNER, dst);

  VISIT_EXPR(ir::ExprConstant);
  VISIT_EXPR(ir::ExprSymbol);
  VISIT_EXPR(ir::ExprAccess);
  VISIT_EXPR(ir::ExprModuleAccess);
  VISIT_EXPR(ir::ExprUnary);
  VISIT_EXPR(ir::ExprBinary);
  VISIT_EXPR(ir::ExprCall);
  VISIT_EXPR(ir::ExprSubscript);
  VISIT_EXPR(ir::ExprCast);
  VISIT_EXPR(ir::ExprTernary);
  VISIT_EXPR(ir::ExprArray);
  VISIT_EXPR(ir::ExprTuple);
  VISIT_EXPR(ir::ExprLambda);
#undef VISIT_EXPR

  UNREACHABLE(VIA_TYPENAME(*expr));
}

template <>
void via::Executable::lower_stat<via::ir::StatVarDecl>(
    const ir::StatVarDecl* ir_stat_var_decl) {
  auto dst = m_reg_state.alloc();
  lower_expr(ir_stat_var_decl->expr, dst);
  push_instruction(OpCode::PUSH, {dst});
  push_instruction(OpCode::FREE1, {dst});
  m_reg_state.free(dst);

  auto& frame = m_stack.top();
  frame.set_local(ir_stat_var_decl->symbol);
}

template <>
void via::Executable::lower_stat<via::ir::StatInstruction>(
    const ir::StatInstruction* ir_stat_instr) {
  m_bytecode.push_back(ir_stat_instr->instr);
}

template <>
void via::Executable::lower_stat<via::ir::StatBlock>(
    const ir::StatBlock* ir_stat_block) {
  set_label(ir_stat_block->id);
  for (const auto& stat : ir_stat_block->stats) {
    lower_stat(stat);
  }
  if (ir_stat_block->term != nullptr) {
    lower_term(ir_stat_block->term);
  }
}

template <>
void via::Executable::lower_stat<via::ir::StatFuncDecl>(
    const ir::StatFuncDecl* ir_stat_func_decl) {
  m_stack.push({});

  auto dst = m_reg_state.alloc();
  auto pc = push_instruction(OpCode::NOP);
  lower_stat(ir_stat_func_decl->body);

  size_t offset = program_counter() - pc + 1;
  uint16_t high, low;

  unpack_halves(static_cast<uint32_t>(offset), high, low);

  push_instruction(OpCode::PUSH, {dst});
  push_instruction(OpCode::FREE1, {dst});
  set_instruction(pc, OpCode::NEWCLOSURE, {dst, high, low});
  m_reg_state.free(dst);
  m_stack.pop();

  auto& frame = m_stack.top();
  frame.set_local(ir_stat_func_decl->symbol);
}

template <>
void via::Executable::lower_stat<via::ir::StatExpr>(
    const ir::StatExpr* ir_stat_expr) {
  lower_expr(ir_stat_expr->expr, std::nullopt);
}

void via::Executable::lower(const ir::Stat* stat) {
#define VISIT_STMT(TYPE) \
  if TRY_COERCE (const TYPE, _INNER, stat) return lower_stat<TYPE>(_INNER);

  VISIT_STMT(ir::StatVarDecl)
  VISIT_STMT(ir::StatFuncDecl)
  VISIT_STMT(ir::StatInstruction)
  VISIT_STMT(ir::StatBlock)
  VISIT_STMT(ir::StatExpr)
#undef VISIT_STMT

  UNREACHABLE(VIA_TYPENAME(*stat));
}

template <>
void via::Executable::lower_term<via::ir::TrReturn>(
    const ir::TrReturn* ir_term_ret) {
  if (ir_term_ret->val) {
    uint16_t reg = m_reg_state.alloc();
    lower_expr(ir_term_ret->val, reg);
    push_instruction(OpCode::RET, {reg});
    m_reg_state.free(reg);
  } else {
    push_instruction(OpCode::RETNIL);
  }
}

template <>
void via::Executable::lower_term<via::ir::TrBranch>(
    const ir::TrBranch* ir_term_branch) {
  uint16_t high, low;
  unpack_halves(ir_term_branch->target->id, high, low);
  push_instruction(OpCode::JMP, {high, low});
}

template <>
void via::Executable::lower_term<via::ir::TrCondBranch>(
    const ir::TrCondBranch* ir_term_cond_branch) {
  uint16_t thigh, tlow, fhigh, flow;
  unpack_halves(ir_term_cond_branch->iftrue->id, thigh, tlow);
  unpack_halves(ir_term_cond_branch->iffalse->id, fhigh, flow);

  uint16_t reg = m_reg_state.alloc();
  lower_expr(ir_term_cond_branch->cnd, reg);
  push_instruction(OpCode::JMPIF, {reg, thigh, tlow});
  push_instruction(OpCode::JMP, {fhigh, flow});
  m_reg_state.free(reg);
}

void via::Executable::lower(const ir::Term* term) {
#define VISIT_TERM(TYPE) \
  if TRY_COERCE (const TYPE, _INNER, term) return lower_term<TYPE>(_INNER);

  VISIT_TERM(ir::TrReturn);
  VISIT_TERM(ir::TrBranch);
  VISIT_TERM(ir::TrCondBranch);
  VISIT_TERM(ir::TrContinue);
  VISIT_TERM(ir::TrBreak);
#undef VISIT_TERM

  UNREACHABLE(VIA_TYPENAME(*term));
}

via::Executable* via::Executable::build(Module* module, Diagnostics& diags,
                                        const IRTree& ir_tree, ExeFlags flags) {
  auto& alloc = module->allocator();
  auto* exe = alloc.emplace<Executable>(diags);
  exe->m_module = module;
  exe->m_flags = flags;

  for (const auto& stat : ir_tree) {
    exe->lower_stat(stat);
  }

  exe->lower_jumps();
  exe->push_instruction(OpCode::HALT);
  return exe;
}

void via::Executable::lower_jumps() {
  size_t pc = 0;
  for (Instruction& instr : m_bytecode) {
    switch (instr.op) {
      case OpCode::JMP: {
        uint32_t label = pack_halves<uint32_t>(instr.a, instr.b);
        uint32_t address = m_labels.at(label);

        uint32_t offset;
        if (address < pc) {
          instr.op = OpCode::JMPBACK;
          offset = pc - address - 1;  // store positive distance
        } else {
          offset = address - pc + 1;
        }

        unpack_halves(offset, instr.a, instr.b);
        break;
      }
      case OpCode::JMPIF:
      case OpCode::JMPIFX: {
        uint32_t label = pack_halves<uint32_t>(instr.b, instr.c);
        uint32_t address = m_labels.at(label);

        uint32_t offset;
        if (address < pc) {
          instr.op = (instr.op == OpCode::JMPIF) ? OpCode::JMPBACKIF
                                                 : OpCode::JMPBACKIFX;
          offset = pc - address - 1;  // positive distance
        } else {
          offset = address - pc + 1;
        }

        unpack_halves(offset, instr.b, instr.c);
        break;
      }
      default:
        break;
    }
    ++pc;
  }
}

std::string via::Executable::to_string() const {
  std::ostringstream oss;
  oss << ansi::format("[disassembly of program code]:\n",
                      ansi::Foreground::YELLOW, ansi::Background::NONE,
                      ansi::Style::UNDERLINE);

  oss << ansi::format(
      "  pc      opcode           operands\n"
      "  ------  ---------------  ---------------\n",
      ansi::Foreground::NONE, ansi::Background::NONE, ansi::Style::FAINT);

  for (size_t pc = 0; const Instruction& insn : m_bytecode) {
    oss << "  "
        << ansi::format(std::format("0x{:0>4x}", pc * 8),
                        ansi::Foreground::NONE, ansi::Background::NONE,
                        ansi::Style::FAINT);
    oss << "  " << insn.to_string(true, pc) << "\n";
    pc++;
  }
  oss << ansi::format("\n[disassembly of program data]:\n",
                      ansi::Foreground::YELLOW, ansi::Background::NONE,
                      ansi::Style::UNDERLINE);

  oss << ansi::format(
      "  id      type        data\n"
      "  ------  ----------  ---------------\n",
      ansi::Foreground::NONE, ansi::Background::NONE, ansi::Style::FAINT);

  for (size_t i = 0; const auto& cv : m_constants) {
    oss << "  "
        << ansi::format(std::format("0x{:0>4x}", i++), ansi::Foreground::NONE,
                        ansi::Background::NONE, ansi::Style::FAINT);
    oss << "  " << std::left << std::setw(21)
        << ansi::format(std::string(via::to_string(cv.kind())),
                        ansi::Foreground::MAGENTA, ansi::Background::NONE,
                        ansi::Style::BOLD);
    oss << "  "
        << ansi::format(std::string(cv.to_string()), ansi::Foreground::GREEN);
    oss << "\n";
  }
  return oss.str();
}
