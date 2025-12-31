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
#include <module/manager.hpp>
#include <module/module.hpp>
#include <module/symbol.hpp>
#include <string>
#include <unordered_set>

#include "ir-local.hpp"
#include "ir-tree.hpp"
#include "syntax-tree.hpp"
#include "type.hpp"

namespace via {

class IRBuilder final {
 public:
  friend ::via::Module;

 public:
  explicit IRBuilder(Module& module, const SyntaxTree& ast, Diagnostics& diags)
    : m_module(module),
      m_ast(ast),
      m_alloc(module.allocator()),
      m_diags(diags),
      m_types(module.manager().type_context()),
      m_symbols(module.manager().symbol_table()) {}

 public:
  [[nodiscard]] IRTree build();

 protected:
  // clang-format off
  void poison(SymbolId symbol)  { m_poisoned_ids.insert(symbol); }
  void poison(QualName name)  { m_poisoned_ids.insert(intern(name)); }
  void poison(std::string symbol)  { m_poisoned_ids.insert(intern(symbol)); }
  bool poisoned(SymbolId symbol)  { return m_poisoned_ids.contains(symbol); }
  bool poisoned(QualName name)  { return m_poisoned_ids.contains(intern(name)); }
  bool poisoned(std::string symbol)  { return m_poisoned_ids.contains(intern(symbol)); }
  // clang-format on

 private:
  QualType type_of(const ast::Expr* expr);
  QualType type_of(const ast::Type* type);
  const ir::Expr* lower(const ast::Expr* expr);
  const ir::Stmt* lower(const ast::Stat* stat);
  ir::Stmt::Block* end_block();
  ir::Stmt::Block* new_block(size_t id);
  std::string dump(QualType type);
  std::string dump(const ast::Expr* expr);

  SymbolId intern(std::string symbol) { return m_symbols.intern(symbol); }
  SymbolId intern(const QualName& name) { return m_symbols.intern(name); }
  SymbolId intern(const via::Token& token) {
    return m_symbols.intern(token.to_string());
  }

  template <derived_from<ast::Expr, ast::Type> Type>
    requires(!std::is_same_v<Type, ast::Type>)
  QualType type_of(const Type*) {
    VIA_PANIC(VIA_TYPENAME(Type));
  }

  template <derived_from<ast::Expr> Expr>
    requires(!std::is_same_v<Type, ast::Expr>)
  const ir::Expr* lower_expr(const Expr*) {
    VIA_PANIC(VIA_TYPENAME(Expr));
  }

  template <derived_from<ast::Stat> Stat>
    requires(!std::is_same_v<Type, ast::Type>)
  const ir::Stmt* lower_stat(const Stat*) {
    VIA_PANIC(VIA_TYPENAME(Stat));
  }

 private:
  Module& m_module;
  const SyntaxTree& m_ast;
  ScopedAllocator& m_alloc;
  Diagnostics& m_diags;
  StackState<IRLocal> m_stack;
  TypeContext& m_types;
  SymbolTable& m_symbols;
  bool m_should_push_block;
  uint32_t m_block_id = 0;
  ir::Stmt::Block* m_current_block;
  std::unordered_set<SymbolId> m_poisoned_ids;
};

}  // namespace via
