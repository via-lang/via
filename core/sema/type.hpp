/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#pragma once

#include <functional>
#include <libassert/assert.hpp>
#include <type_traits>
#include <vector>
#include <via/config.hpp>

#include "support/enum.hpp"
#include "support/memory.hpp"
#include "support/type.hpp"
#include "support/utility.hpp"

namespace via {

class TypeContext;

enum class CastResult {
  OK,
  THROW,
  INVALID,
};

enum class TypeFlags : uint8_t {
  NONE = 0,
  DEPENDENT = 1 << 0,
};

enum class TypeQualifier {
  NONE = 0,
  CONST = 1 << 0,
  STRONG = 1 << 1,
  REFERENCE = 1 << 2,
};

#define FOR_EACH_BUILTIN_KIND(X) \
  X(NIL)                         \
  X(BOOL)                        \
  X(INT)                         \
  X(FLOAT)                       \
  X(STRING)

enum class BuiltinKind : uint8_t { FOR_EACH_BUILTIN_KIND(DEFINE_ENUM) };

DEFINE_TO_STRING(BuiltinKind, FOR_EACH_BUILTIN_KIND(DEFINE_CASE_TO_STRING))

class Type {
 public:
  virtual ~Type() = default;

 public:
  bool is_dependent() const { return flags & 0x1; }

  virtual bool is_integral() const { return false; }
  virtual bool is_float() const { return false; }
  virtual bool is_arithmetic() const { return false; }
  virtual bool is_callable() const { return false; }
  virtual bool is_subscriptable() const { return false; }
  virtual CastResult cast_result(const Type* to) const = 0;
  virtual std::string to_string() const { UNREACHABLE(); }

 public:
  const TypeFlags flags;

 protected:
  explicit Type(TypeFlags flags = TypeFlags::NONE) : flags(flags) {}
};

class QualType final {
 public:
  QualType() = default;
  QualType(const Type* type) : m_type(type) {}

  bool operator==(const QualType& other) const {
    return m_type == other.m_type;
  }
  bool operator!=(const QualType& other) const {
    return m_type != other.m_type;
  }
  operator bool() const { return m_type != nullptr; }

 public:
  auto unwrap() const { return m_type; }
  auto qualifiers() const { return m_quals; }
  bool is_dependent() const { return m_type->is_dependent(); }
  bool is_const() const { return m_quals & TypeQualifier::CONST; }
  bool is_strong() const { return m_quals & TypeQualifier::STRONG; }
  bool is_reference() const { return m_quals & TypeQualifier::REFERENCE; }
  CastResult cast_result(QualType to) const;
  std::string to_string() const;

 private:
  const Type* m_type = nullptr;
  TypeQualifier m_quals = TypeQualifier::NONE;
};

class BuiltinType final : public Type {
 public:
  explicit BuiltinType(BuiltinKind kind)
      : Type(TypeFlags::NONE), m_kind(kind) {}

 public:
  BuiltinKind kind() const { return m_kind; }
  CastResult cast_result(const Type* to) const override;
  std::string to_string() const override;

  template <BuiltinKind... Kinds>
  bool is_one_of() const {
    return ((m_kind == Kinds) || ...);
  }

 private:
  const BuiltinKind m_kind;
};

class OptionalType final : public Type {
 public:
  explicit OptionalType(QualType type)
      : Type((TypeFlags)type.is_dependent()), m_type(type) {}

 public:
  QualType unwrap() const { return m_type; }
  CastResult cast_result(const Type* to) const override;
  std::string to_string() const override;

 private:
  QualType m_type;
};

class ArrayType final : public Type {
 public:
  explicit ArrayType(QualType type)
      : Type((TypeFlags)type.is_dependent()), m_type(type) {}

 public:
  QualType unwrap() const { return m_type; }
  CastResult cast_result(const Type* to) const override;
  std::string to_string() const override;

 private:
  QualType m_type;
};

struct MapType : public Type {
 public:
  explicit MapType(QualType key, QualType value)
      : Type(TypeFlags(key.is_dependent() || value.is_dependent())),
        m_key(key),
        m_value(value) {}

 public:
  QualType key() const { return m_key; }
  QualType value() const { return m_value; }
  CastResult cast_result(const Type* to) const override;
  std::string to_string() const override;

 private:
  QualType m_key, m_value;
};

struct FunctionType : public Type {
 public:
  explicit FunctionType(QualType ret, std::vector<QualType> parms)
      : Type(({
          bool dependent = ret.is_dependent();
          for (const auto& type : parms) dependent |= type.is_dependent();
          (TypeFlags) dependent;
        })),
        m_return(ret),
        m_parms(std::move(parms)) {}

 public:
  auto returns() const { return m_return; }
  auto parameters() const { return m_parms; }
  CastResult cast_result(const Type* to) const override;
  std::string to_string() const override;

 private:
  QualType m_return;
  std::vector<QualType> m_parms;
};

struct MapKey {
  QualType key, val;
};

struct FunctionKey {
  QualType result;
  std::vector<QualType> parms;
};

}  // namespace via

// clang-format off
template <> struct std::hash<via::QualType> { size_t operator()(const via::QualType& type) const; };
template <> struct std::hash<via::MapKey> { size_t operator()(const via::MapKey& key) const; };
template <> struct std::hash<via::FunctionKey> { size_t operator()(const via::FunctionKey& key) const; };

template <> struct std::equal_to<via::QualType> { bool operator()(const via::QualType& a, const via::QualType& b) const; };
template <> struct std::equal_to<via::MapKey> { bool operator()(const via::MapKey& a, const via::MapKey& b) const; };
template <> struct std::equal_to<via::FunctionKey> { bool operator()(const via::FunctionKey& a, const via::FunctionKey& b) const; };
// clang-format on

namespace via {

class TypeContext final {
 public:
  template <derived_from<Type> T, typename... Args>
    requires std::is_constructible_v<T, Args...>
  const T* instance(Args... args);

 private:
  template <typename T, typename K, typename... Args>
    requires std::is_constructible_v<T, Args...> &&
             std::is_constructible_v<K, Args...>
  const T* m_instantiate(std::unordered_map<K, const T*>& map, Args&&... args) {
    K key(args...);
    if (auto it = map.find(key); it != map.end()) return it->second;
    T* type = m_alloc.emplace<T>(args...);
    return map[key] = type, type;
  }

 private:
  BumpAllocator<> m_alloc{8 * 1024 * 1024};
  std::unordered_map<BuiltinKind, const BuiltinType*> m_builtins;
  std::unordered_map<QualType, const OptionalType*> m_optionals;
  std::unordered_map<QualType, const ArrayType*> m_arrays;
  std::unordered_map<MapKey, const MapType*> m_maps;
  std::unordered_map<FunctionKey, const FunctionType*> m_functions;
};

template <>
inline const via::BuiltinType*
via::TypeContext::instance<via::BuiltinType, via::BuiltinKind>(
    BuiltinKind kind) {
  return m_instantiate(m_builtins, kind);
}

template <>
inline const via::OptionalType*
via::TypeContext::instance<via::OptionalType, via::QualType>(QualType type) {
  return m_instantiate(m_optionals, std::move(type));
}

template <>
inline const via::ArrayType*
via::TypeContext::instance<via::ArrayType, via::QualType>(QualType type) {
  return m_instantiate(m_arrays, std::move(type));
}

template <>
inline const via::MapType*
via::TypeContext::instance<via::MapType, via::QualType, via::QualType>(
    QualType key, QualType value) {
  return m_instantiate(m_maps, std::move(key), std::move(value));
}

template <>
inline const via::FunctionType* via::TypeContext::instance<
    via::FunctionType, via::QualType, std::vector<via::QualType>>(
    QualType ret, std::vector<QualType> parms) {
  return m_instantiate(m_functions, std::move(ret), std::move(parms));
}

}  // namespace via
