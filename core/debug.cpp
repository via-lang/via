/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include "debug.hpp"

#include <cpptrace/cpptrace.hpp>
#include <iostream>

#include "logger.hpp"

#ifdef NDEBUG
#define CRASH_IMPL(MSG)
#define ASSERT_IMPL(COND, MSG)
#define UNREACHABLE() std::unreachable()
#else
#define CRASH_IMPL(MSG) (::log_error(MSG), ::via::debug::panic())
#define ASSERT_IMPL(COND, MSG) (!COND) ? CRASH_IMPL(MSG) : void(0)
#define UNREACHABLE()
#endif

static void log_error(std::string message) {
  static auto logger = via::Logger::stderr_logger();
  logger.error("{}", message);
}

[[noreturn]] void via::debug::panic() noexcept {
  log_error("program execution panicked");
  cpptrace::generate_trace().print(std::cerr);
  std::abort();
}

void via::debug::require(bool cond, std::string message) noexcept {
  ASSERT_IMPL(
      cond, std::format(
                "program execution reached failing `debug::require()` call: {}",
                message));
  UNREACHABLE();
}

[[noreturn]] void via::debug::bug(std::string message) noexcept {
  ASSERT_IMPL(false,
              std::format("program execution reached `debug::bug()` call: {}",
                          message));
  UNREACHABLE();
}

[[noreturn]] void via::debug::todo(std::string message) noexcept {
  ASSERT_IMPL(false,
              std::format("program execution reached `debug::todo()` call: {}",
                          message));
  UNREACHABLE();
}

[[noreturn]] void via::debug::unimplemented(std::string message) noexcept {
  ASSERT_IMPL(
      false,
      std::format("program execution reached `debug::unimplemented()` call: {}",
                  message));
  UNREACHABLE();
}
