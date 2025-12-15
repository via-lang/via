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
#include <string>

namespace via {

struct SourceLoc {
  size_t begin;
  size_t end;
};

struct RelSourceLoc {
  size_t line;
  size_t offset;
};

struct Token;

class SourceBuffer final {
 public:
  SourceBuffer() = default;
  SourceBuffer(std::string&& source) : m_buffer(source) {}

 public:
  const char* begin() const { return m_buffer.data(); }
  const char* end() const { return m_buffer.data() + m_buffer.size(); }

 public:
  bool is_valid_range(SourceLoc loc) const;
  SourceLoc location(const char* begin, const char* end) const;
  SourceLoc location(const Token& tok) const;
  SourceLoc to_absolute(RelSourceLoc loc) const;
  RelSourceLoc to_relative(SourceLoc loc) const;
  std::string get_slice(SourceLoc loc) const;

 private:
  const std::string m_buffer;
};

}  // namespace via
