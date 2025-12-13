/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include "builder.hpp"

#include <cstring>

via::NativeModuleInfo* via::ModuleBuilder::build() & {
  auto& alloc = m_manager.allocator();
  auto size = m_defs.size();
  auto* buffer = alloc.emplace_array<const Binding*>(size);
  std::memcpy(buffer, m_defs.data(), size * sizeof(void*));
  return alloc.emplace<NativeModuleInfo>(buffer, size);
}

via::FunctionBinding& via::ModuleBuilder::function(std::string symbol) & {
  auto& alloc = m_manager.allocator();
  auto& symbols = m_manager.symbol_table();
  auto* bind =
      alloc.emplace<FunctionBinding>(m_manager, symbols.intern(symbol));
  m_defs.push_back(bind);
  return *bind;
}
