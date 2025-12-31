/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include "executable.hpp"

#include <ansi.hpp>
#include <bit.hpp>
#include <cassert>
#include <compiler/ir-tree.hpp>
#include <compiler/type.hpp>
#include <compiler/value.hpp>
#include <diagnostics.hpp>
#include <limits>
#include <module/manager.hpp>
#include <module/module.hpp>
#include <unordered_map>
#include <vm/instruction.hpp>

template <>
void via::Executable::lower_expr<via::ir::Expr::Constant>(
    const ir::Expr::Constant* ir_expr_constant, std::optional<uint16_t> dst) {
  VIA_DEBUG_ASSERT(dst != std::nullopt,
                   "destination register must not be null in this context");

  const ConstValue& cvalue = ir_expr_constant->value;

  switch (cvalue.kind()) {
    case NONE:
      push(OpCode::LOADNONE, {*dst});
      break;
    case BOOL:
      push(cvalue.unwrap<BOOL>() ? OpCode::LOADTRUE : OpCode::LOADFALSE,
           {*dst});
      break;
    case INT: {
      int64_t integer = cvalue.unwrap<INT>();
      if (integer <= std::numeric_limits<int32_t>::max() &&
          integer >= std::numeric_limits<int32_t>::min()) {
        uint16_t b, c;
        int32_t val32 = static_cast<int32_t>(integer);  // preserve sign
        unpack_halves(static_cast<uint32_t>(val32), b, c);
        push(OpCode::LOADINT, {*dst, b, c});
        break;
      }
      [[fallthrough]];
    }
    default:
      push(cvalue);
      push(OpCode::LOADK, {*dst, (uint16_t)constant_id()});
      break;
  }
}

template <>
void via::Executable::lower_expr<via::ir::Expr::Symbol>(
    const ir::Expr::Symbol* ir_expr_symbol, std::optional<uint16_t> dst) {
  VIA_DEBUG_ASSERT(dst != std::nullopt,
                   "destination register must not be null in this context");

  auto& frame = m_stack.top();
  if (auto lref = frame.get_local(ir_expr_symbol->symbol)) {
    push(OpCode::GETLOCAL, {*dst, lref->id});
    return;
  }
  VIA_PANIC("unimplemented ir symbol lookup");
}

template <>
void via::Executable::lower_expr<via::ir::Expr::ModuleAccess>(
    const ir::Expr::ModuleAccess* ir_expr_module_access,
    std::optional<uint16_t> dst) {
  VIA_DEBUG_ASSERT(dst != std::nullopt,
                   "destination register must not be null in this context");

  push(OpCode::GETIMPORT,
       {
           *dst,
           static_cast<uint16_t>(ir_expr_module_access->mod_id),
           static_cast<uint16_t>(ir_expr_module_access->key_id),
       });
}

template <>
void via::Executable::lower_expr<via::ir::Expr::Binary>(
    const ir::Expr::Binary* ir_expr_binary, std::optional<uint16_t> dst) {
  VIA_DEBUG_ASSERT(dst != std::nullopt,
                   "destination register must not be null in this context");

  uint16_t opid = static_cast<uint16_t>(ir_expr_binary->op);
  uint16_t rlhs = m_reg_state.alloc(), rrhs = m_reg_state.alloc();

  lower(ir_expr_binary->lhs, rlhs);
  lower(ir_expr_binary->rhs, rrhs);

  if (opid >= static_cast<uint16_t>(BinaryOp::ADD) &&
      opid <= static_cast<uint16_t>(BinaryOp::MOD)) {
    /* TODO: Check if rhs is constexpr, in which case increment base by one
     * for K instructions*/
    uint16_t base = static_cast<uint16_t>(OpCode::IADD) +
                    static_cast<uint16_t>(ir_expr_binary->op);

    if (ir_expr_binary->lhs->type.unwrap()->is_integral()) {
      if (ir_expr_binary->rhs->type.unwrap()->is_float()) {
        base += 2;  // FP mode
        push(OpCode::TOFLOAT, {rlhs, rlhs});
      }
    } else {
      base += 2;  // FP mode

      if (ir_expr_binary->rhs->type.unwrap()->is_integral()) {
        push(OpCode::TOFLOAT, {rrhs, rrhs});
      }
    }

    push(static_cast<OpCode>(base), {*dst, rlhs, rrhs});
  } else if (opid >= static_cast<uint16_t>(BinaryOp::AND) &&
             opid <= static_cast<uint16_t>(BinaryOp::OR)) {
    /* TODO: Check if rhs is constexpr, in which case increment base by one
     * for K instructions*/
    uint16_t base = static_cast<uint16_t>(OpCode::AND) +
                    static_cast<uint16_t>(ir_expr_binary->op);
    push(static_cast<OpCode>(base), {*dst, rlhs, rrhs});
  } else if (opid >= static_cast<uint16_t>(BinaryOp::BAND) &&
             opid <= static_cast<uint16_t>(BinaryOp::BSHR)) {
    /* TODO: Check if rhs is constexpr, in which case increment base by one
     * for K instructions*/
    uint16_t base = static_cast<uint16_t>(OpCode::BAND) +
                    static_cast<uint16_t>(ir_expr_binary->op);
    push(static_cast<OpCode>(base), {*dst, rlhs, rrhs});
  }

  push(OpCode::FREE2, {rlhs, rrhs});
  m_reg_state.free_all(rlhs, rrhs);
}

template <>
void via::Executable::lower_expr<via::ir::Expr::Call>(
    const ir::Expr::Call* ir_expr_call, std::optional<uint16_t> dst) {
  uint16_t callee = m_reg_state.alloc();
  auto args = ir_expr_call->args;
  std::reverse(args.begin(), args.end());

  for (const auto& arg : args) {
    lower(arg, callee);
    push(OpCode::PUSH, {callee});
  }

  lower(ir_expr_call->callee, callee);
  push(OpCode::CALL, {callee});
  push(OpCode::FREE1, {callee});
  m_reg_state.free(callee);

  if (dst.has_value()) {
    push(OpCode::GETTOP, {*dst});
  }
}

template <>
void via::Executable::lower_expr<via::ir::Expr::Cast>(
    const ir::Expr::Cast* ir_expr_cast, std::optional<uint16_t> dst) {
  using enum BuiltinKind;

  VIA_DEBUG_ASSERT(dst != std::nullopt,
                   "destination register must not be null in this context");

  lower(ir_expr_cast->expr, dst);

  if (ir_expr_cast->cast == ir_expr_cast->expr->type) {
    // Redundant cast
    return;
  }

  auto& type_ctx = m_module.manager().type_context();

  if VIA_TRY_COERCE (const BuiltinType, cast_bultin_type,
                     ir_expr_cast->cast.unwrap()) {
    if VIA_TRY_COERCE (const BuiltinType, expr_builtin_type,
                       ir_expr_cast->expr->type.unwrap()) {
      static std::unordered_map<const Type*, OpCode> cast_rules = {
          {type_ctx.instance<BuiltinType>(BuiltinKind::INT), OpCode::TOINT},
          {type_ctx.instance<BuiltinType>(BuiltinKind::FLOAT), OpCode::TOFLOAT},
          {type_ctx.instance<BuiltinType>(BuiltinKind::BOOL), OpCode::TOBOOL},
          {type_ctx.instance<BuiltinType>(BuiltinKind::STRING),
           OpCode::TOSTRING},
      };

      if (auto it = cast_rules.find(cast_bultin_type); it != cast_rules.end()) {
        push(it->second, {*dst, *dst});
      } else {
        VIA_PANIC("unmapped builtin cast directive");
      }
    }
  }
}

void via::Executable::lower(const ir::Expr* expr, std::optional<uint16_t> dst) {
#define VISIT_EXPR(TYPE)                       \
  if VIA_TRY_COERCE (const TYPE, _INNER, expr) \
    return lower_expr<TYPE>(_INNER, dst);

  VISIT_EXPR(ir::Expr::Constant);
  VISIT_EXPR(ir::Expr::Symbol);
  VISIT_EXPR(ir::Expr::Access);
  VISIT_EXPR(ir::Expr::ModuleAccess);
  VISIT_EXPR(ir::Expr::Unary);
  VISIT_EXPR(ir::Expr::Binary);
  VISIT_EXPR(ir::Expr::Call);
  VISIT_EXPR(ir::Expr::Subscript);
  VISIT_EXPR(ir::Expr::Cast);
  VISIT_EXPR(ir::Expr::Ternary);
  VISIT_EXPR(ir::Expr::Array);
  VISIT_EXPR(ir::Expr::Tuple);
  VISIT_EXPR(ir::Expr::Lambda);
#undef VISIT_EXPR

  VIA_PANIC(VIA_TYPENAME(*expr));
}

template <>
void via::Executable::lower_stat<via::ir::Stmt::VarDecl>(
    const ir::Stmt::VarDecl* ir_stat_var_decl) {
  auto dst = m_reg_state.alloc();
  lower(ir_stat_var_decl->expr, dst);
  push(OpCode::PUSH, {dst});
  push(OpCode::FREE1, {dst});
  m_reg_state.free(dst);

  auto& frame = m_stack.top();
  frame.set_local(ir_stat_var_decl->symbol);
}

template <>
void via::Executable::lower_stat<via::ir::Stmt::Instruction>(
    const ir::Stmt::Instruction* ir_stat_instr) {
  m_bytecode.push_back(ir_stat_instr->instr);
}

template <>
void via::Executable::lower_stat<via::ir::Stmt::Block>(
    const ir::Stmt::Block* ir_stat_block) {
  label(ir_stat_block->id);
  for (const auto& stat : ir_stat_block->stats) {
    lower(stat);
  }
  if (ir_stat_block->term != nullptr) {
    lower(ir_stat_block->term);
  }
}

template <>
void via::Executable::lower_stat<via::ir::Stmt::FuncDecl>(
    const ir::Stmt::FuncDecl* ir_stat_func_decl) {
  m_stack.push({});

  auto dst = m_reg_state.alloc();
  auto pc = push(OpCode::NOP);
  lower(ir_stat_func_decl->body);

  size_t offset = this->pc() - pc + 1;
  uint16_t high, low;

  unpack_halves(static_cast<uint32_t>(offset), high, low);

  push(OpCode::PUSH, {dst});
  push(OpCode::FREE1, {dst});
  modify(pc, OpCode::NEWCLOSURE, {dst, high, low});
  m_reg_state.free(dst);
  m_stack.pop();

  auto& frame = m_stack.top();
  frame.set_local(ir_stat_func_decl->ident);
}

template <>
void via::Executable::lower_stat<via::ir::Stmt::Expr>(
    const ir::Stmt::Expr* ir_stat_expr) {
  lower(ir_stat_expr->expr, std::nullopt);
}

void via::Executable::lower(const ir::Stmt* stat) {
#define VISIT_STMT(TYPE) \
  if VIA_TRY_COERCE (const TYPE, _INNER, stat) return lower_stat<TYPE>(_INNER);

  VISIT_STMT(ir::Stmt::VarDecl)
  VISIT_STMT(ir::Stmt::FuncDecl)
  VISIT_STMT(ir::Stmt::Instruction)
  VISIT_STMT(ir::Stmt::Block)
  VISIT_STMT(ir::Stmt::Expr)
#undef VISIT_STMT

  VIA_PANIC(VIA_TYPENAME(*stat));
}

template <>
void via::Executable::lower_term<via::ir::Term::Return>(
    const ir::Term::Return* ir_term_ret) {
  if (ir_term_ret->val) {
    uint16_t reg = m_reg_state.alloc();
    lower(ir_term_ret->val, reg);
    push(OpCode::RET, {reg});
    m_reg_state.free(reg);
  } else {
    push(OpCode::RETNONE);
  }
}

template <>
void via::Executable::lower_term<via::ir::Term::Branch>(
    const ir::Term::Branch* ir_term_branch) {
  uint16_t high, low;
  unpack_halves(ir_term_branch->target->id, high, low);
  push(OpCode::JMP, {high, low});
}

template <>
void via::Executable::lower_term<via::ir::Term::CondBranch>(
    const ir::Term::CondBranch* ir_term_cond_branch) {
  uint16_t thigh, tlow, fhigh, flow;
  unpack_halves(ir_term_cond_branch->iftrue->id, thigh, tlow);
  unpack_halves(ir_term_cond_branch->iffalse->id, fhigh, flow);

  uint16_t reg = m_reg_state.alloc();
  lower(ir_term_cond_branch->cnd, reg);
  push(OpCode::JMPIF, {reg, thigh, tlow});
  push(OpCode::JMP, {fhigh, flow});
  m_reg_state.free(reg);
}

void via::Executable::lower(const ir::Term* term) {
#define VISIT_TERM(TYPE) \
  if VIA_TRY_COERCE (const TYPE, _INNER, term) return lower_term<TYPE>(_INNER);

  VISIT_TERM(ir::Term::Return);
  VISIT_TERM(ir::Term::Branch);
  VISIT_TERM(ir::Term::CondBranch);
  VISIT_TERM(ir::Term::Continue);
  VISIT_TERM(ir::Term::Break);
#undef VISIT_TERM

  VIA_PANIC(VIA_TYPENAME(*term));
}

size_t via::Executable::label(size_t id) {
  m_labels[id] = pc();
  return m_labels.size() - 1;
}

void via::Executable::push(ConstValue cvalue) {
  VIA_DEBUG_ASSERT(
      m_constants.size() < (size_t)std::numeric_limits<uint16_t>::max(),
      "Constant count exceeds limit");
  m_constants.push_back(std::move(cvalue));
}

size_t via::Executable::push(OpCode op, std::array<uint16_t, 3> ops) {
  m_bytecode.emplace_back(op, ops[0], ops[1], ops[2]);
  return pc();
}

void via::Executable::modify(size_t pc, OpCode op,
                             std::array<uint16_t, 3> ops) {
  auto& insn = m_bytecode[pc];
  insn.op = op;
  insn.a = ops[0];
  insn.b = ops[1];
  insn.c = ops[2];
}

via::Executable* via::Executable::build(Module& module, const IRTree& ir_tree,
                                        ExeFlags flags) {
  auto& alloc = module.allocator();
  auto* exe = alloc.emplace<Executable>(module);
  exe->m_flags = flags;

  for (const auto& stat : ir_tree) exe->lower(stat);
  exe->lower_jumps();
  exe->push(OpCode::HALT);
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
