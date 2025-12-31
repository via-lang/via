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
#include <vector>

#include "instruction.hpp"
#include "stack.hpp"

namespace via {

class Value;
class ValueRef;
class VirtualMachine;

using NativeCallback = ValueRef (*)(VirtualMachine* vm, CallInfo& ci);

class Closure final {
 public:
  explicit Closure(const Instruction* pc)
    : m_native(false), m_payload({.bytecode = pc}) {}
  explicit Closure(size_t argc, const NativeCallback callback)
    : m_native(true), m_argc(argc), m_payload({.callback = callback}) {}

  Closure(const Closure& other)
    : m_native(other.m_native),
      m_argc(other.m_argc),
      m_payload(other.m_payload) {}

 public:
  [[nodiscard]] size_t argc() const { return m_argc; }
  [[nodiscard]] bool is_native() const { return m_native; }
  [[nodiscard]] auto bytecode() const { return m_payload.bytecode; }
  [[nodiscard]] auto callback() const { return m_payload.callback; }
  [[nodiscard]] auto& upvalues() const { return m_upvs; }

 private:
  const bool m_native;
  const size_t m_argc = 0;
  std::vector<Value*> m_upvs;

  union {
    const Instruction* bytecode;
    const NativeCallback callback;
  } m_payload;
};

}  // namespace via
