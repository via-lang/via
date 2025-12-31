/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#pragma once

#include <compiler/value.hpp>
#include <concepts>
#include <config.hpp>
#include <conversion.hpp>
#include <cstdint>
#include <cstring>
#include <optional>
#include <stdfloat>
#include <utility.hpp>

#include "closure.hpp"
#include "memory.hpp"

namespace via {

template <>
struct DataType<STRING> {
  using type = std::string;
};

template <>
struct DataType<FUNCTION> {
  using type = Closure;
};

template <ValueKind Vk>
struct CType {
  using type = DataTypeT<Vk>;
};

template <>
struct CType<BOOL> {
  using type = std::optional<DataTypeT<BOOL>>;
};

template <>
struct CType<INT> {
  using type = std::optional<DataTypeT<INT>>;
};

template <>
struct CType<FLOAT> {
  using type = std::optional<DataTypeT<FLOAT>>;
};

template <>
struct CType<STRING> {
  using type = std::string;
};

template <ValueKind Vk>
using CTypeT = typename CType<Vk>::type;

class Value final {
 public:
  friend ::via::ValueRef;
  friend ::via::VirtualMachine;
  friend ::via::ScopedAllocator;
  friend ::via::ObjectTracker;

 public:
  Value(const Value& other) = delete;
  Value(Value&& other) = delete;
  ~Value() { free(); }

  Value& operator=(const Value& other) = delete;
  Value& operator=(Value&& other) = delete;

  bool operator==(const Value& other) const { return compare(other); }
  bool operator==(const std::string& other) const {
    return m_kind == STRING && m_payload.string.compare(other) == 0;
  }

  template <std::integral T>
  bool operator==(const T& value) const {
    return m_kind == INT && m_payload.integer == value;
  }

  template <std::floating_point T>
  bool operator==(const T& value) const {
    return m_kind == FLOAT && m_payload.float_ == value;
  }

  template <size_t N>
  bool operator==(const char (&value)[N]) const {
    return m_kind == STRING && std::strncmp(m_payload.string.c_str(), value, N);
  }

 public:
  [[nodiscard]] auto kind() const { return m_kind; }
  [[nodiscard]] auto& unwrap() { return m_payload; }
  [[nodiscard]] const auto& unwrap() const { return m_payload; }
  bool release();
  void free();

  [[nodiscard]] Value* clone();
  [[nodiscard]] bool compare(const Value& other) const;
  [[nodiscard]] std::string to_string() const;

  template <ValueKind Vk>
  [[nodiscard]] DataTypeT<Vk>& unwrap();

  template <ValueKind Vk>
  [[nodiscard]] const DataTypeT<Vk>& unwrap() const;

  template <ValueKind Vk>
  [[nodiscard]] CTypeT<Vk> as_c() const;

  template <ValueKind Vk>
  [[nodiscard]] ValueRef as() const;

 protected:
  Value(VirtualMachine& vm) : m_vm(vm), m_kind(NONE) {}
  Value(VirtualMachine& vm, DataTypeT<BOOL> boolean)
    : m_vm(vm), m_kind(BOOL), m_payload(boolean) {}

  Value(VirtualMachine& vm, DataTypeT<INT> integer)
    : m_vm(vm), m_kind(INT), m_payload(integer) {}

  Value(VirtualMachine& vm, DataTypeT<FLOAT> float_)
    : m_vm(vm), m_kind(FLOAT), m_payload(float_) {}

  Value(VirtualMachine& vm, DataTypeT<STRING> string)
    : m_vm(vm), m_kind(STRING), m_payload(string) {}

  Value(VirtualMachine& vm, DataTypeT<FUNCTION> func)
    : m_vm(vm), m_kind(FUNCTION), m_payload(func) {}

  Value(VirtualMachine& vm, const ConstValue& cvalue);

 private:
  union Payload {
    DataTypeT<INT> integer;
    DataTypeT<FLOAT> float_;
    DataTypeT<BOOL> boolean;
    DataTypeT<STRING> string;
    DataTypeT<FUNCTION> function;

    ~Payload() {}
    Payload() {}
    Payload(DataTypeT<BOOL> boolean) : boolean(boolean) {}
    Payload(DataTypeT<INT> integer) : integer(integer) {}
    Payload(DataTypeT<FLOAT> float_) : float_(float_) {}
    Payload(DataTypeT<STRING> string) : string(string) {}
    Payload(DataTypeT<FUNCTION> function) : function(function) {}
    Payload(const Payload&) {}
    Payload(Payload&&) {}
    Payload& operator=(const Payload&) { return *this; }
    Payload& operator=(Payload&&) { return *this; }
  };

 private:
  VirtualMachine& m_vm;
  ValueKind m_kind = NONE;
  Payload m_payload = {};
  uint64_t m_rc = 1;
};

template <>
inline DataTypeT<BOOL>& Value::unwrap<BOOL>() {
  return m_payload.boolean;
}

template <>
inline DataTypeT<INT>& Value::unwrap<INT>() {
  return m_payload.integer;
}

template <>
inline DataTypeT<FLOAT>& Value::unwrap<FLOAT>() {
  return m_payload.float_;
}

template <>
inline DataTypeT<STRING>& Value::unwrap<STRING>() {
  return m_payload.string;
}

template <>
inline DataTypeT<FUNCTION>& Value::unwrap<FUNCTION>() {
  return m_payload.function;
}

template <>
inline const DataTypeT<BOOL>& Value::unwrap<BOOL>() const {
  return m_payload.boolean;
}

template <>
inline const DataTypeT<INT>& Value::unwrap<INT>() const {
  return m_payload.integer;
}

template <>
inline const DataTypeT<FLOAT>& Value::unwrap<FLOAT>() const {
  return m_payload.float_;
}

template <>
inline const DataTypeT<STRING>& Value::unwrap<STRING>() const {
  return m_payload.string;
}

template <>
inline const DataTypeT<FUNCTION>& Value::unwrap<FUNCTION>() const {
  return m_payload.function;
}

template <>
inline CTypeT<BOOL> Value::as_c<BOOL>() const {
  switch (m_kind) {
    case NONE:
      return false;
    case BOOL:
      return unwrap<BOOL>();
    default:
      break;
  }
  return std::nullopt;
}

template <>
inline CTypeT<INT> Value::as_c<INT>() const {
  using T = DataTypeT<INT>;
  switch (m_kind) {
    case FLOAT:
      return static_cast<T>(m_payload.float_);
    case STRING:
      return detail::stoi<T>(m_payload.string.c_str());
    case BOOL:
      return static_cast<T>(m_payload.boolean);
    case INT:
      return m_payload.integer;
    default:
      break;
  }
  return std::nullopt;
}

template <>
inline CTypeT<FLOAT> Value::as_c<FLOAT>() const {
  using T = DataTypeT<FLOAT>;
  switch (m_kind) {
    case INT:
      return static_cast<T>(m_payload.integer);
    case STRING:
      return detail::stof<T>(m_payload.string.c_str());
    case BOOL:
      return static_cast<T>(m_payload.boolean);
    case FLOAT:
      return m_payload.float_;
    default:
      break;
  }
  return std::nullopt;
}

template <>
inline CTypeT<STRING> Value::as_c<STRING>() const {
  switch (m_kind) {
    case NONE:
      return "none";
    case BOOL:
      return std::to_string(m_payload.boolean);
    case INT:
      return std::to_string(m_payload.integer);
    case FLOAT:
      return std::to_string(m_payload.float_);
    case STRING:
      return m_payload.string;
    case FUNCTION:
      return std::format("closure<{}>@{}",
                         m_payload.function.is_native() ? "native" : "bytecode",
                         reinterpret_cast<const void*>(&m_payload.function));
    default:
      break;
  }
  VIA_PANIC(via::to_string(m_kind));
}

}  // namespace via
