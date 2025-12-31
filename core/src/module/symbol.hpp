/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#pragma once

#include <cstdint>
#include <deque>
#include <intern-table.hpp>
#include <sstream>
#include <string>

namespace via {

using SymbolId = uint64_t;
using QualName = std::deque<std::string>;

[[nodiscard]] std::string to_string(const QualName& path);

class SymbolTable final : public InternTable<std::string, SymbolId> {
 public:
  using InternTable::intern;

 public:
  [[nodiscard]] SymbolId intern(const QualName& path) {
    return intern(via::to_string(path));
  }
  [[nodiscard]] std::string to_string() const;
};

}  // namespace via
