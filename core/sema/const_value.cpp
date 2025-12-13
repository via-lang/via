/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include "const_value.hpp"

#include <format>
#include <optional>

#include "debug.hpp"
#include "support/conv.hpp"

std::optional<via::ConstValue> via::ConstValue::from_token(const Token& tok) {
  switch (tok.kind) {
    case TokenKind::LIT_NIL:
      return ConstValue();
    case TokenKind::LIT_TRUE:
      return ConstValue(true);
    case TokenKind::LIT_FALSE:
      return ConstValue(false);
    case TokenKind::LIT_INT:
    case TokenKind::LIT_XINT:
    case TokenKind::LIT_BINT:
      if (auto val = detail::stoi<data_type_t<INT>>(tok.to_view()))
        return ConstValue(*val);
      break;
    case TokenKind::LIT_FLOAT:
      if (auto val = detail::stof<data_type_t<FLOAT>>(tok.to_string()))
        return ConstValue(*val);
      break;
    case TokenKind::LIT_STRING: {
      std::string_view view = tok.to_view();
      std::string literal(view.begin() + 1, view.end() - 1);
      return ConstValue(literal);
    }
    default:
      break;
  }
  return std::nullopt;
}

std::string via::ConstValue::to_string() const {
  // clang-format off
  switch (kind()) {
    case NIL:    return "nil";
    case BOOL:   return unwrap<BOOL>() ? "true" : "false";
    case INT:    return std::to_string(unwrap<INT>());
    case FLOAT:  return std::to_string(unwrap<FLOAT>());
    case STRING: return std::format("\"{}\"", unwrap<STRING>());
    default:     break;
  }  // clang-format on
  debug::unimplemented();
}
