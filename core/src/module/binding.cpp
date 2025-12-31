/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include "binding.hpp"

#include <compiler/ir-tree.hpp>
#include <compiler/type.hpp>
#include <compiler/value.hpp>
#include <sstream>

#include "ansi.hpp"
#include "manager.hpp"
#include "symbol.hpp"

VIA_NOINLINE via::Binding* via::Binding::from(ModuleManager& manager,
                                              const ir::Stat* node) {
  if VIA_TRY_COERCE (const ir::StatFuncDecl, decl, node) {
    auto* function =
        manager.allocator().emplace<FunctionBinding>(manager, decl->ident);
    function->m_kind = ImplKind::SOURCE;
    function->m_impl.source = decl;
    function->m_return = decl->ret;

    for (const auto& parm : decl->parms)
      function->m_params.push_back({parm.type});
    return function;
  }
  return nullptr;
}

via::FunctionBinding& via::FunctionBinding::returns(QualType type) & {
  m_return = type;
  return *this;
}

via::FunctionBinding& via::FunctionBinding::parameter(QualType type,
                                                      ConstValue value) & {
  m_params.push_back({.type = type, .value = value});
  return *this;
}

via::FunctionBinding& via::FunctionBinding::implement(NativeCallback impl) & {
  m_impl.native = impl;
  return *this;
}

std::string via::FunctionBinding::signature(const SymbolTable& table) const {
  return std::format(
      "fn {} {} -> {}", table.lookup(m_symbol).value_or("<symbol error>"),
      via::to_string(
          m_params, [](const auto& parm) { return parm.type.to_string(); }, "(",
          ")"),
      m_return.to_string());
}
