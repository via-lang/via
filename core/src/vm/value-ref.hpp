/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#pragma once

#include <mimalloc.h>

#include <config.hpp>
#include <cstddef>

#include "utility.hpp"
#include "value.hpp"

namespace via {

class VirtualMachine;
class ValueRef final {
 public:
  friend ::via::Value;
  friend ::via::VirtualMachine;

 public:
  inline ValueRef() : m_ptr(nullptr) {}
  inline ValueRef(ValueRef&& other) : m_ptr(other.m_ptr) {
    other.m_ptr = nullptr;
  }
  inline ValueRef(const ValueRef& other) : m_ptr(other.m_ptr) {
    if (!other.is_null()) acquire();
  }

  inline ~ValueRef() {
    if (!is_null()) release();
  }

  [[gnu::hot, gnu::flatten]] ValueRef& operator=(const ValueRef& other) {
    if (this != &other) {
      if (!other.is_null()) acquire();
      if (!is_null()) release();
      this->m_ptr = other.m_ptr;
    }
    return *this;
  }

  [[gnu::hot, gnu::flatten]] ValueRef& operator=(ValueRef&& other) {
    if (this != &other) {
      if (!is_null()) release();
      m_ptr = other.m_ptr;
      other.m_ptr = nullptr;
    }
    return *this;
  }

  inline Value* operator->() const {
    VIA_DEBUG_ASSERT(!is_null(), "attempt to read NULL ValueRef (operator->)");
    return m_ptr;
  }

  inline Value& operator*() const {
    VIA_DEBUG_ASSERT(!is_null(), "attempt to read NULL ValueRef (operator*)");
    return *m_ptr;
  }

 public:
  [[nodiscard]] Value* unwrap() const { return m_ptr; }
  [[nodiscard]] bool is_null() const { return m_ptr == nullptr; }

  [[gnu::flatten]] void acquire() {
    VIA_DEBUG_ASSERT(!is_null(), "acquire() called on NULL ValueRef");
    m_ptr->m_rc++;
  }

  [[gnu::flatten]] void release() {
    VIA_DEBUG_ASSERT(!is_null(), "release() called on NULL ValueRef");
    m_ptr->release();
    m_ptr = nullptr;
  }

  [[nodiscard, gnu::always_inline]] size_t refs() const {
    VIA_DEBUG_ASSERT(!is_null(), "refs() called on NULL ValueRef");
    return m_ptr->m_rc;
  }

 protected:
  inline ValueRef(Value* ptr) : m_ptr(ptr) {}

 private:
  Value* m_ptr;
};

}  // namespace via
