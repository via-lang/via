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
#include <memory.hpp>
#include <utility.hpp>

#include "machine.hpp"

via::Value::Value(VirtualMachine& vm, const ConstValue& cvalue)
  : m_vm(vm), m_kind(cvalue.kind()) {
  switch (m_kind) {
    case BOOL:
      m_payload.boolean = cvalue.unwrap<BOOL>();
      break;
    case INT:
      m_payload.integer = cvalue.unwrap<INT>();
      break;
    case FLOAT:
      m_payload.float_ = cvalue.unwrap<FLOAT>();
      break;
    case STRING:
      m_payload.string = cvalue.unwrap<STRING>();
      break;
    default:
      break;
  }
  VIA_PANIC();
}

bool via::Value::release() {
  m_rc--;
  [[unlikely]] if (m_rc == 0) {
    free();
    return true;
  }
  return false;
}

void via::Value::free() {
  switch (m_kind) {
    case STRING:
      m_payload.string.~basic_string();
      break;
    case FUNCTION:
      m_payload.function.~Closure();
      break;
    default:
      break;
  }
  m_kind = NONE;
}

via::Value* via::Value::clone() {
  switch (m_kind) {
    case NONE:
      return m_vm.none.unwrap();
    case BOOL:
      return m_vm.value<BOOL>(m_payload.boolean);
    case INT:
      return m_vm.value<INT>(m_payload.integer);
    case FLOAT:
      return m_vm.value<FLOAT>(m_payload.float_);
    case STRING:
      return m_vm.value<STRING>(m_payload.string);
    case FUNCTION:
      return m_vm.value<FUNCTION>(m_payload.function);
    default:
      break;
  }
  VIA_PANIC();
}

bool via::Value::compare(const Value& other) const {
  if (m_kind != other.m_kind) return false;
  switch (m_kind) {
    case NONE:
      return true;
    case BOOL:
      return m_payload.boolean == other.m_payload.boolean;
    case INT:
      return m_payload.integer == other.m_payload.integer;
    case FLOAT:
      return m_payload.float_ == other.m_payload.float_;
    case STRING:
      return m_payload.string == other.m_payload.string;
    default:
      break;
  }
  VIA_PANIC();
}

std::string via::Value::to_string() const {
  return std::format("&{} {}({}) ", m_rc, via::to_string(m_kind),
                     as_c<STRING>());
}
