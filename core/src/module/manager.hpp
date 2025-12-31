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
#include <filesystem>
#include <functional>

#include "module.hpp"
#include "symbol.hpp"

namespace via {

class ModuleManager {
 public:
  friend ::via::Module;

 public:
  [[nodiscard]] auto& allocator() { return m_alloc; }
  [[nodiscard]] auto& type_context() { return m_type_ctx; }
  [[nodiscard]] auto& symbol_table() { return m_symbol_table; }
  [[nodiscard]] auto import_paths() const { return m_import_paths; }

  void push_import_path(std::filesystem::path path) {
    m_import_paths.push_back(path);
  }

  void push_module(Module* module) { m_modules[module->m_path] = module; }

  template <ModuleQuery Q>
  [[nodiscard]] ModuleQueryResult query(const ModuleQueryParamT<Q>& param);

 protected:
  bool is_current_import(const std::string& name) const;

  void push_import(const std::string& name) { m_imports.push_back(name); }
  void pop_import();

 private:
  ScopedAllocator m_alloc;
  SymbolTable m_symbol_table;
  TypeContext m_type_ctx;
  std::vector<std::string> m_imports;
  std::vector<std::filesystem::path> m_import_paths;
  std::unordered_map<std::filesystem::path, Module*> m_modules;
};

}  // namespace via
