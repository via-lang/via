/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include "ir-tree.hpp"

#include <ansi.hpp>
#include <module/symbol.hpp>

#include "token.hpp"

#define SYMBOL_ERROR "<symbol error>"
#define EXPR_ERROR "<expression error>"
#define STMT_ERROR "<statement error>"
#define TERM_ERROR "<terminator error>"

#define INDENT(DEPTH) (std::string((DEPTH) * 2, ' '))
#define SYMBOL(ID) (table->lookup((ID)).value_or(SYMBOL_ERROR))
#define TOSTRING(OBJ, DEPTH, ALT) \
  ((OBJ) ? (OBJ)->to_string(table, (DEPTH)) : INDENT((DEPTH)) + (ALT))

using enum via::TokenKind;

via::UnaryOp via::to_unary_op(via::TokenKind kind) {
  switch (kind) {
    case OP_MINUS:
      return UnaryOp::NOT;
    case OP_BANG:
      return UnaryOp::NOT;
    case OP_TILDE:
      return UnaryOp::BNOT;
    default:
      break;
  }
  VIA_PANIC("unmapped UnaryOp TokenKind");
}

via::BinaryOp via::to_binary_op(via::TokenKind kind) {
  switch (kind) {
    case OP_PLUS:
      return BinaryOp::ADD;
    case OP_MINUS:
      return BinaryOp::SUB;
    case OP_STAR:
      return BinaryOp::MUL;
    case OP_SLASH:
      return BinaryOp::DIV;
    case OP_STAR_STAR:
      return BinaryOp::POW;
    case OP_PERCENT:
      return BinaryOp::MOD;
    case OP_AMP_AMP:
      return BinaryOp::AND;
    case OP_PIPE_PIPE:
      return BinaryOp::OR;
    case OP_AMP:
      return BinaryOp::BAND;
    case OP_PIPE:
      return BinaryOp::BOR;
    case OP_CARET:
      return BinaryOp::BXOR;
    case OP_SHL:
      return BinaryOp::BSHL;
    case OP_SHR:
      return BinaryOp::BSHR;
    default:
      break;
  }
  VIA_PANIC("unmapped BinaryOp TokenKind");
}

std::string via::ir::Term::Return::to_string(const SymbolTable* table,
                                             size_t depth) const {
  return INDENT(depth) + std::format("RETURN {} {}",
                                     TOSTRING(val, 0, EXPR_ERROR),
                                     implicit ? "(implicit)" : "");
}

std::string via::ir::Term::Continue::to_string(const SymbolTable* table,
                                               size_t depth) const {
  return INDENT(depth) + "CONTINUE";
}

std::string via::ir::Term::Break::to_string(const SymbolTable* table,
                                            size_t depth) const {
  return INDENT(depth) + "BREAK";
}

std::string via::ir::Term::Branch::to_string(const SymbolTable* table,
                                             size_t depth) const {
  return INDENT(depth) + std::format("BRANCH #{}", target->id);
}

std::string via::ir::Term::CondBranch::to_string(const SymbolTable* table,
                                                 size_t depth) const {
  return INDENT(depth) + std::format("BRANCH {} ? #{} : #{}",
                                     TOSTRING(cnd, 0, EXPR_ERROR), iftrue->id,
                                     iffalse->id);
}

std::string via::ir::Expr::Constant::to_string(const SymbolTable* table,
                                               size_t depth) const {
  return INDENT(depth) + value.to_string();
}

std::string via::ir::Expr::Symbol::to_string(const SymbolTable* table,
                                             size_t depth) const {
  return INDENT(depth) + std::string(SYMBOL(symbol));
}

std::string via::ir::Expr::Access::to_string(const SymbolTable* table,
                                             size_t depth) const {
  return INDENT(depth) +
         std::format("{}{}{}", TOSTRING(root, 0, EXPR_ERROR),
                     kind == Kind::DYNAMIC ? "." : "::", SYMBOL(index));
}

std::string via::ir::Expr::ModuleAccess::to_string(const SymbolTable* table,
                                                   size_t depth) const {
  return INDENT(depth) +
         std::format("MODULE({})::{}", SYMBOL(mod_id), SYMBOL(key_id));
}

std::string via::ir::Expr::Unary::to_string(const SymbolTable* table,
                                            size_t depth) const {
  return INDENT(depth) + std::format("({} {})", via::to_string(op),
                                     TOSTRING(expr, 0, EXPR_ERROR));
}

std::string via::ir::Expr::Binary::to_string(const SymbolTable* table,
                                             size_t depth) const {
  return INDENT(depth) + std::format("({} {} {})", TOSTRING(lhs, 0, EXPR_ERROR),
                                     via::to_string(op),
                                     TOSTRING(rhs, 0, EXPR_ERROR));
}

std::string via::ir::Expr::Call::to_string(const SymbolTable* table,
                                           size_t depth) const {
  return INDENT(depth) + std::format("CALL {}{}",
                                     TOSTRING(callee, 0, EXPR_ERROR),
                                     via::to_string(
                                         args,
                                         [&](const auto& expr) {
                                           return TOSTRING(expr, 0, EXPR_ERROR);
                                         },
                                         "(", ")"));
}

std::string via::ir::Expr::Subscript::to_string(const SymbolTable* table,
                                                size_t depth) const {
  return INDENT(depth) + std::format("{}[{}]", TOSTRING(expr, 0, EXPR_ERROR),
                                     TOSTRING(idx, 0, EXPR_ERROR));
}

std::string via::ir::Expr::Cast::to_string(const SymbolTable* table,
                                           size_t depth) const {
  return INDENT(depth) + std::format("{} AS {}", TOSTRING(expr, 0, EXPR_ERROR),
                                     cast.to_string());
}

std::string via::ir::Expr::Ternary::to_string(const SymbolTable* table,
                                              size_t depth) const {
  return INDENT(depth) + std::format("({} ? {} : {})",
                                     TOSTRING(cnd, 0, EXPR_ERROR),
                                     TOSTRING(iftrue, 0, EXPR_ERROR),
                                     TOSTRING(iffalse, 0, EXPR_ERROR));
}

std::string via::ir::Expr::Array::to_string(const SymbolTable* table,
                                            size_t depth) const {
  return INDENT(depth) + via::to_string(exprs, [&](const auto& expr) {
           return TOSTRING(expr, 0, EXPR_ERROR);
         });
}

std::string via::ir::Expr::Tuple::to_string(const SymbolTable* table,
                                            size_t depth) const {
  return INDENT(depth) + "<tuple>";
}

std::string via::ir::Expr::Lambda::to_string(const SymbolTable* table,
                                             size_t depth) const {
  return INDENT(depth) + "<lambda>";
}

std::string via::ir::Stmt::VarDecl::to_string(const SymbolTable* table,
                                              size_t depth) const {
  return INDENT(depth) + std::format("LOCAL {}: {} = {}", SYMBOL(symbol),
                                     type.to_string(),
                                     TOSTRING(expr, 0, EXPR_ERROR));
}

std::string via::ir::Stmt::FuncDecl::to_string(const SymbolTable* table,
                                               size_t depth) const {
  std::ostringstream oss;
  oss << INDENT(depth)
      << std::format(
             "FUNCTION {} {} -> {}:\n", SYMBOL(ident),
             via::to_string(
                 parms, [&](const auto& parm) { return parm.to_string(table); },
                 "(", ")"),
             ret.to_string());

  for (const Stmt* stat : body->stats)
    oss << TOSTRING(stat, depth + 1, STMT_ERROR) << "\n";
  oss << TOSTRING(body->term, depth + 1, TERM_ERROR);
  return oss.str();
}

std::string via::ir::Stmt::Instruction::to_string(const SymbolTable* table,
                                                  size_t depth) const {
  return INDENT(depth) + instr.to_string(false);
}

std::string via::ir::Stmt::Block::to_string(const SymbolTable* table,
                                            size_t depth) const {
  std::ostringstream oss;
  oss << INDENT(depth) << "BLOCK #" << id << ":\n";

  for (const Stmt* stat : stats)
    oss << TOSTRING(stat, depth + 1, STMT_ERROR) << "\n";
  oss << TOSTRING(term, depth + 1, TERM_ERROR);
  return oss.str();
}

std::string via::ir::Stmt::Expr::to_string(const SymbolTable* table,
                                           size_t depth) const {
  return TOSTRING(expr, depth, EXPR_ERROR);
}

std::string via::to_string(const SymbolTable& table, const IRTree& ir_tree) {
  std::ostringstream oss;
  oss << ansi::format("[disassembly of program IR]:\n",
                      ansi::Foreground::YELLOW, ansi::Background::NONE,
                      ansi::Style::UNDERLINE);

  for (const auto& node : ir_tree)
    oss << (node ? node->to_string(&table, 1) : INDENT(1) + EXPR_ERROR) << "\n";
  return oss.str();
}
