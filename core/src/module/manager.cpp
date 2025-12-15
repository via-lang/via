/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include "manager.hpp"

bool via::ModuleManager::has_module(std::filesystem::path name) {
  return m_modules.find(name) != m_modules.end();
}

via::Module* via::ModuleManager::get_module_by_name(std::string name) {
  for (const auto& [_, module] : m_modules)
    if (module->m_name == name) return module;
  return nullptr;
}

via::Module* via::ModuleManager::get_module_by_name(SymbolId name) {
  if (auto symbol = m_symbol_table.lookup(name))
    if (auto module = get_module_by_name(std::string(*symbol))) return module;
  return nullptr;
}

bool via::ModuleManager::is_current_import(const std::string& name) const {
  return std::find(m_imports.begin(), m_imports.end(), name) != m_imports.end();
}

void via::ModuleManager::pop_import() {
  if (!m_imports.empty()) {
    m_imports.pop_back();
  }
}
