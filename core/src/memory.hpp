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
#include <functional>
#include <map>
#include <utility>

#include "utility.hpp"

namespace via {

struct ObjectTracker {
 public:
  struct Entry {
    size_t count;
    bool destroyed;
    void (*destroy)(void*, size_t);
  };

 public:
  ObjectTracker() = default;
  ~ObjectTracker() { clear(); }

 public:
  void clear();
  auto find(void* ptr) {
    auto it = m_tracker.find(ptr);
    return it != m_tracker.end() ? std::optional(std::ref(it->second))
                                 : std::nullopt;
  }

  void delete_at(void* ptr) {
    if (auto raw = find(ptr)) {
      auto& entry = raw.value().get();
      if (entry.destroyed) return;
      entry.destroyed = true;
      entry.destroy(ptr, entry.count);
    }
  }

  template <typename T, typename... Args>
  void construct_at(void* ptr, Args&&... args) {
    new (ptr) T(std::forward<Args>(args)...);
    m_tracker[ptr] = {
        .count = 1,
        .destroyed = false,
        .destroy = [](void* ptr, size_t) { ((T*)ptr)->~T(); },
    };
  }

  template <typename T, typename... Args>
  void construct_range_at(void* ptr, size_t count, Args&&... args) {
    for (size_t i = 0; i < count; ++i)
      new ((T*)ptr + i) T(std::forward<Args>(args)...);

    m_tracker[ptr] = {
        .count = count,
        .destroyed = false,
        .destroy =
            [](void* base, size_t count) {
              for (size_t i = count; i-- > 0;) ((T*)base + i)->~T();
            },
    };
  }

 private:
  std::map<void*, Entry> m_tracker;
};

struct DefaultAllocator {
  template <typename T>
  static T* alloc(size_t size) {
    return (T*)std::malloc(size * sizeof(T));
  }

  template <typename T>
  static void free(T* ptr) {
    std::free((void*)ptr);
  }
};

template <typename Alloc = DefaultAllocator>
class BumpAllocator final {
 public:
  BumpAllocator(size_t size)
    : m_base(Alloc::template alloc<std::byte>(size)),
      m_cursor(m_base),
      m_end(m_base + size) {}

  ~BumpAllocator() {
    m_tracker.clear();

    if (m_base) {
      Alloc::template free<std::byte>(m_base);
      m_base = nullptr;
      m_cursor = nullptr;
      m_end = nullptr;
    }
  }

  VIA_NOCOPY(BumpAllocator);
  VIA_NOMOVE(BumpAllocator);

 public:
  [[nodiscard]] inline void* alloc(size_t size,
                                   size_t align = alignof(std::max_align_t)) {
    std::uintptr_t cur = reinterpret_cast<std::uintptr_t>(m_cursor);
    std::uintptr_t aligned =
        (cur + (align - 1)) & ~(static_cast<std::uintptr_t>(align) - 1);
    std::byte* out = reinterpret_cast<std::byte*>(aligned);

    VIA_DEBUG_ASSERT(out + size <= m_end, "bump allocator overflow");
    m_cursor = out + size;
    return out;
  }

  template <typename T, typename... Args>
    requires std::is_constructible_v<T, Args...>
  [[nodiscard]] inline T* emplace(Args&&... args) {
    T* ptr = (T*)alloc(sizeof(T), alignof(T));
    m_tracker.construct_at<T>(ptr, std::forward<Args>(args)...);
    return ptr;
  }

  template <typename T, typename... Args>
    requires std::is_constructible_v<T, Args...>
  [[nodiscard]] inline T* emplace_array(size_t count, Args&&... args) {
    std::byte* block = (std::byte*)alloc(sizeof(T) * count, alignof(T));
    T* ptr = reinterpret_cast<T*>(block);
    m_tracker.construct_range_at<T>(ptr, count, std::forward<Args>(args)...);
    return ptr;
  }

 private:
  std::byte* m_base = nullptr;
  std::byte* m_cursor = nullptr;
  std::byte* m_end = nullptr;
  ObjectTracker m_tracker;
};

class ScopedAllocator final {
 public:
  ScopedAllocator();
  ~ScopedAllocator();

  VIA_NOCOPY(ScopedAllocator);
  VIA_NOMOVE(ScopedAllocator);

 public:
  bool owns(void* ptr);
  void* alloc(size_t size);
  char* strdup(const char* str);
  char* strndup(const char* str, size_t n);
  void free(void* ptr);

  template <typename T, typename... Args>
  [[nodiscard, gnu::flatten]] T* emplace(Args&&... args) {
    auto* buffer = (T*)alloc(sizeof(T));
    m_tracker.construct_at<T>(buffer, std::forward<Args>(args)...);
    return buffer;
  }

  template <typename T, typename... Args>
  [[nodiscard, gnu::flatten]] T* emplace_array(size_t count, Args&&... args) {
    auto* buffer = (T*)alloc(count * sizeof(T));
    m_tracker.construct_range_at<T>(buffer, count, std::forward<Args>(args)...);
    return buffer;
  }

 private:
  void* m_heap;
  ObjectTracker m_tracker;
};

#undef CONSTRUCT_AT
#undef CONSTRUCT_RANGE_AT

}  // namespace via
