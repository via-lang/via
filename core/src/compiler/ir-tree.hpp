/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#pragma once

#include <config.hpp>
#include <cstddef>
#include <module/symbol.hpp>
#include <optional>
#include <utility.hpp>
#include <vm/instruction.hpp>

#include "type.hpp"
#include "value.hpp"

namespace via {

#define FOR_EACH_IMPL_KIND(X) \
  X(SOURCE)                   \
  X(BINARY)                   \
  X(NATIVE)

enum class ImplKind { FOR_EACH_IMPL_KIND(VIA_DEFINE_ENUM) };

VIA_DEFINE_TO_STRING(ImplKind, FOR_EACH_IMPL_KIND(VIA_DEFINE_CASE_TO_STRING));

class Module;
class Binding;

#define FOR_EACH_UNARY_OP(X) \
  X(NEG)                     \
  X(NOT)                     \
  X(BNOT)

#define FOR_EACH_BINARY_OP(X) \
  X(ADD)                      \
  X(SUB)                      \
  X(MUL)                      \
  X(DIV)                      \
  X(POW)                      \
  X(MOD)                      \
  X(AND)                      \
  X(OR)                       \
  X(BAND)                     \
  X(BOR)                      \
  X(BXOR)                     \
  X(BSHL)                     \
  X(BSHR)

enum class UnaryOp { FOR_EACH_UNARY_OP(VIA_DEFINE_ENUM) };

enum class BinaryOp { FOR_EACH_BINARY_OP(VIA_DEFINE_ENUM) };

VIA_DEFINE_TO_STRING(UnaryOp, FOR_EACH_UNARY_OP(VIA_DEFINE_CASE_TO_STRING));
VIA_DEFINE_TO_STRING(BinaryOp, FOR_EACH_BINARY_OP(VIA_DEFINE_CASE_TO_STRING));

UnaryOp to_unary_op(TokenKind kind);
BinaryOp to_binary_op(TokenKind kind);

#define NODE_FIELDS(BASE)                                                     \
  [[nodiscard]] std::string to_string(const SymbolTable* table, size_t depth) \
      const override;

namespace ir {

struct Node {
  SourceLoc loc;

  [[nodiscard]] virtual std::string to_string(const SymbolTable* table,
                                              size_t depth = 0) const = 0;
};

struct Parameter final : Node {
  SymbolId symbol;
  QualType type;

  [[nodiscard]] virtual std::string to_string(const SymbolTable* table,
                                              size_t depth = 0) const {
    return std::format("{}: {}", table->lookup(symbol).value_or("<symbol>"),
                       type.to_string());
  }
};

struct Expr : Node {
  struct Constant;
  struct Symbol;
  struct Access;
  struct ModuleAccess;
  struct Unary;
  struct Binary;
  struct Call;
  struct Subscript;
  struct Cast;
  struct Ternary;
  struct Array;
  struct Tuple;
  struct Lambda;

  QualType type;
};

struct Expr::Constant final : Expr {
  NODE_FIELDS(Expr)
  ConstValue value;
};

struct Expr::Symbol final : Expr {
  NODE_FIELDS(Expr)
  SymbolId symbol;
};

struct Expr::Access final : Expr {
  NODE_FIELDS(Expr)

  enum class Kind {
    STATIC,
    DYNAMIC,
  } kind;

  const Expr* root;
  SymbolId index;
};

struct Expr::ModuleAccess final : Expr {
  NODE_FIELDS(Expr)
  Module* module;
  SymbolId mod_id, key_id;
  const Binding* bind;
};

struct Expr::Unary final : Expr {
  NODE_FIELDS(Expr)
  UnaryOp op;
  const Expr* expr;
};

struct Expr::Binary final : Expr {
  NODE_FIELDS(Expr)
  BinaryOp op;
  const Expr *lhs, *rhs;
};

struct Expr::Call final : Expr {
  NODE_FIELDS(Expr)
  const Expr* callee;
  std::vector<const Expr*> args;
};

struct Expr::Subscript final : Expr {
  NODE_FIELDS(Expr)
  const Expr *expr, *idx;
};

struct Expr::Cast final : Expr {
  NODE_FIELDS(Expr)
  const Expr* expr;
  QualType cast;
};

struct Expr::Ternary final : Expr {
  NODE_FIELDS(Expr)
  const Expr *cnd, *iftrue, *iffalse;
};

struct Expr::Array final : Expr {
  NODE_FIELDS(Expr)
  std::vector<const Expr*> exprs;
};

struct Expr::Tuple final : Expr {
  NODE_FIELDS(Expr)
  std::vector<const Expr*> init;
};

struct Function;
struct Expr::Lambda final : Expr {
  NODE_FIELDS(Expr)
};

struct Term;
struct Stmt : Node {
  struct Expr;
  struct Block;
  struct VarDecl;
  struct FuncDecl;
  struct Instruction;

  [[nodiscard]] virtual std::optional<SymbolId> symbol() const {
    return std::nullopt;
  }
};

struct Stmt::Expr final : Stmt {
  NODE_FIELDS()
  const ir::Expr* expr;
};

struct Stmt::Block final : Stmt {
  NODE_FIELDS()
  uint32_t id;
  std::vector<const Stmt*> stats;
  const Term* term;
};

struct Stmt::VarDecl final : Stmt {
  NODE_FIELDS()
  SymbolId symbol;
  const ir::Expr* expr;
  QualType type;
};

struct Stmt::FuncDecl final : Stmt {
  NODE_FIELDS()

  ImplKind kind;
  SymbolId ident;
  QualType ret;
  std::vector<Parameter> parms;
  const Block* body;

  std::optional<SymbolId> symbol() const override { return ident; }
};

struct Stmt::Instruction final : Stmt {
  NODE_FIELDS()
  via::Instruction instr;
};

struct Term : Node {
  struct Return;
  struct Continue;
  struct Break;
  struct Branch;
  struct CondBranch;
};

struct Term::Return final : Term {
  NODE_FIELDS(Term)
  bool implicit;
  const Expr* val;
  QualType type;
};

struct Term::Continue final : Term {
  NODE_FIELDS(Term)
};

struct Term::Break final : Term {
  NODE_FIELDS(Term)
};

struct Term::Branch final : Term {
  NODE_FIELDS(Term)
  Stmt::Block* target;
};

struct Term::CondBranch final : Term {
  NODE_FIELDS(Term)
  const Expr* cnd;
  Stmt::Block *iftrue, *iffalse;
};

}  // namespace ir

using IRTree = std::vector<const ir::Stmt*>;

std::string to_string(const SymbolTable& table, const IRTree& ir_tree);

}  // namespace via
