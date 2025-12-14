/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#pragma once

#include <cstdint>
#include <libassert/assert.hpp>
#include <optional>
#include <stdfloat>
#include <via/config.hpp>

#include "closure.hpp"
#include "machine.hpp"
#include "sema/const_value.hpp"
#include "support/conv.hpp"

namespace via {

template <>
struct data_type<STRING> {
  using type = char*;
};

template <>
struct data_type<FUNCTION> {
  using type = Closure*;
};

template <ValueKind Vk>
struct c_type {
  using type = data_type_t<Vk>;
};

template <>
struct c_type<INT> {
  using type = std::optional<data_type_t<INT>>;
};

template <>
struct c_type<FLOAT> {
  using type = std::optional<data_type_t<FLOAT>>;
};

template <>
struct c_type<STRING> {
  using type = std::string;
};

template <ValueKind Vk>
using c_type_t = typename c_type<Vk>::type;

class Value final {
 public:
  union Union {
    data_type_t<INT> integer;
    data_type_t<FLOAT> float_;
    data_type_t<BOOL> boolean;
    data_type_t<STRING> string;
    data_type_t<FUNCTION> function;
  };

  friend class ValueRef;
  friend class VirtualMachine;

 public:
  static Value* create(VirtualMachine* vm);
  static Value* create(VirtualMachine* vm, int64_t integer);
  static Value* create(VirtualMachine* vm, float64 float_);
  static Value* create(VirtualMachine* vm, bool boolean);
  static Value* create(VirtualMachine* vm, char* string);
  static Value* create(VirtualMachine* vm, Closure* closure);
  static Value* create(VirtualMachine* vm, const ConstValue& cv);

 public:
  auto kind() const { return m_kind; }
  auto& data() { return m_data; }
  const auto& data() const { return m_data; }
  auto* context() const { return m_vm; }
  bool unref() noexcept;
  void free() noexcept;
  Value* clone() noexcept;
  std::string to_string() const noexcept;

  // clang-format off
  template <ValueKind Vk> data_type_t<Vk> unwrap() const;
  template <ValueKind Vk> c_type_t<Vk> as_c() const;
  template <ValueKind Vk> Value* as() const;
  // clang-format on

 private:
  static Value* create(VirtualMachine* vm, ValueKind kind, Union data = {});

 private:
  ValueKind m_kind = NIL;
  Union m_data = {};
  uint64_t m_rc = 1;
  VirtualMachine* m_vm;
};

template <>
inline data_type_t<BOOL> Value::unwrap<BOOL>() const {
  return m_data.boolean;
}

template <>
inline data_type_t<INT> Value::unwrap<INT>() const {
  return m_data.integer;
}

template <>
inline data_type_t<FLOAT> Value::unwrap<FLOAT>() const {
  return m_data.float_;
}

template <>
inline data_type_t<STRING> Value::unwrap<STRING>() const {
  return m_data.string;
}

template <>
inline data_type_t<FUNCTION> Value::unwrap<FUNCTION>() const {
  return m_data.function;
}

template <>
inline c_type_t<BOOL> Value::as_c<BOOL>() const {
  // clang-format off
  switch (m_kind) {
  case NIL:  return false;
  case BOOL: return unwrap<BOOL>();
  default:   break;
  }  // clang-format on
  return true;
}

template <>
inline c_type_t<INT> Value::as_c<INT>() const {
  using T = data_type_t<INT>;
  // clang-format off
  switch (m_kind) {
    case FLOAT:  return static_cast<T>(m_data.float_);
    case STRING: return detail::stoi<T>(m_data.string);
    case BOOL:   return static_cast<T>(m_data.boolean);
    case INT:    return m_data.integer;
    default:     break;
  }  // clang-format on
  return std::nullopt;
}

template <>
inline c_type_t<FLOAT> Value::as_c<FLOAT>() const {
  using T = data_type_t<FLOAT>;
  // clang-format off
  switch (m_kind) {
    case INT:    return static_cast<T>(m_data.integer);
    case STRING: return detail::stof<T>(m_data.string);
    case BOOL:   return static_cast<T>(m_data.boolean);
    case FLOAT:  return m_data.float_;
    default:     break;
  }  // clang-format on
  return std::nullopt;
}

template <>
inline c_type_t<STRING> Value::as_c<STRING>() const {
  // clang-format off
  switch (m_kind) {
    case NIL:      return "nil";
    case BOOL:     return std::to_string(m_data.boolean);
    case INT:      return std::to_string(m_data.integer);
    case FLOAT:    return std::to_string(m_data.float_);
    case STRING:   return m_data.string;
    case FUNCTION: return std::format("closure<{}>@{}",
                      m_data.function->is_native() ? "native" : "bytecode",
                      reinterpret_cast<void*>(m_data.function));
    default:       break;
  }  // clang-format on
  UNREACHABLE(via::to_string(m_kind));
}

template <>
inline Value* Value::as<INT>() const {
  auto val = as_c<INT>();
  DEBUG_ASSERT_VAL(val);
  return create(m_vm, *val);
}

template <>
inline Value* Value::as<FLOAT>() const {
  auto val = as_c<FLOAT>();
  DEBUG_ASSERT_VAL(val);
  return create(m_vm, *val);
}

template <>
inline Value* Value::as<BOOL>() const {
  return Value::create(m_vm, as_c<BOOL>());
}

template <>
inline Value* Value::as<STRING>() const {
  auto& alloc = m_vm->allocator();
  auto string = as_c<STRING>();
  auto buffer = alloc.strdup(string.c_str());
  return Value::create(m_vm, buffer);
}

}  // namespace via
