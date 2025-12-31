/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#pragma once

#include <compiler/ir-tree.hpp>
#include <compiler/type.hpp>
#include <compiler/value.hpp>
#include <config.hpp>
#include <string>
#include <vm/closure.hpp>

#include "module/manager.hpp"
#include "symbol.hpp"

namespace via {

union ImplStorage {
  const ir::Stmt::FuncDecl* source;
  NativeCallback native;
};

struct SymbolInfo {
  const Binding* symbol;
  const Module* module;
};

class Module;
class ModuleManager;
class ValueRef;
class Binding {
 public:
  [[nodiscard]] static Binding* from(ModuleManager& manager,
                                     const ir::Stmt* node);

  [[nodiscard]] virtual std::optional<SymbolId> identity() const = 0;
  [[nodiscard]] virtual std::string signature(const SymbolTable&) const {
    return "<identity error>";
  }
};

class IRBuilder;
class FunctionBinding final : public Binding {
 public:
  friend ::via::Binding;
  friend ::via::IRBuilder;
  friend ::via::VirtualMachine;

  struct Parameter {
    QualType type;
    ConstValue value{};
  };

 public:
  explicit FunctionBinding(ModuleManager& manager, SymbolId symbol)
    : m_manager(manager), m_symbol(symbol) {}

 public:
  [[nodiscard]] FunctionBinding& returns(QualType type) &;
  [[nodiscard]] FunctionBinding& parameter(QualType type,
                                           ConstValue value = {}) &;
  [[nodiscard]] FunctionBinding& implement(NativeCallback impl) &;

  [[nodiscard]] virtual std::string signature(const SymbolTable& table) const;
  [[nodiscard]] virtual std::optional<SymbolId> identity() const {
    return m_symbol;
  }

 protected:
  ModuleManager& m_manager;
  ImplKind m_kind;
  ImplStorage m_impl;
  SymbolId m_symbol;
  QualType m_return;
  std::vector<Parameter> m_params;
};

}  // namespace via
