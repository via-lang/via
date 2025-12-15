/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include "value.hpp"

#include <compiler/value.hpp>
#include <libassert/assert.hpp>
#include <memory.hpp>

// clang-format off
via::Value* via::Value::create(VirtualMachine* vm)
    { return create(vm, NIL); }
via::Value* via::Value::create(VirtualMachine* vm, int64_t integer)
    { return create(vm, INT, {.integer = integer}); }
via::Value* via::Value::create(VirtualMachine* vm, float64 float_)
    { return create(vm, FLOAT, {.float_ = float_}); }
via::Value* via::Value::create(VirtualMachine* vm, bool boolean)
    { return create(vm, BOOL, {.boolean = boolean}); }
// clang-format on

via::Value* via::Value::create(VirtualMachine* vm, char* string) {
  DEBUG_ASSERT(vm->allocator().owns(string),
               "Value construction via string requires it to be allocated by "
               "the corresponding Value::vm");
  return create(vm, STRING, {.string = string});
}

via::Value* via::Value::create(VirtualMachine* vm, Closure* closure) {
  DEBUG_ASSERT(
      vm->allocator().owns(closure),
      "Value construction via closure object requires it to be allocated by "
      "the corresponding Value::vm");
  return create(vm, FUNCTION, {.function = closure});
}

via::Value* via::Value::create(VirtualMachine* vm, const ConstValue& cv) {
  auto& alloc = vm->allocator();

  switch (cv.kind()) {
    case NIL:
      return create(vm);
    case BOOL:
      return create(vm, cv.unwrap<BOOL>());
    case INT:
      return create(vm, cv.unwrap<INT>());
    case FLOAT:
      return create(vm, cv.unwrap<FLOAT>());
    case STRING: {
      auto string = cv.unwrap<STRING>();
      auto buffer = alloc.strdup(string.c_str());
      return create(vm, buffer);
    }
    default:
      break;
  }
  UNREACHABLE();
}

bool via::Value::unref() noexcept {
  m_rc--;
  [[unlikely]] if (m_rc == 0) {
    free();
    return true;
  }
  return false;
}

void via::Value::free() noexcept {
  switch (m_kind) {
    case STRING:
    case FUNCTION:
      m_vm->allocator().free(std::bit_cast<void*>(m_data));
      break;
    default:
      // Trivial types don't require explicit destruction
      break;
  }

  m_kind = NIL;
}

via::Value* via::Value::clone() noexcept {
  return create(m_vm, m_kind, m_data);
}

std::string via::Value::to_string() const noexcept {
  return std::format("&{} {}({}) ", m_rc, via::to_string(m_kind),
                     as_c<STRING>());
}

via::Value* via::Value::create(VirtualMachine* vm, ValueKind kind, Union data) {
  Value* ptr = vm->allocator().emplace<Value>();
  ptr->m_kind = kind;
  ptr->m_data = data;
  ptr->m_vm = vm;
  return ptr;
}
