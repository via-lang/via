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

#include "symbol.hpp"

namespace via {

union ImplStorage {
  const ir::StatFuncDecl* source;
  NativeCallback native;
};

class Module;
class ValueRef;
class Binding;

struct SymbolInfo {
  const Binding* symbol;
  const Module* module;
};

class Binding {
 public:
  static Binding* from(ModuleManager& manager, const ir::Stat* node);

  virtual std::optional<SymbolId> identity() const = 0;
  virtual std::string signature(const SymbolTable&) const {
    return "<identity error>";
  }
};

class FunctionBinding final : public Binding {
 public:
  friend class Binding;
  friend class IRBuilder;
  friend class VirtualMachine;

  struct Parameter {
    QualType type;
    ConstValue value{};
  };

 public:
  explicit FunctionBinding(ModuleManager& manager, SymbolId symbol)
      : m_manager(manager), m_symbol(symbol) {}

 public:
  FunctionBinding& returns(QualType type) &;
  FunctionBinding& parameter(QualType type, ConstValue value = {}) &;
  FunctionBinding& implement(NativeCallback impl) &;

  std::string signature(const SymbolTable& table) const override;
  std::optional<SymbolId> identity() const override { return m_symbol; }

 protected:
  ModuleManager& m_manager;
  ImplKind m_kind;
  ImplStorage m_impl;
  SymbolId m_symbol;
  QualType m_return;
  std::vector<Parameter> m_params;
};

std::string to_string(
    const SymbolTable& table,
    const std::unordered_map<SymbolId, const Binding*>& map) noexcept;

}  // namespace via
