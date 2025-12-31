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
#include <expected>
#include <filesystem>
#include <type_traits>
#include <utility.hpp>

// We assume that the <dlfcn.h> header is the proper POSIX dynamic library API
// Everything should be fine as long as the implementation is not cooked
#if __has_include(<dlfcn.h>)
#define HAS_DLFCN 1
#else
#define HAS_DLFCN 0
#endif

namespace via {
namespace os {

#if HAS_DLFCN
#define DL_SUPPORTED 1
constexpr auto DL_EXTENSION = ".so";
#elif defined(VIA_PLATFORM_WINDOWS)
#define DL_SUPPORTED 1
constexpr auto DL_EXTENSION = ".dll";
#else
#define DL_SUPPORTED 0
#endif

class DynamicLibrary final {
 public:
  DynamicLibrary() = default;
  ~DynamicLibrary();

  VIA_IMPLMOVE(DynamicLibrary);
  VIA_NOCOPY(DynamicLibrary);

 public:
  [[nodiscard]] static consteval bool supported() { return DL_SUPPORTED; }
  [[nodiscard]] static std::expected<DynamicLibrary, std::string> load(
      std::filesystem::path path);

 public:
  [[nodiscard]] std::expected<void*, std::string> raw_symbol(
      const char* symbol);

  template <typename T>
    requires std::is_pointer_v<T>
  [[nodiscard]] std::expected<T, std::string> symbol(const char* symbol) {
    auto result = raw_symbol(symbol);
    if (result.has_value()) return reinterpret_cast<T>(*result);
    return std::unexpected(result.error());
  }

 private:
  explicit DynamicLibrary(void* handle) : m_handle(handle) {}

 private:
  void* m_handle = nullptr;
};

}  // namespace os
}  // namespace via
