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
  X(NIL)                       \
  X(INT)                       \
  X(FLOAT)                     \
  X(BOOL)                      \
  X(STRING)                    \
  X(FUNCTION)

enum ValueKind : uint8_t { FOR_EACH_VALUE_KIND(DEFINE_ENUM) };

DEFINE_TO_STRING(ValueKind, FOR_EACH_VALUE_KIND(DEFINE_CASE_TO_STRING))

#if defined(VIA_COMPILER_GCC) || defined(VIA_COMPILER_CLANG)
using float64 = _Float64;
#else
using float64 = double;
#endif

template <ValueKind Vk>
struct data_type;

// clang-format off
template <> struct data_type<NIL> { using type = std::monostate; };
template <> struct data_type<BOOL> { using type = bool; };
template <> struct data_type<INT> { using type = int64_t; };
template <> struct data_type<FLOAT> { using type = float64; };
// clang-format on

template <ValueKind Vk>
using data_type_t = typename data_type<Vk>::type;

class ConstValue final {
 public:
  using Union = std::variant<data_type_t<NIL>, data_type_t<BOOL>,
                             data_type_t<INT>, data_type_t<FLOAT>, std::string>;

 public:
  // clang-format off
    constexpr ConstValue() : m_data(std::monostate{}) {}
    constexpr explicit ConstValue(bool boolean) : m_data(boolean) {}
    constexpr explicit ConstValue(int64_t integer) : m_data(integer) {}
    constexpr explicit ConstValue(float64 float_) : m_data(float_) {}
    constexpr explicit ConstValue(std::string string) : m_data(string) {}
  // clang-format on

  static std::optional<ConstValue> from_token(const Token& tok);

 public:
  constexpr auto kind() const { return static_cast<ValueKind>(m_data.index()); }
  constexpr auto& data() { return m_data; }
  constexpr const auto& data() const { return m_data; }

  template <const ValueKind kind>
  constexpr auto unwrap() const {
    return std::get<static_cast<size_t>(kind)>(m_data);
  }

  constexpr bool compare(const ConstValue& other) const {
    return std::visit(
        [&other](auto&& lhs) -> bool {
          using T = std::decay_t<decltype(lhs)>;
          if (!std::holds_alternative<T>(other.m_data)) return false;
          return lhs == std::get<T>(other.m_data);
        },
        m_data);
  }

  std::string to_string() const;

 private:
  Union m_data;
};

}  // namespace via
