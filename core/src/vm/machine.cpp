/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include "machine.hpp"

#include <compiler/value.hpp>
#include <module/binding.hpp>
#include <module/manager.hpp>
#include <module/module.hpp>

#include "closure.hpp"
#include "utility.hpp"
#include "value-ref.hpp"
#include "value.hpp"

via::VirtualMachine::VirtualMachine(Module& module, const Executable& exe)
  : m_exe(exe),
    m_alloc(),
    m_module(module),
    m_bp(exe.bytecode().data()),
    m_pc(m_bp),
    m_stack(m_alloc),
    m_registers(m_alloc.emplace_array<Value*>(config::vm::REGISTER_COUNT)),
    none(m_alloc.emplace<Value>(*this)) {}

template <>
via::IntAction via::VirtualMachine::m_handle<via::Interrupt::ERROR>() {
  Closure* handler = m_unwind_stack(
      [](auto, auto, auto flags, auto) { return flags & CallFlags::PROTECT; });
  return handler ? IntAction::RESUME : IntAction::EXIT;
}

via::Snapshot::Snapshot(VirtualMachine* vm)
  : sp(vm->m_sp - vm->m_stack.base()),
    fp(vm->m_fp ? vm->m_fp - vm->m_stack.base() : 0),
    pc(vm->m_pc),
    rpc(vm->m_pc - vm->m_bp),
    stack(vm->m_stack.begin(), vm->m_stack.end()),
    registers(vm->m_registers, vm->m_registers + config::vm::REGISTER_COUNT) {}

std::string via::Snapshot::to_string() const {
  std::ostringstream oss;
  return oss.str();
}

void via::VirtualMachine::m_save_stack() { m_sp = m_stack.end(); }
void via::VirtualMachine::m_restore_stack() {
  for (auto* ptr = m_stack.end() - 1; ptr > m_sp; ptr--) {
    auto* value = reinterpret_cast<Value*>(*ptr);
    value->release();
  }
  m_stack.jump(m_sp);
}

via::IntAction via::VirtualMachine::m_handle_interrupt() {
  if (m_int_hook != nullptr) m_int_hook(this, m_int, m_int_arg);

#define DEFINE_INTERRUPT_HANDLER(INT) \
  case Interrupt::INT:                \
    return m_handle<Interrupt::INT>();

  switch (m_int) {
    FOR_EACH_INTERRUPT(DEFINE_INTERRUPT_HANDLER)
    default:
      break;
  }
  return IntAction::RESUME;
}

via::Closure* via::VirtualMachine::m_unwind_stack(StackUnwindCallback pred) {
  for (uintptr_t* fp = m_fp; fp != nullptr;) {
    m_stack.jump(fp + 1);

    auto* this_fp = reinterpret_cast<uintptr_t*>(m_stack.pop());
    auto* this_pc = reinterpret_cast<const Instruction*>(m_stack.pop());
    auto flags = static_cast<CallFlags>(m_stack.pop());
    auto* callee = reinterpret_cast<Value*>(m_stack.pop());

    if (pred(this_fp, this_pc, flags, ValueRef(callee))) {
      return &callee->unwrap<FUNCTION>();
    } else {
      fp = this_fp;
      callee->release();
    }
  }
  return nullptr;
}

via::ValueRef via::VirtualMachine::get_import(SymbolId module_id,
                                              SymbolId key_id) {
  auto& manager = m_module.manager();
  if (auto module = manager.get_module_by_name(module_id)) {
    if (auto bind = module->lookup(key_id)) {
      if VIA_TRY_COERCE (const FunctionBinding, fn_def, *bind) {
        if (fn_def->m_kind != ImplKind::NATIVE) goto error;
        auto argc = fn_def->m_params.size();
        auto closure = Closure(argc, fn_def->m_impl.native);
        return value<FUNCTION>(closure);
      }
    }
  }

// TODO: Better error handling
error:
  VIA_PANIC("invalid call to VirtualMachine::get_import");
}

void via::VirtualMachine::interrupt(Interrupt code, void* arg) {
  if (m_int_arg != nullptr) m_alloc.free(m_int_arg);
  m_int = code;
  m_int_arg = arg;
}

void via::VirtualMachine::push_local(ValueRef val) {
  val->m_rc++;  // Manually increment reference count as the stack is managed
                // manually
  m_stack.push((uintptr_t)val.unwrap());  // Push the value onto the stack
}

via::ValueRef via::VirtualMachine::get_local(size_t sp) {
  // Ensure the stack pointer is within bounds
  VIA_DEBUG_ASSERT(sp < m_stack.size(), "bad stack pointer");
  return ValueRef((Value*)m_stack.at(sp));
}

via::ValueRef via::VirtualMachine::get_constant(uint16_t id) {
  const auto& cvalue = m_exe.constants().at(id);  // Get the constant value
  return m_alloc.emplace<Value>(*this, cvalue);
}

void via::VirtualMachine::call(ValueRef callee, CallFlags flags) {
  callee->m_rc++;  // Keep callee alive just in case

  auto& closure = callee->unwrap<FUNCTION>();
  auto* base = &m_stack.top();

  m_stack.push((uintptr_t)callee.unwrap());
  m_stack.push((uintptr_t)flags);
  m_stack.push((uintptr_t)m_pc + !closure.is_native());
  m_stack.push((uintptr_t)m_fp);

  m_fp = &m_stack.top();

  if (closure.is_native()) {
    CallInfo call_info;
    call_info.callee = callee.unwrap();
    call_info.flags = flags;

    for (uintptr_t* ptr = base; ptr > base - (ptrdiff_t)closure.argc(); --ptr) {
      Value* arg = reinterpret_cast<Value*>(*ptr);
      call_info.args.push_back(ValueRef(arg));
    }

    auto result = closure.callback()(this, call_info);
    return_(result);
  } else {
    m_pc = closure.bytecode();
  }
}

void via::VirtualMachine::return_(ValueRef value) {
  VIA_DEBUG_ASSERT(
      m_fp != nullptr,
      "bad internal frame pointer while returning from function call");

  uintptr_t* top = &m_stack.top();

  for (uintptr_t* local = top; local > m_fp; --local) {
    if (*local) {
      auto* val = reinterpret_cast<Value*>(local);
      val->release();
    }
  }

  m_stack.jump(m_fp + 1);

  m_fp = reinterpret_cast<uintptr_t*>(m_stack.pop());
  m_pc = reinterpret_cast<const Instruction*>(m_stack.pop());

  auto flags = static_cast<CallFlags>(m_stack.pop());
  auto* callee = reinterpret_cast<Value*>(m_stack.pop());
  callee->release();

  push_local(value.is_null() ? none : value);
}

void via::VirtualMachine::raise(ValueRef error) {
  auto* payload = m_alloc.emplace<ExecutionError>();
  payload->err = std::move(error);
  payload->fp = m_fp;
  payload->pc = m_pc;

  interrupt(Interrupt::ERROR, (void*)payload);
}
