/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include "token.hpp"

#include <format>

#include "lexer/source_buffer.hpp"

std::string via::Token::get_dump() const {
  return std::format("[{} '{}']", via::to_string(kind),
                     (*lexeme == '\0') ? "<eof>" : to_string());
}
