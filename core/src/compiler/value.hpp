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
#include <cstddef>
#include <cstdint>
#include <optional>
#include <utility.hpp>
#include <variant>

#include "token.hpp"

namespace via {

#define FOR_EACH_VALUE_KIND(X) \
  X(NONE)                      \
  X(INT)                       \
  X(FLOAT)                     \
  X(BOOL)                      \
  X(STRING)                    \
  X(FUNCTION)

enum ValueKind : uint8_t { FOR_EACH_VALUE_KIND(VIA_DEFINE_ENUM) };

VIA_DEFINE_TO_STRING(ValueKind, FOR_EACH_VALUE_KIND(VIA_DEFINE_CASE_TO_STRING))

#if defined(VIA_COMPILER_GCC) || defined(VIA_COMPILER_CLANG)
using float64 = _Float64;
#else
using float64 = double;
#endif

template <ValueKind Vk>
struct DataType;

template <>
struct DataType<NONE> {
  using type = std::monostate;
};

template <>
struct DataType<BOOL> {
  using type = bool;
};

template <>
struct DataType<INT> {
  using type = int64_t;
};

template <>
struct DataType<FLOAT> {
  using type = float64;
};

template <ValueKind Vk>
using DataTypeT = typename DataType<Vk>::type;

class ConstValue final {
 public:
  using Union = std::variant<DataTypeT<NONE>, DataTypeT<BOOL>, DataTypeT<INT>,
                             DataTypeT<FLOAT>, std::string>;

 public:
  // clang-format off
    constexpr ConstValue() : m_payload(std::monostate{}) {}
    constexpr explicit ConstValue(bool boolean) : m_payload(boolean) {}
    constexpr explicit ConstValue(int64_t integer) : m_payload(integer) {}
    constexpr explicit ConstValue(float64 float_) : m_payload(float_) {}
    constexpr explicit ConstValue(std::string string) : m_payload(string) {}
  // clang-format on

  static std::optional<ConstValue> from_token(const Token& tok);

 public:
  constexpr auto kind() const {
    return static_cast<ValueKind>(m_payload.index());
  }
  constexpr auto& unwrap() { return m_payload; }
  constexpr const auto& unwrap() const { return m_payload; }

  template <const ValueKind kind>
  constexpr auto unwrap() const {
    return std::get<static_cast<size_t>(kind)>(m_payload);
  }

  constexpr bool compare(const ConstValue& other) const {
    return std::visit(
        [&other](auto&& lhs) -> bool {
          using T = std::decay_t<decltype(lhs)>;
          if (!std::holds_alternative<T>(other.m_payload)) return false;
          return lhs == std::get<T>(other.m_payload);
        },
        m_payload);
  }

  std::string to_string() const;

 private:
  Union m_payload;
};

}  // namespace via
