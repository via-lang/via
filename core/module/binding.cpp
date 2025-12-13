/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include "binding.hpp"

#include <sstream>

#include "ast/tree.hpp"
#include "manager.hpp"
#include "sema/const_value.hpp"
#include "sema/type.hpp"
#include "support/ansi.hpp"
#include "symbol.hpp"

std::string via::BindingParameter::to_string() const {
  return std::format("__parm_{0}: {0}", type.to_string());
}

VIA_NOINLINE via::Binding* via::Binding::from(ModuleManager& manager,
                                              const ir::Stmt* node) {
  if TRY_COERCE (const ir::StmtFuncDecl, decl, node) {
    auto* function =
        manager.allocator().emplace<FunctionBinding>(manager, decl->symbol);
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
          m_params, [](const auto& parm) { return parm.to_string(); }, "(",
          ")"),
      m_return.to_string());
}

std::string via::to_string(
    const SymbolTable& table,
    const std::unordered_map<SymbolId, const Binding*>& map) noexcept {
  std::ostringstream oss;
  oss << ansi::format("[disassembly of bind table]:\n",
                      ansi::Foreground::YELLOW, ansi::Background::NONE,
                      ansi::Style::UNDERLINE);

  oss << ansi::format(
      "  id    kind        signature           \n"
      "  ----  ----------  --------------------\n",
      ansi::Foreground::NONE, ansi::Background::NONE, ansi::Style::FAINT);

  for (size_t i = 0; const auto& it : map) {
    oss << "  "
        << ansi::format(std::format("{:0>4}  ", i++), ansi::Foreground::NONE,
                        ansi::Background::NONE, ansi::Style::FAINT);
    if TRY_COERCE (const FunctionBinding, function_def, it.second) {
      oss << "function  ";
      oss << "  " << function_def->signature(table) << "\n";
    } else {
      oss << "unknown   ";
      oss << "address: " << (void*)it.second << "\n";
    }
  }
  oss << "\n";
  return oss.str();
}
