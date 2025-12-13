/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#pragma once

#include <vector>
#include <via/config.hpp>

#include "manager.hpp"
#include "module.hpp"
#include "module/binding.hpp"
#include "sema/type.hpp"

namespace via {

class ModuleBuilder final {
 public:
  explicit ModuleBuilder(ModuleManager& manager)
      : m_manager(manager), m_types(manager.type_context()) {}

 public:
  NativeModuleInfo* build() &;

  FunctionBinding& function(std::string symbol) &;

  // clang-format off
  QualType nil_t() & { return m_types.instance<BuiltinType>(BuiltinKind::NIL); }
  QualType bool_t() & { return m_types.instance<BuiltinType>(BuiltinKind::BOOL); }
  QualType int_t() & { return m_types.instance<BuiltinType>(BuiltinKind::INT); }
  QualType float_t() & { return m_types.instance<BuiltinType>(BuiltinKind::FLOAT); }
  QualType string_t() & { return m_types.instance<BuiltinType>(BuiltinKind::STRING); }
  // clang-format on

 private:
  ModuleManager& m_manager;
  TypeContext& m_types;
  std::vector<const Binding*> m_defs;
};

}  // namespace via
