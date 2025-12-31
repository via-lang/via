/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#pragma once

#include <compiler/executable.hpp>
#include <config.hpp>
#include <cstddef>
#include <functional>
#include <module/symbol.hpp>
#include <string>
#include <utility.hpp>

#include "compiler/value.hpp"
#include "instruction.hpp"
#include "stack.hpp"
#include "value-ref.hpp"
#include "value.hpp"

namespace via {
namespace config {
namespace vm {

constexpr size_t REGISTER_COUNT = std::numeric_limits<uint16_t>::max() + 1;

}
}  // namespace config

class Closure;
class VirtualMachine;

#define FOR_EACH_INTERRUPT(X) \
  X(NONE)                     \
  X(ERROR)

enum class Interrupt : uint8_t { FOR_EACH_INTERRUPT(VIA_DEFINE_ENUM) };

enum class IntAction {
  RESUME,
  REINTERP,
  EXIT,
};

VIA_DEFINE_TO_STRING(Interrupt, FOR_EACH_INTERRUPT(VIA_DEFINE_CASE_TO_STRING));

using InterruptHook = void (*)(VirtualMachine*, Interrupt, void*);
using StackUnwindCallback =
    std::function<bool(const uintptr_t* fp, const Instruction* pc,
                       const CallFlags flags, ValueRef callee)>;

class Value;
class Snapshot {
 public:
  explicit Snapshot(VirtualMachine* vm);

 public:
  std::string to_string() const;

 public:
  const uintptr_t sp;
  const uintptr_t fp;
  const Instruction* pc;
  const size_t rpc;
  const std::vector<uintptr_t> stack;
  const std::vector<Value*> registers;
};

struct ExecutionError {
  ValueRef err;
  const uintptr_t* fp;
  const Instruction* pc;

  [[nodiscard]] std::string to_string() const {
    return std::format("ExecutionError(error: {}, fp: {}, pc: {}@{})",
                       err->to_string(), (void*)fp, pc->to_string(false),
                       (void*)pc);
  }
};

class ValueRef;
class Debugger;
class ModuleManager;
class VirtualMachine final {
 public:
  friend ::via::Snapshot;
  friend ::via::Debugger;

 public:
  explicit VirtualMachine(Module& module, const Executable& exe);

  VIA_NOCOPY(VirtualMachine);
  VIA_NOMOVE(VirtualMachine);

 public:
  [[nodiscard]] ScopedAllocator& allocator() { return m_alloc; }
  [[nodiscard]] ValueRef get_import(SymbolId module_id, SymbolId key_id);
  [[nodiscard]] ValueRef get_constant(uint16_t id);
  [[nodiscard]] ValueRef get_local(size_t sp);
  [[nodiscard]] ExecutionError* execute();
  [[nodiscard]] ExecutionError* execute_once();
  void push_local(ValueRef val);
  void call(ValueRef callee, CallFlags flags = CallFlags::NONE);
  void return_(ValueRef value);
  void raise(ValueRef error);
  void free(Value& value);
  void interrupt_hook(InterruptHook hook) { m_int_hook = hook; }
  void interrupt(Interrupt code, void* arg = nullptr);

  template <ValueKind Vk>
  [[nodiscard]] Value* value(DataTypeT<Vk> data) {
    return m_alloc.emplace<Value>(*this, data);
  }

  template <ValueKind Vk>
  [[nodiscard]] ValueRef value_ref(DataTypeT<Vk> data) {
    return m_alloc.emplace<Value>(*this, data);
  }

 private:
  void m_save_stack();
  void m_restore_stack();
  Closure* m_unwind_stack(StackUnwindCallback pred);
  bool m_has_interrupt() const { return m_int != Interrupt::NONE; }
  IntAction m_handle_interrupt();

  template <Interrupt Int>
  inline IntAction m_handle() {
    VIA_PANIC(Int);
  }

  template <bool SingleStep, bool OverridePC>
  ExecutionError* m_execute();

 protected:
  const Executable& m_exe;
  ScopedAllocator m_alloc;
  Module& m_module;
  uintptr_t *m_sp, *m_fp = nullptr;
  const Instruction *m_bp, *m_pc;
  Stack<uintptr_t> m_stack;
  Value** m_registers;
  Interrupt m_int = Interrupt::NONE;
  InterruptHook m_int_hook = nullptr;
  void* m_int_arg;

 public:
  const ValueRef none;
};

template <>
inline ValueRef Value::as<BOOL>() const {
  auto value = as_c<BOOL>();
  VIA_DEBUG_ASSERT(value);
  return m_vm.value_ref<BOOL>(*value);
}

template <>
inline ValueRef Value::as<INT>() const {
  auto value = as_c<INT>();
  VIA_DEBUG_ASSERT(value);
  return m_vm.value_ref<INT>(*value);
}

template <>
inline ValueRef Value::as<FLOAT>() const {
  auto value = as_c<FLOAT>();
  VIA_DEBUG_ASSERT(value);
  return m_vm.value_ref<FLOAT>(*value);
}

template <>
inline ValueRef Value::as<STRING>() const {
  return m_vm.value_ref<STRING>(as_c<STRING>());
}

}  // namespace via
