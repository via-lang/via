/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#pragma once

#include <string>
#include <via/config.hpp>

#include "ir/tree.hpp"
#include "sema/const_value.hpp"
#include "sema/types.hpp"
#include "symbol.hpp"
#include "vm/closure.hpp"

namespace via {

union ImplStorage {
  const ir::StmtFuncDecl* source;
  NativeCallback native;
};

class Module;
class ValueRef;
struct CallInfo;
class Binding;

struct SymbolInfo {
  const Binding* symbol;
  const Module* module;
};

struct BindingParameter {
  QualType type;
  ConstValue value{};

  std::string to_string() const;
};

using BindingTable = const Binding*[];

class Binding {
 public:
  static Binding* from(ModuleManager& manager, const ir::Stmt* node);

  virtual std::string signature(const SymbolTable&) const {
    return "<identity error>";
  }
  virtual std::optional<SymbolId> identity() const = 0;
};

class FunctionBinding final : public Binding {
 public:
  friend class Binding;
  friend class IRBuilder;
  friend class VirtualMachine;

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
  std::vector<BindingParameter> m_params;
};

std::string to_string(
    const SymbolTable& table,
    const std::unordered_map<SymbolId, const Binding*>& map) noexcept;

}  // namespace via
