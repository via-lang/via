/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include "ir-builder.hpp"

#include <ansi.hpp>
#include <compiler/control-flow.hpp>
#include <diagnostics.hpp>
#include <module/binding.hpp>
#include <type.hpp>
#include <vm/instruction.hpp>

#include "ir-tree.hpp"
#include "syntax-tree.hpp"
#include "type.hpp"

#define UNARY_OP_CASE(VALID, RESULT)                                \
  {                                                                 \
      .is_valid = [](via::QualType type) -> bool { return VALID; }, \
      .get_result = [](via::TypeContext& ctx, via::QualType type)   \
          -> via::QualType { return RESULT; },                      \
  }

struct UnaryOpInfo {
  std::function<bool(via::QualType)> is_valid;  // predicate
  std::function<via::QualType(via::TypeContext&, via::QualType)>
      get_result;  // compute result
};

static const UnaryOpInfo UNARY_OP_TABLE[] = {
    /* UnaryOp::NEG */ UNARY_OP_CASE(type.unwrap()->is_arithmetic(), type),
    /* UnaryOp::NOT */
    UNARY_OP_CASE(true, ctx.instance<via::BuiltinType>(via::BuiltinKind::BOOL)),
    /* UnaryOp::BNOT */ UNARY_OP_CASE(type.unwrap()->is_integral(), type),
};

#define BINARY_OP_CASE(VALID, RESULT)                                          \
  {                                                                            \
      .is_valid = [](via::QualType lhs, via::QualType rhs) -> bool {           \
        return VALID;                                                          \
      },                                                                       \
      .get_result = [](via::TypeContext& ctx, via::QualType lhs,               \
                       via::QualType rhs) -> via::QualType { return RESULT; }, \
  }

struct BinaryOpInfo {
  std::function<bool(via::QualType, via::QualType)> is_valid;  // predicate
  std::function<via::QualType(via::TypeContext&, via::QualType, via::QualType)>
      get_result;  // compute result
};

#define BINARY_OP_PROMOTE(LHS, RHS)                              \
  ctx.instance<via::BuiltinType>(                                \
      ((LHS).unwrap()->is_float() || (RHS).unwrap()->is_float()) \
          ? via::BuiltinKind::FLOAT                              \
          : via::BuiltinKind::INT)

static const BinaryOpInfo BINARY_OP_TABLE[] = {
    /* BinaryOp::ADD */
    BINARY_OP_CASE(
        lhs.unwrap()->is_arithmetic() && rhs.unwrap()->is_arithmetic(),
        BINARY_OP_PROMOTE(lhs, rhs)),
    /* BinaryOp::SUB */
    BINARY_OP_CASE(
        lhs.unwrap()->is_arithmetic() && rhs.unwrap()->is_arithmetic(),
        BINARY_OP_PROMOTE(lhs, rhs)),
    /* BinaryOp::MUL */
    BINARY_OP_CASE(
        lhs.unwrap()->is_arithmetic() && rhs.unwrap()->is_arithmetic(),
        BINARY_OP_PROMOTE(lhs, rhs)),
    /* BinaryOp::DIV */
    BINARY_OP_CASE(
        lhs.unwrap()->is_arithmetic() && rhs.unwrap()->is_arithmetic(),
        ctx.instance<via::BuiltinType>(via::BuiltinKind::BOOL)),
    /* BinaryOp::POW */
    BINARY_OP_CASE(
        lhs.unwrap()->is_arithmetic() && rhs.unwrap()->is_arithmetic(),
        BINARY_OP_PROMOTE(lhs, rhs)),
    /* BinaryOp::MOD */
    BINARY_OP_CASE(lhs.unwrap()->is_integral() && rhs.unwrap()->is_integral(),
                   ctx.instance<via::BuiltinType>(via::BuiltinKind::INT)),
    /* BinaryOp::AND */
    BINARY_OP_CASE(true,
                   ctx.instance<via::BuiltinType>(via::BuiltinKind::BOOL)),
    /* BinaryOp::OR */
    BINARY_OP_CASE(true,
                   ctx.instance<via::BuiltinType>(via::BuiltinKind::BOOL)),
    /* BinaryOp::BAND */
    BINARY_OP_CASE(lhs.unwrap()->is_integral() && rhs.unwrap()->is_integral(),
                   ctx.instance<via::BuiltinType>(via::BuiltinKind::INT)),
    /* BinaryOp::BOR */
    BINARY_OP_CASE(lhs.unwrap()->is_integral() && rhs.unwrap()->is_integral(),
                   ctx.instance<via::BuiltinType>(via::BuiltinKind::INT)),
    /* BinaryOp::BXOR */
    BINARY_OP_CASE(lhs.unwrap()->is_integral() && rhs.unwrap()->is_integral(),
                   ctx.instance<via::BuiltinType>(via::BuiltinKind::INT)),
    /* BinaryOp::BSHL */
    BINARY_OP_CASE(lhs.unwrap()->is_integral() && rhs.unwrap()->is_integral(),
                   ctx.instance<via::BuiltinType>(via::BuiltinKind::INT)),
    /* BinaryOp::BSHR */
    BINARY_OP_CASE(lhs.unwrap()->is_integral() && rhs.unwrap()->is_integral(),
                   ctx.instance<via::BuiltinType>(via::BuiltinKind::INT)),
};

template <>
via::QualType via::IRBuilder::type_of<via::ast::ExprLiteral>(
    const ast::ExprLiteral* ast_expr_literal) {
  using enum TokenKind;
  using enum BuiltinKind;

  BuiltinKind kind;

  switch (ast_expr_literal->tok->kind) {
    case KW_NONE:
      kind = NONE;
      break;
    case KW_TRUE:
    case KW_FALSE:
      kind = BOOL;
      break;
    case LIT_INT:
    case LIT_XINT:
    case LIT_BINT:
      kind = INT;
      break;
    case LIT_FLOAT:
      kind = FLOAT;
      break;
    case LIT_STRING:
      kind = STRING;
      break;
    default:
      VIA_PANIC("invalid literal expression");
  }
  return m_types.instance<via::BuiltinType>(kind);
}

template <>
via::QualType via::IRBuilder::type_of<via::ast::ExprSymbol>(

    const ast::ExprSymbol* ast_expr_symbol) {
  auto& frame = m_stack.top();
  auto symbol = ast_expr_symbol->symbol->to_string();
  auto id = intern(ast_expr_symbol->symbol->to_string());

  if (auto local = frame.get_local(id)) {
    auto* ir_decl = local->local->get_ir_decl();

    if VIA_TRY_COERCE (const ir::Stmt::VarDecl, var_decl, ir_decl) {
      return var_decl->type;
    } else if VIA_TRY_COERCE (const ir::Stmt::FuncDecl, func_decl, ir_decl) {
      std::vector<via::QualType> parms;
      for (const auto& parm : func_decl->parms) parms.push_back(parm.type);
      return m_types.instance<FunctionType>(func_decl->ret, std::move(parms));
    }
  }
  return nullptr;
}

template <>
via::QualType via::IRBuilder::type_of<via::ast::ExprStaticAccess>(

    const ast::ExprStaticAccess* ast_expr_st_access) {
  if VIA_TRY_COERCE (const ast::ExprSymbol, symbol, ast_expr_st_access->root) {
    auto& manager = m_module.manager();
    auto low = intern(symbol->symbol->to_string());

    if (auto module = manager.get_module_by_name(low)) {
      auto high = intern(ast_expr_st_access->index->to_string());

      if (auto bind = module->lookup(high)) {
        if VIA_TRY_COERCE (const FunctionBinding, binding, *bind) {
          std::vector<via::QualType> parm_types;

          for (const auto& parm : binding->m_params)
            parm_types.push_back(parm.type);
          return m_types.instance<FunctionType>(binding->m_return,
                                                std::move(parm_types));
        }
      }
    }
  }
  return nullptr;
}

template <>
via::QualType via::IRBuilder::type_of<via::ast::ExprUnary>(

    const ast::ExprUnary* ast_expr_unary) {
  auto op = to_unary_op(ast_expr_unary->op->kind);
  auto info = UNARY_OP_TABLE[static_cast<uint8_t>(op)];
  auto type = type_of(ast_expr_unary->expr);
  return info.is_valid(type) ? info.get_result(m_types, type) : nullptr;
}

template <>
via::QualType via::IRBuilder::type_of<via::ast::ExprBinary>(

    const ast::ExprBinary* ast_expr_binary) {
  auto op = to_binary_op(ast_expr_binary->op->kind);
  auto info = BINARY_OP_TABLE[static_cast<uint8_t>(op)];
  auto lhs = type_of(ast_expr_binary->lhs), rhs = type_of(ast_expr_binary->rhs);
  return info.is_valid(lhs, rhs) ? info.get_result(m_types, lhs, rhs) : nullptr;
}

template <>
via::QualType via::IRBuilder::type_of<via::ast::ExprCall>(

    const ast::ExprCall* ast_expr_call) {
  auto callee = type_of(ast_expr_call->callee);
  if VIA_TRY_COERCE (const FunctionType, function, callee.unwrap())
    return function->returns();
  return nullptr;
}

template <>
via::QualType via::IRBuilder::type_of<via::ast::ExprCast>(

    const ast::ExprCast* ast_expr_cast) {
  return type_of(ast_expr_cast->type);
}

template <>
via::QualType via::IRBuilder::type_of<via::ast::ExprTernary>(

    const ast::ExprTernary* ast_expr_ternary) {
  auto lhs = type_of(ast_expr_ternary->lhs),
       rhs = type_of(ast_expr_ternary->rhs);
  return lhs == rhs ? lhs : nullptr;
}

template <>
via::QualType via::IRBuilder::type_of<via::ast::TypeBuiltin>(

    const ast::TypeBuiltin* ast_type_builtin) {
  using enum TokenKind;
  using enum BuiltinKind;
  BuiltinKind kind;

  switch (ast_type_builtin->token->kind) {
    case KW_NONE:
      kind = NONE;
      break;
    case KW_BOOL:
      kind = BOOL;
      break;
    case KW_INT:
      kind = INT;
      break;
    case KW_FLOAT:
      kind = FLOAT;
      break;
    case KW_STRING:
      kind = STRING;
      break;
    default:
      VIA_PANIC("unmapped builtin type token");
  }

  return m_types.instance<BuiltinType>(kind);
}

via::QualType via::IRBuilder::type_of(const ast::Expr* expr) {
#define VISIT_EXPR(TYPE) \
  if VIA_TRY_COERCE (const TYPE, _INNER, expr) return type_of<TYPE>(_INNER);

  VISIT_EXPR(ast::ExprLiteral);
  VISIT_EXPR(ast::ExprSymbol);
  VISIT_EXPR(ast::ExprStaticAccess);
  VISIT_EXPR(ast::ExprDynAccess);
  VISIT_EXPR(ast::ExprUnary);
  VISIT_EXPR(ast::ExprBinary);
  VISIT_EXPR(ast::ExprCall);
  VISIT_EXPR(ast::ExprSubscript);
  VISIT_EXPR(ast::ExprCast);
  VISIT_EXPR(ast::ExprTernary);
  VISIT_EXPR(ast::ExprArray);
  VISIT_EXPR(ast::ExprTuple);
  VISIT_EXPR(ast::ExprLambda);
#undef VISIT_EXPR

  if VIA_TRY_COERCE (const ast::ExprGroup, expr_group, expr)
    return type_of(expr_group->expr);

  VIA_PANIC(std::format("type_of({})", VIA_TYPENAME(*expr)));
}

via::QualType via::IRBuilder::type_of(const ast::Type* type) {
#define VISIT_TYPE(TYPE) \
  if VIA_TRY_COERCE (const TYPE, _INNER, type) return type_of<TYPE>(_INNER);

  if ((type->quals & TypeQualifier::STRONG) &&
      !(type->quals & TypeQualifier::REFERENCE)) {
    m_diags.report<Level::ERROR>(type->loc,
                                 "Invalid usage of 'strong' qualifier",
                                 Note(Note::HINT,
                                      "'strong' must be used in conjunction "
                                      "with the '&' (REFERENCE) qualifier "
                                      "to denote strongly referenced type"));
    return {};
  }

  VISIT_TYPE(ast::TypeBuiltin);
  VISIT_TYPE(ast::TypeArray);
  VISIT_TYPE(ast::TypeMap);
  VISIT_TYPE(ast::TypeFunc);

  VIA_PANIC(std::format("type_of({})", VIA_TYPENAME(*type)));
#undef VISIT_TYPE
}

template <>
const via::ir::Expr* via::IRBuilder::lower_expr<via::ast::ExprLiteral>(

    const ast::ExprLiteral* ast_literal_expr) {
  auto const_value = ConstValue::from_token(*ast_literal_expr->tok);
  VIA_DEBUG_ASSERT(const_value != std::nullopt);

  auto* constant_expr = m_alloc.emplace<ir::Expr::Constant>();
  constant_expr->loc = ast_literal_expr->loc;
  constant_expr->value = *const_value;
  constant_expr->type = type_of(ast_literal_expr);
  return constant_expr;
}

template <>
const via::ir::Expr* via::IRBuilder::lower_expr<via::ast::ExprSymbol>(

    const ast::ExprSymbol* ast_symbol_expr) {
  if (poisoned(ast_symbol_expr->symbol->to_string())) return nullptr;

  auto& frame = m_stack.top();
  auto symbol = ast_symbol_expr->symbol->to_string();
  auto* ast_expr_symbol = m_alloc.emplace<ir::Expr::Symbol>();
  ast_expr_symbol->loc = ast_symbol_expr->loc;
  ast_expr_symbol->symbol = m_symbols.intern(symbol);
  ast_expr_symbol->type = type_of(ast_symbol_expr);

  if (!frame.get_local(ast_expr_symbol->symbol).has_value()) {
    poison(ast_expr_symbol->symbol);
    m_diags.report<Level::ERROR>(
        ast_expr_symbol->loc,
        std::format("use of undefined symbol '{}'", symbol),
        Note(Note::HINT,
             std::format("did you mistype '{}' or forget to declare it?",
                         symbol)));
  }
  return ast_expr_symbol;
}

template <>
const via::ir::Expr* via::IRBuilder::lower_expr<via::ast::ExprStaticAccess>(

    const ast::ExprStaticAccess* ast_stc_access_expr) {
  // Check for module thing
  if VIA_TRY_COERCE (const ast::ExprSymbol, root_symbol,
                     ast_stc_access_expr->root) {
    if (poisoned(root_symbol->symbol->to_string())) return nullptr;

    auto& manager = m_module.manager();
    auto high = intern(root_symbol->symbol->to_string());

    if (auto* module = manager.get_module_by_name(high)) {
      auto low = intern(ast_stc_access_expr->index->to_string());

      if (auto bind = module->lookup(low)) {
        auto* maccess = m_alloc.emplace<ir::Expr::ModuleAccess>();
        maccess->module = module;
        maccess->mod_id = high;
        maccess->key_id = low;
        maccess->bind = *bind;
        return maccess;
      }
    }
  }

  auto* access_expr = m_alloc.emplace<ir::Expr::Access>();
  access_expr->kind = ir::Expr::Access::Kind::STATIC;
  access_expr->root = lower(ast_stc_access_expr->root);
  access_expr->index = intern(*ast_stc_access_expr->index);
  access_expr->type = type_of(ast_stc_access_expr);
  access_expr->loc = ast_stc_access_expr->loc;
  return access_expr;
}

template <>
const via::ir::Expr* via::IRBuilder::lower_expr<via::ast::ExprDynAccess>(

    const ast::ExprDynAccess* ast_dyn_access_expr) {
  auto* access_expr = m_alloc.emplace<ir::Expr::Access>();
  access_expr->kind = ir::Expr::Access::Kind::DYNAMIC;
  access_expr->root = lower(ast_dyn_access_expr->root);
  access_expr->index = intern(*ast_dyn_access_expr->index);
  access_expr->type = type_of(ast_dyn_access_expr);
  access_expr->loc = ast_dyn_access_expr->loc;
  return access_expr;
}

template <>
const via::ir::Expr* via::IRBuilder::lower_expr<via::ast::ExprUnary>(

    const ast::ExprUnary* ast_expr_unary) {
  auto* unary_expr = m_alloc.emplace<ir::Expr::Unary>();
  unary_expr->op = to_unary_op(ast_expr_unary->op->kind);
  unary_expr->expr = lower(ast_expr_unary->expr);
  unary_expr->loc = ast_expr_unary->loc;

  auto op = to_unary_op(ast_expr_unary->op->kind);
  auto info = UNARY_OP_TABLE[static_cast<uint8_t>(op)];
  auto type = type_of(ast_expr_unary->expr);

  if (!info.is_valid(type)) {
    m_diags.report<Level::ERROR>(
        ast_expr_unary->loc, std::format("invalid unary operation '{}' ({}) on "
                                         "incompatible type '{}'",
                                         ast_expr_unary->op->to_string(),
                                         to_string(op), dump(type)));
  }

  unary_expr->type = info.get_result(m_types, type);
  return unary_expr;
}

template <>
const via::ir::Expr* via::IRBuilder::lower_expr<via::ast::ExprBinary>(

    const ast::ExprBinary* ast_expr_binary) {
  auto* binary_expr = m_alloc.emplace<ir::Expr::Binary>();
  binary_expr->op = to_binary_op(ast_expr_binary->op->kind);
  binary_expr->lhs = lower(ast_expr_binary->lhs);
  binary_expr->rhs = lower(ast_expr_binary->rhs);
  binary_expr->loc = ast_expr_binary->loc;

  auto op = to_binary_op(ast_expr_binary->op->kind);
  auto info = BINARY_OP_TABLE[static_cast<uint8_t>(op)];
  auto lhs = type_of(ast_expr_binary->lhs), rhs = type_of(ast_expr_binary->rhs);

  if (!info.is_valid(lhs, rhs)) {
    m_diags.report<Level::ERROR>(
        ast_expr_binary->loc,
        std::format("invalid binary operation '{}' ({}) on "
                    "incompatible types '{}' (LEFT) "
                    "'{}' (RIGHT)",
                    ast_expr_binary->op->to_string(), to_string(op),
                    lhs.to_string(), rhs.to_string()));
  }

  binary_expr->type = info.get_result(m_types, lhs, rhs);
  return binary_expr;
}

template <>
const via::ir::Expr* via::IRBuilder::lower_expr<via::ast::ExprGroup>(

    const ast::ExprGroup* ast_expr_group) {
  return lower(ast_expr_group->expr);
}

template <>
const via::ir::Expr* via::IRBuilder::lower_expr<via::ast::ExprCall>(

    const ast::ExprCall* ast_expr_call) {
  auto* call_expr = m_alloc.emplace<ir::Expr::Call>();
  call_expr->callee = lower(ast_expr_call->callee);
  call_expr->loc = ast_expr_call->loc;
  call_expr->args = [&]() {
    std::vector<const ir::Expr*> args;
    for (const auto& ast_arg : ast_expr_call->args)
      args.push_back(lower(ast_arg));
    return args;
  }();

  if (!call_expr->callee) return nullptr;

  auto callee = type_of(ast_expr_call->callee);

  if VIA_TRY_COERCE (const FunctionType, func, callee.unwrap()) {
    const size_t arg_count = ast_expr_call->args.size(),
                 parm_count = func->parameters().size();

    for (size_t arg_id = 0; via::QualType parm_type : func->parameters()) {
      if (arg_id >= arg_count) {
        m_diags.report<Level::ERROR>(
            {ast_expr_call->loc.end - 1, ast_expr_call->loc.end},
            std::format("in function call to '{}': "
                        "missing required argument for parameter #{}",
                        dump(ast_expr_call->callee), arg_id));
      } else {
        auto* arg = ast_expr_call->args.at(arg_id);
        auto arg_type = type_of(arg);
        if (arg_type != parm_type && arg_type && parm_type) {
          auto cast_result = arg_type.cast_result(parm_type);
          m_diags.report<Level::ERROR>(
              arg->loc,
              std::format(
                  "in function call to '{}': "
                  "argument #{} of type '{}' is incompatible with parameter "
                  "that expects type '{}'",
                  dump(ast_expr_call->callee), arg_id, dump(arg_type),
                  dump(parm_type)),
              cast_result != CastResult::INVALID
                  ? Note(Note::NOTE,
                         std::format(
                             "conversion from '{}' to '{}' possible with "
                             "explicit cast",
                             dump(arg_type), dump(parm_type)))
                  : Note{});
        }
      }
      arg_id++;
    }

    if (arg_count > parm_count) {
      auto first = ast_expr_call->args[parm_count];
      auto last = ast_expr_call->args.back();
      m_diags.report<Level::ERROR>(
          {first->loc.begin, last->loc.end},
          std::format("in function call to '{}': expected {} arguments, got {}",
                      dump(ast_expr_call->callee), parm_count, arg_count),
          {Note::SUGGESTION, "remove argument(s)"});
    }

    call_expr->type = func->returns();
  } else {
    m_diags.report<Level::ERROR>(
        ast_expr_call->loc,
        std::format("attempt to call non-function type '{}'", dump(callee)));
  }
  return call_expr;
}

template <>
const via::ir::Expr* via::IRBuilder::lower_expr<via::ast::ExprCast>(

    const ast::ExprCast* ast_expr_cast) {
  auto cast_type = type_of(ast_expr_cast->type);
  auto* cast_expr = m_alloc.emplace<ir::Expr::Cast>();
  cast_expr->expr = lower(ast_expr_cast->expr);
  cast_expr->cast = cast_type;
  cast_expr->type = cast_type;

  if (cast_expr->expr != nullptr && cast_expr->expr->type != nullptr) {
    auto expr_type = cast_expr->expr->type;
    if (expr_type == cast_type) {
      m_diags.report<Level::WARNING>(
          ast_expr_cast->expr->loc,
          std::format("redundant type cast: expression is already of type '{}'",
                      dump(cast_type)),
          {Note::SUGGESTION, "Remove cast"});
    }

    if (expr_type.cast_result(cast_type) == CastResult::INVALID) {
      m_diags.report<Level::ERROR>(
          ast_expr_cast->expr->loc,
          std::format("expression of type '{}' cannot be casted into type '{}'",
                      dump(expr_type), dump(cast_type)));
    }
  }
  return cast_expr;
}

const via::ir::Expr* via::IRBuilder::lower(const ast::Expr* expr) {
#define VISIT_EXPR(TYPE) \
  if VIA_TRY_COERCE (const TYPE, _INNER, expr) return lower_expr<TYPE>(_INNER);

  VISIT_EXPR(ast::ExprLiteral);
  VISIT_EXPR(ast::ExprSymbol);
  VISIT_EXPR(ast::ExprStaticAccess);
  VISIT_EXPR(ast::ExprDynAccess);
  VISIT_EXPR(ast::ExprUnary);
  VISIT_EXPR(ast::ExprBinary);
  VISIT_EXPR(ast::ExprCall);
  VISIT_EXPR(ast::ExprSubscript);
  VISIT_EXPR(ast::ExprCast);
  VISIT_EXPR(ast::ExprTernary);
  VISIT_EXPR(ast::ExprArray);
  VISIT_EXPR(ast::ExprTuple);
  VISIT_EXPR(ast::ExprLambda);
#undef VISIT_EXPR

  if VIA_TRY_COERCE (const ast::ExprGroup, expr_group, expr)
    return lower(expr_group->expr);

  VIA_PANIC(std::format("IRBuilder::lower({})", VIA_TYPENAME(*expr)));
}

template <>
const via::ir::Stmt* via::IRBuilder::lower_stat<via::ast::StatIf>(

    const ast::StatIf* ast_stat_if) {
  auto* merge_block = m_alloc.emplace<ir::Stmt::Block>();
  merge_block->id = m_block_id++;

  ir::Term::CondBranch* last = nullptr;

  for (size_t i = 0; const auto& branch : ast_stat_if->branches) {
    auto* then_block = m_alloc.emplace<ir::Stmt::Block>();
    then_block->id = m_block_id++;

    auto* then_term = m_alloc.emplace<ir::Term::Branch>();
    then_term->target = merge_block;
    then_block->term = then_term;

    auto* current_block = m_current_block;
    m_current_block = then_block;
    m_current_block->stats.push_back(({
      auto* save = m_alloc.emplace<ir::Stmt::Instruction>();
      save->instr = {OpCode::SAVE, 0, 0, 0};
      save;
    }));

    for (const auto& stat : branch.body->stats) {
      m_current_block->stats.push_back(lower(stat));
    }

    m_current_block->stats.push_back(({
      auto* restore = m_alloc.emplace<ir::Stmt::Instruction>();
      restore->instr = {OpCode::RESTORE, 0, 0, 0};
      restore;
    }));
    m_current_block = current_block;

    if (branch.cond != nullptr) {
      auto* cond_block = m_alloc.emplace<ir::Stmt::Block>();
      cond_block->id = m_block_id++;

      m_current_block->stats.push_back(cond_block);
      m_current_block->stats.push_back(then_block);

      auto* term = m_alloc.emplace<ir::Term::CondBranch>();
      term->cnd = lower(branch.cond);
      term->iftrue = then_block;

      if (i == ast_stat_if->branches.size() - 1) {
        term->iffalse = merge_block;
      }
      if (last != nullptr) {
        last->iffalse = cond_block;
      }

      last = term;
      cond_block->term = term;
    } else {
      last->iffalse = then_block;
      m_current_block->stats.push_back(then_block);
    }

    i++;
  }
  return merge_block;
}

template <>
const via::ir::Stmt* via::IRBuilder::lower_stat<via::ast::StatWhile>(

    const ast::StatWhile* ast_stat_while) {
  auto* merge_block = m_alloc.emplace<ir::Stmt::Block>();
  merge_block->id = m_block_id++;

  auto* cond_block = m_alloc.emplace<ir::Stmt::Block>();
  cond_block->id = m_block_id++;

  auto* body_block = m_alloc.emplace<ir::Stmt::Block>();
  body_block->id = m_block_id++;
  body_block->term = ({
    auto* body_term = m_alloc.emplace<ir::Term::Branch>();
    body_term->target = cond_block;
    body_term;
  });

  auto* current_block = m_current_block;
  m_current_block = body_block;

  m_current_block->stats.push_back(({
    auto* save = m_alloc.emplace<ir::Stmt::Instruction>();
    save->instr = {OpCode::SAVE, 0, 0, 0};
    save;
  }));

  for (const auto& stat : ast_stat_while->body->stats) {
    m_current_block->stats.push_back(lower(stat));
  }

  m_current_block->stats.push_back(({
    auto* restore = m_alloc.emplace<ir::Stmt::Instruction>();
    restore->instr = {OpCode::RESTORE, 0, 0, 0};
    restore;
  }));

  m_current_block = current_block;

  cond_block->term = ({
    auto* cond_term = m_alloc.emplace<ir::Term::CondBranch>();
    cond_term->cnd = lower(ast_stat_while->cond);
    cond_term->iftrue = body_block;
    cond_term->iffalse = merge_block;
    cond_term;
  });

  current_block->stats.push_back(cond_block);
  current_block->stats.push_back(body_block);
  return merge_block;
}

template <>
const via::ir::Stmt* via::IRBuilder::lower_stat<via::ast::StatVarDecl>(

    const ast::StatVarDecl* ast_stat_var_decl) {
  auto* decl_stat = m_alloc.emplace<ir::Stmt::VarDecl>();
  decl_stat->expr = lower(ast_stat_var_decl->rval);
  decl_stat->loc = ast_stat_var_decl->loc;

  if VIA_TRY_COERCE (const ast::ExprSymbol, lval, ast_stat_var_decl->lval) {
    auto rval_type = type_of(ast_stat_var_decl->rval);

    if (ast_stat_var_decl->type != nullptr) {
      decl_stat->type = type_of(ast_stat_var_decl->type);
      if (decl_stat->type != rval_type) {
        m_diags.report<Level::ERROR>(
            ast_stat_var_decl->rval->loc,
            std::format(
                "expression of type '{}' does not match declaration type '{}'",
                dump(rval_type), dump(decl_stat->type)),
            rval_type.cast_result(decl_stat->type) != CastResult::INVALID
                ? Note{Note::NOTE,
                       std::format("conversion from '{}' to '{}' "
                                   "possible with explicit cast",
                                   dump(rval_type), dump(decl_stat->type))}
                : Note{});
        goto fallback;
      }
    } else {
    fallback:
      decl_stat->type = rval_type;
    }

    decl_stat->symbol = intern(lval->symbol->to_string());
  } else {
    VIA_PANIC("bad lvalue");
  }

  m_stack.top().set_local(decl_stat->symbol, ast_stat_var_decl, decl_stat);
  return decl_stat;
}

template <>
const via::ir::Stmt* via::IRBuilder::lower_stat<via::ast::StatReturn>(

    const ast::StatReturn* ast_stat_return) {
  auto* term = m_alloc.emplace<ir::Term::Return>();
  term->implicit = false;
  term->loc = ast_stat_return->loc;
  term->val = ast_stat_return->expr ? lower(ast_stat_return->expr) : nullptr;
  term->type = ast_stat_return->expr
                   ? type_of(ast_stat_return->expr)
                   : m_types.instance<BuiltinType>(BuiltinKind::NONE);

  auto* block = end_block();
  block->term = term;
  return block;
}

template <>
const via::ir::Stmt* via::IRBuilder::lower_stat<via::ast::StatImport>(

    const ast::StatImport* ast_stat_import) {
  QualName qual_name;
  for (const Token* token : ast_stat_import->path) {
    qual_name.push_back(token->to_string());
  }
  auto name = qual_name.back();

  if (m_stack.size() > 1) {
    poison(name);
    m_diags.report<Level::ERROR>(ast_stat_import->loc,
                                 "import statements cannot be nested");
    return nullptr;
  }

  if (auto module = m_module.manager().get_module_by_name(name)) {
    poison(name);
    m_diags.report<Level::ERROR>(
        ast_stat_import->loc,
        std::format("module '{}' imported more than once", name));

    if (auto* import_decl = module->ast_decl()) {
      m_diags.report<Level::INFO>(import_decl->loc, "previously imported here");
    }
  }

  auto result = m_module.import(qual_name, ast_stat_import);
  if (!result.has_value()) {
    poison(name);
    m_diags.report<Level::ERROR>(ast_stat_import->loc, result.error());
  }
  return nullptr;
}

template <>
const via::ir::Stmt* via::IRBuilder::lower_stat<via::ast::StatFunctionDecl>(

    const ast::StatFunctionDecl* ast_stat_function_decl) {
  auto* decl_stat = m_alloc.emplace<ir::Stmt::FuncDecl>();
  decl_stat->kind = ImplKind::SOURCE;
  decl_stat->ident = intern(ast_stat_function_decl->name->to_string());
  decl_stat->ret = ast_stat_function_decl->ret
                       ? type_of(ast_stat_function_decl->ret)
                       : nullptr;

  // TODO: Get rid of this
  if (decl_stat->ret == nullptr) {
    poison(decl_stat->ident);
    m_diags.report<Level::ERROR>(
        ast_stat_function_decl->loc,
        "compiler infered return types are not implemented");
    return nullptr;
  }

  for (const auto& parm : ast_stat_function_decl->parms) {
    ir::Parameter new_parm;
    new_parm.symbol = intern(parm->symbol->to_string());
    new_parm.type = type_of(parm->type);
    decl_stat->parms.push_back(new_parm);
  }

  auto* block = m_alloc.emplace<ir::Stmt::Block>();
  block->id = m_block_id++;

  m_stack.push({});

  for (const auto& stat : ast_stat_function_decl->body->stats) {
    if VIA_TRY_COERCE (const ast::StatReturn, ret, stat) {
      auto* term = m_alloc.emplace<ir::Term::Return>();
      term->implicit = false;
      term->loc = ret->loc;
      term->val = ret->expr ? lower(ret->expr) : nullptr;
      term->type = ret->expr ? type_of(ret->expr)
                             : m_types.instance<BuiltinType>(BuiltinKind::NONE);
      block->term = term;
      break;
    }

    block->stats.push_back(lower(stat));
  }

  m_stack.pop();

  if (block->term == nullptr) {
    SourceLoc loc{ast_stat_function_decl->body->loc.end - 1,
                  ast_stat_function_decl->body->loc.end};

    auto* nil = m_alloc.emplace<ir::Expr::Constant>();
    nil->loc = loc;
    nil->type = m_types.instance<BuiltinType>(BuiltinKind::NONE);
    nil->value = ConstValue();

    auto* term = m_alloc.emplace<ir::Term::Return>();
    term->implicit = true;
    term->loc = loc;
    term->val = nil;
    term->type = m_types.instance<BuiltinType>(BuiltinKind::NONE);
    block->term = term;
  }

  QualType expected_ret_type = decl_stat->ret;

  for (const auto& term : get_control_paths(block)) {
    if VIA_TRY_COERCE (const ir::Term::Return, ret, term) {
      if (!ret->type) {
        // Already failed, no need to diagnose further
        continue;
      }

      if (!expected_ret_type) {
        expected_ret_type = ret->type;
      } else if (expected_ret_type != ret->type) {
        Note implicit_return_node =
            ret->implicit ? Note(Note::NOTE, std::format("implicit return here",
                                                         ret->type.to_string()))
                          : Note();

        if (decl_stat->ret) {
          poison(decl_stat->ident);
          m_diags.report<Level::ERROR>(
              ret->loc,
              std::format("function return type '{}' does not match type "
                          "'{}' returned by control path",
                          decl_stat->ret.to_string(), ret->type.to_string()),
              implicit_return_node);
        } else {
          poison(decl_stat->ident);
          m_diags.report<Level::ERROR>(
              ret->loc,
              "all code paths must return the same type "
              "in function with inferred return type",
              implicit_return_node);
        }
        break;
      }
    } else {
      poison(decl_stat->ident);
      m_diags.report<Level::ERROR>(
          term->loc, "all control paths must return from function");
      break;
    }
  }

  if (decl_stat->ret && expected_ret_type &&
      decl_stat->ret != expected_ret_type) {
    poison(decl_stat->ident);
    m_diags.report<Level::ERROR>(
        block->loc,
        std::format("Function return type '{}' does not match inferred "
                    "return type '{}' from all control paths",
                    dump(decl_stat->ret), dump(expected_ret_type)));
  }

  auto& frame = m_stack.top();
  frame.set_local(decl_stat->ident, ast_stat_function_decl, decl_stat,
                  IRLocal::Qual::CONST);

  decl_stat->body = block;
  decl_stat->loc = ast_stat_function_decl->loc;
  return decl_stat;
}

template <>
const via::ir::Stmt* via::IRBuilder::lower_stat<via::ast::StatExpr>(

    const ast::StatExpr* ast_stat_expr) {
  auto* expr = m_alloc.emplace<ir::Stmt::Expr>();
  expr->expr = lower(ast_stat_expr->expr);
  expr->loc = ast_stat_expr->loc;
  return expr;
}

via::ir::Stmt::Block* via::IRBuilder::end_block() {
  m_should_push_block = true;
  return m_current_block;
}

via::ir::Stmt::Block* via::IRBuilder::new_block(size_t id) {
  ir::Stmt::Block* block = m_current_block;
  m_should_push_block = false;
  m_current_block = m_alloc.emplace<ir::Stmt::Block>();
  m_current_block->id = id;
  return block;
}

std::string via::IRBuilder::dump(QualType type) {
  return ansi::format(type.unwrap() ? type.to_string() : "<type error>",
                      ansi::Foreground::MAGENTA, ansi::Background::NONE,
                      ansi::Style::BOLD);
}

std::string via::IRBuilder::dump(const ast::Expr* expr) {
  std::ostringstream oss;
  if (expr != nullptr) {
    for (const char chr : m_module.source().get_slice(expr->loc)) {
      if (chr == '\n') {
        oss << " ...";
        break;
      }
      oss << chr;
    }
  }

  return ansi::format(expr ? oss.str() : "<expression error>",
                      ansi::Foreground::YELLOW, ansi::Background::NONE,
                      ansi::Style::BOLD);
}

const via::ir::Stmt* via::IRBuilder::lower(const ast::Stat* stat) {
#define VISIT_STMT(TYPE) \
  if VIA_TRY_COERCE (const TYPE, _INNER, stat) return lower_stat<TYPE>(_INNER);

  VISIT_STMT(ast::StatVarDecl);
  VISIT_STMT(ast::StatScope);
  VISIT_STMT(ast::StatIf);
  VISIT_STMT(ast::StatFor);
  VISIT_STMT(ast::StatForEach);
  VISIT_STMT(ast::StatWhile);
  VISIT_STMT(ast::StatAssign);
  VISIT_STMT(ast::StatReturn);
  VISIT_STMT(ast::StatEnum);
  VISIT_STMT(ast::StatImport);
  VISIT_STMT(ast::StatFunctionDecl);
  VISIT_STMT(ast::StatStructDecl);
  VISIT_STMT(ast::StatTypeDecl);
  VISIT_STMT(ast::StatExpr);
#undef VISIT_STMT

  if VIA_ISA (const ast::StatEmpty, stat) return nullptr;
  VIA_PANIC(VIA_TYPENAME(*stat));
}

via::IRTree via::IRBuilder::build() {
  m_stack.push({});         // Push root stack frame
  new_block(m_block_id++);  // Push block

  IRTree tree;

  for (const auto& ast : m_ast) {
    if (const ir::Stmt* stat = lower(ast))
      m_current_block->stats.push_back(stat);
    if (m_should_push_block) {
      ir::Stmt::Block* block = new_block(m_block_id++);
      tree.push_back(block);
    }
  }

  // Push last block (it likely will not have a terminator)
  tree.push_back(end_block());
  return tree;
}
