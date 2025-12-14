/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#pragma once

#include <mimalloc.h>

#include <via/config.hpp>

#include "ast/tree.hpp"
#include "diagnostics.hpp"
#include "lexer/lexer.hpp"
#include "lexer/source_buffer.hpp"
#include "lexer/token.hpp"
#include "support/memory.hpp"

namespace via {

class Parser final {
 public:
  Parser(const SourceBuffer& source, const TokenTree& ttree, Diagnostics& diags)
      : m_diags(diags), m_source(source), m_cursor(ttree.data()) {}

 public:
  ScopedAllocator& allocator() { return m_alloc; }
  SyntaxTree parse();

 private:
  bool match(TokenKind kind, int ahead = 0);
  bool optional(TokenKind kind);

  const Token* peek(int ahead = 0);
  const Token* advance();
  const Token* expect(TokenKind kind, const char* task);

  // Special
  const ast::Path* parse_static_path();
  const ast::Expr* parse_lvalue();
  const ast::Parameter* parse_parameter();
  const ast::Scope* parse_scope();

  // Expression
  const ast::ExprLiteral* parse_expr_literal();
  const ast::ExprSymbol* parse_expr_symbol();
  const ast::Expr* parse_expr_group_or_tuple();
  const ast::ExprDynAccess* parse_expr_dyn_access(const ast::Expr* expr);
  const ast::ExprStaticAccess* parse_expr_st_access(const ast::Expr* expr);
  const ast::ExprUnary* parse_expr_unary(const ast::Expr* expr);
  const ast::ExprCall* parse_expr_call(const ast::Expr* expr);
  const ast::ExprSubscript* parse_expr_subscript(const ast::Expr* expr);
  const ast::ExprCast* parse_expr_cast(const ast::Expr* expr);
  const ast::ExprTernary* parse_expr_ternary(const ast::Expr* expr);
  const ast::ExprArray* parse_expr_array();
  const ast::ExprLambda* parse_expr_lambda();
  const ast::Expr* parse_expr_primary();
  const ast::Expr* parse_expr_affix();
  const ast::Expr* parse_expr(int min_prec = 0);

  // Types
  const ast::TypeBuiltin* parse_type_builtin();
  const ast::TypeArray* parse_type_array();
  const ast::TypeMap* parse_type_map();
  const ast::TypeFunc* parse_type_function();
  const ast::Type* parse_type_primary();
  const ast::Type* parse_type();

  // Statement
  const ast::StatVarDecl* parse_stat_var_decl(bool semicolon);
  const ast::StatFor* parse_stat_for();
  const ast::StatForEach* parse_stat_for_each();
  const ast::StatIf* parse_stat_if();
  const ast::StatWhile* parse_stat_while();
  const ast::StatAssign* parse_stat_assign(const ast::Expr* expr);
  const ast::StatReturn* parse_stat_return();
  const ast::StatEnum* parse_stat_enum();
  const ast::StatImport* parse_stat_import();
  const ast::StatFunctionDecl* parse_stat_func_decl();
  const ast::StatStructDecl* parse_stat_struct_decl();
  const ast::StatTypeDecl* parse_stat_type_decl();
  const ast::Stat* parse_stat();

 private:
  Diagnostics& m_diags;
  const SourceBuffer& m_source;
  const Token* const* m_cursor;
  ScopedAllocator m_alloc;
};

}  // namespace via
