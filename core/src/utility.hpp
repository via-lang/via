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
#include <iostream>
#include <print>
#include <ranges>
#include <sstream>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>
#include <vector>

#include "ansi.hpp"
#include "type.hpp"

#define _PASTE_(a, b) a##b
#define _STRING_(X) #X

#define _EXPAND_(X) X
#define _EXPAND_STRING_(X) _STRING_(X)
#define _EXPAND_AND_PASTE_(A, B) _PASTE_(A, B)

#define VIA_TRY_COERCE(T, a, b) (T* a = dynamic_cast<T*>(b))
#define VIA_ISA(T, a) (dynamic_cast<T*>(a) != nullptr)

// xmacro utils
#define VIA_DEFINE_ENUM(OP) OP,
#define VIA_DEFINE_CASE(OP, ...) \
  case OP:                       \
    __VA_ARGS__
#define VIA_DEFINE_CASE_TO_STRING(OP) \
  case OP:                            \
    return _EXPAND_STRING_(OP);

// Enum utils
#define VIA_DEFINE_TO_STRING(ENUM, ...)            \
  constexpr std::string_view to_string(ENUM val) { \
    using enum ENUM;                               \
    switch (val) {                                 \
      __VA_ARGS__                                  \
      default:                                     \
        return "<error enum " #ENUM ">";           \
    }                                              \
  }

#define VIA_NOCOPY(TARGET)                   \
  TARGET& operator=(const TARGET&) = delete; \
  TARGET(const TARGET&) = delete;

#define VIA_IMPLCOPY(TARGET)        \
  TARGET& operator=(const TARGET&); \
  TARGET(const TARGET&);

#define VIA_NOMOVE(TARGET)              \
  TARGET& operator=(TARGET&&) = delete; \
  TARGET(TARGET&&) = delete;

#define VIA_IMPLMOVE(TARGET)   \
  TARGET& operator=(TARGET&&); \
  TARGET(TARGET&&);

#define _FIRST_ARG_(A, ...) A
#define _SECOND_ARG_(A, B, ...) B
#define _THIRD_ARG_(A, B, C, ...) C

#define _PANIC0_                                                       \
  do {                                                                 \
    std::println(                                                      \
        std::cerr, "{} {}:{}",                                         \
        via::ansi::format(                                             \
            "internal interpreter error:", via::ansi::Foreground::RED, \
            via::ansi::Background::NONE, via::ansi::Style::BOLD),      \
        __FILE_NAME__, __LINE__);                                      \
    std::exit(1);                                                      \
  } while (0);

#define _PANIC1_(MESSAGE)                                              \
  do {                                                                 \
    std::println(                                                      \
        std::cerr, "{} {}:{}: {}",                                     \
        via::ansi::format(                                             \
            "internal interpreter error:", via::ansi::Foreground::RED, \
            via::ansi::Background::NONE, via::ansi::Style::BOLD),      \
        __FILE_NAME__, __LINE__, (MESSAGE));                           \
    std::exit(1);                                                      \
  } while (0);

#define _PANIC_(_0, _1, NAME, ...) NAME
#define VIA_PANIC(...) _PANIC_(__VA_ARGS__, _PANIC1_, _PANIC0_)

#ifndef NDEBUG
#define VIA_DEBUG_PANIC(...) VIA_PANIC(__VA_ARGS__)
#else
#define VIA_DEBUG_PANIC(...) (void)0;
#endif

#if __cpp_lib_stacktrace >= 202011L
#include <stacktrace>
#define _PRINT_STACK_TRACE_                                    \
  do {                                                         \
    std::println(std::cerr);                                   \
    std::println(std::cerr, "{}", std::stacktrace::current()); \
  } while (0);
#else
#define _PRINT_STACK_TRACE_
#endif

#define _ASSERT1_(CONDITION)                                       \
  if (CONDITION) {                                                 \
    VIA_PANIC(std::format("assertion failure: '{}'", #CONDITION)); \
    _PRINT_STACK_TRACE_;                                           \
  }

#define _ASSERT2_(CONDITION, MESSAGE)                                        \
  if (CONDITION) {                                                           \
    VIA_PANIC(                                                               \
        std::format("assertion failure: '{}' ({})", #CONDITION, (MESSAGE))); \
    _PRINT_STACK_TRACE_;                                                     \
  }

#define _ASSERT_(...) _THIRD_ARG_(__VA_ARGS__, _ASSERT2_, _ASSERT1_, )
#define VIA_ASSERT(...) _ASSERT_(__VA_ARGS__)(__VA_ARGS__)

#ifndef NDEBUG
#define VIA_DEBUG_ASSERT(...) VIA_ASSERT(__VA_ARGS__);
#else
#define VIA_DEBUG_ASSERT(...) (void)0;
#endif

namespace via {
namespace detail {

template <typename Stream, typename T>
concept streamable_into = requires(Stream& stream, T a) {
  { stream << a } -> std::same_as<Stream&>;
};

}  // namespace detail

template <std::ranges::range Range,
          std::invocable<std::ranges::range_value_t<Range>> Fn>
  requires std::convertible_to<
      std::invoke_result_t<Fn, std::ranges::range_value_t<Range>>, std::string>
constexpr std::string to_string(const Range& range, Fn callback,
                                std::string_view open = "[",
                                std::string_view close = "]",
                                std::string_view delimiter = ",") {
  std::ostringstream oss;
  oss << open;

  auto it = range.begin();
  auto end = range.end();

  while (it != end) {
    oss << callback(*it);
    if (std::next(it) != end) oss << delimiter;
    ++it;
  }

  oss << close;
  return oss.str();
}

template <std::ranges::range Range>
  requires detail::streamable_into<std::ostringstream,
                                   std::ranges::range_value_t<Range>>
constexpr std::string to_string(const Range& range, std::string_view open = "[",
                                std::string_view close = "]",
                                std::string_view delimiter = ",") {
  std::ostringstream oss;
  oss << open;

  auto it = range.begin();
  auto end = range.end();

  while (it != end) {
    oss << *it;
    if (std::next(it) != end) oss << delimiter;
    ++it;
  }

  oss << close;
  return oss.str();
}

}  // namespace via
