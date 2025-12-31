/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include <bit.hpp>
#include <compiler/value.hpp>
#include <module/manager.hpp>
#include <module/module.hpp>

#include "machine.hpp"
#include "value-ref.hpp"
#include "value.hpp"

#if defined(VIA_COMPILER_GCC) || defined(VIA_COMPILER_CLANG)
#define HAS_CGOTO
#endif

#ifdef HAS_CGOTO
#define CASE(OP) OP_##OP:
#else
#define CASE(OP) case OpCode::OP:
#endif

#define DISPATCH()                   \
  {                                  \
    if constexpr (!OverridePC) pc++; \
    if constexpr (SingleStep) {      \
      goto exit;                     \
    } else {                         \
      goto dispatch;                 \
    }                                \
  }

#define CV(ID) m_alloc.emplace<Value>(*this, consts.at(ID))
#define CV_REF(ID) ValueRef(CV(ID))

#define LGET(ID) reinterpret_cast<Value*>(stack.at(ID))
#define LSET(ID, VAL) stack.at(ID) = reinterpret_cast<uintptr_t>(VAL);
#define LFREE(ID)                                      \
  if (RGET(ID) != nullptr) {                           \
    reinterpret_cast<Value*>(stack.at(ID))->release(); \
  }

#define RGET(ID) regs[ID]
#define RSET(ID, VAL) regs[ID] = VAL
#define RFREE(ID)                       \
  [[likely]] if (RGET(ID) != nullptr) { \
    RGET(ID)->release();                \
    RSET(ID, nullptr);                  \
  }

#define JFWD(OFF) pc += (OFF) - 1
#define JBACK(OFF) pc -= (OFF) + 1

// Common Subexpression Elimination utility
#define CSE_A() const uint16_t a = pc->a;
#define CSE_AB() const uint16_t a = pc->a, b = pc->b;
#define CSE_ABC() const uint16_t a = pc->a, b = pc->b, c = pc->b;

#define DBG_TRAP(FORMAT, ...) debug::bug(std::format(FORMAT, __VA_ARGS__));

template <bool SingleStep, bool OverridePC>
[[gnu::flatten]] via::ExecutionError* via::VirtualMachine::m_execute() {
#ifdef HAS_CGOTO
  [[gnu::aligned(64)]] static void* dispatch_table[] = {
#define DEFINE_DISPATCH_OP(OP) &&OP_##OP,
      FOR_EACH_OPCODE(DEFINE_DISPATCH_OP)
#undef DEFINE_DISPATCH_OP
  };
#endif

  ExecutionError* error = nullptr;

  /* Explicit VM stuff CSE */
  auto& stack = m_stack;
  auto& regs = m_registers;
  auto& consts = m_exe.constants();

  /* Explicit module stuff CSE */
  auto& manager = m_module.manager();
  auto& symtab = manager.symbol_table();

  const auto*& pc = m_pc;

[[maybe_unused]] dispatch:

  if (m_has_interrupt()) [[unlikely]] {
    if (m_int == Interrupt::ERROR)
      error = reinterpret_cast<ExecutionError*>(m_int_arg);

    auto action = m_handle_interrupt();
    interrupt(Interrupt::NONE, nullptr);

    switch (action) {
      case IntAction::EXIT:
        goto exit;
      case IntAction::REINTERP:
        DISPATCH();
      case IntAction::RESUME:
        DISPATCH();
      default:
        break;
    }
  }

#ifdef HAS_CGOTO
  goto* dispatch_table[static_cast<uint16_t>(pc->op)];
  {
#else
  switch (pc->op) {
#endif
    CASE(NOP) { DISPATCH(); }
    CASE(HALT) { goto exit; }
    CASE(EXTRAARG) { goto trap__reserved_opcode; }
    CASE(MOVE) {
      CSE_AB();
      RFREE(a);
      RSET(a, RGET(b));
      RSET(b, nullptr);
      DISPATCH();
    }
    CASE(FREE1) {
      RFREE(pc->a);
      DISPATCH();
    }
    CASE(FREE2) {
      RFREE(pc->a);
      RFREE(pc->b);
      DISPATCH();
    }
    CASE(FREE3) {
      RFREE(pc->a);
      RFREE(pc->b);
      RFREE(pc->c);
      DISPATCH();
    }
    CASE(XCHG) {
      CSE_AB();
      Value* ra = RGET(a);
      RSET(a, RGET(b));
      RSET(b, ra);
      DISPATCH();
    }
    CASE(COPY) {
      CSE_A();
      RFREE(a);
      RSET(a, RGET(pc->b)->clone());
      DISPATCH();
    }
    CASE(COPYREF) {
      CSE_A();
      RFREE(a);
      RSET(a, RGET(pc->b));
      DISPATCH();
    }
    CASE(LOADK) {
      CSE_A();
      RFREE(a);
      RSET(a, CV(pc->b));
      DISPATCH();
    }
    CASE(LOADNONE) {
      CSE_A();
      RFREE(a);
      RSET(a, none.unwrap());
      DISPATCH();
    }
    CASE(LOADTRUE) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(true));
      DISPATCH();
    }
    CASE(LOADFALSE) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(false));
      DISPATCH();
    }
    CASE(LOADINT) {
      CSE_A();
      RFREE(a);
      RSET(a, m_alloc.emplace<Value>(
                  *this,
                  static_cast<int64_t>(pack_halves<uint32_t>(pc->b, pc->c))));
      DISPATCH();
    }
    CASE(NEWSTR)
    CASE(NEWARR)
    CASE(NEWDICT)
    CASE(NEWTUPLE) { goto trap__unimplemented_opcode; }
    CASE(NEWCLOSURE) {
      auto closure = Closure(m_pc);
      RSET(pc->a, value<FUNCTION>(closure));
      JFWD(pack_halves<uint32_t>(pc->b, pc->c));
      DISPATCH();
    }
    CASE(IADD) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(RGET(pc->b)->m_payload.integer +
                         RGET(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(IADDK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(RGET(pc->b)->m_payload.integer +
                         CV(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(FADD) {
      CSE_A();
      RFREE(a);
      RSET(a, value<FLOAT>(RGET(pc->b)->m_payload.float_ +
                           RGET(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(FADDK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<FLOAT>(RGET(pc->b)->m_payload.float_ +
                           CV(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(ISUB) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(RGET(pc->b)->m_payload.integer -
                         RGET(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(ISUBK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(RGET(pc->b)->m_payload.integer -
                         CV(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(FSUB) {
      CSE_A();
      RFREE(a);
      RSET(a, value<FLOAT>(RGET(pc->b)->m_payload.float_ -
                           RGET(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(FSUBK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<FLOAT>(RGET(pc->b)->m_payload.float_ -
                           CV(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(IMUL) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(RGET(pc->b)->m_payload.integer *
                         RGET(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(IMULK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(RGET(pc->b)->m_payload.integer *
                         CV(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(FMUL) {
      CSE_A();
      RFREE(a);
      RSET(a, value<FLOAT>(RGET(pc->b)->m_payload.float_ *
                           RGET(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(FMULK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<FLOAT>(RGET(pc->b)->m_payload.float_ *
                           CV(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(IDIV) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(RGET(pc->b)->m_payload.integer /
                         RGET(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(IDIVK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(RGET(pc->b)->m_payload.integer /
                         CV(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(FDIV) {
      CSE_A();
      RFREE(a);
      RSET(a, value<FLOAT>(RGET(pc->b)->m_payload.float_ /
                           RGET(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(FDIVK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<FLOAT>(RGET(pc->b)->m_payload.float_ /
                           CV(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(INEG) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(-RGET(pc->b)->m_payload.integer));
      DISPATCH();
    }
    CASE(INEGK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(-CV(pc->b)->m_payload.integer));
      DISPATCH();
    }
    CASE(FNEG) {
      CSE_A();
      RFREE(a);
      RSET(a, value<FLOAT>(-RGET(pc->b)->m_payload.float_));
      DISPATCH();
    }
    CASE(FNEGK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<FLOAT>(-CV(pc->b)->m_payload.float_));
      DISPATCH();
    }
    CASE(BAND) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(RGET(pc->b)->m_payload.integer &
                         RGET(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(BANDK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(RGET(pc->b)->m_payload.integer &
                         CV(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(BOR) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(RGET(pc->b)->m_payload.integer |
                         RGET(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(BORK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(RGET(pc->b)->m_payload.integer |
                         CV(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(BXOR) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(RGET(pc->b)->m_payload.integer ^
                         RGET(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(BXORK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(RGET(pc->b)->m_payload.integer ^
                         CV(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(BSHL) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(RGET(pc->b)->m_payload.integer
                         << RGET(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(BSHLK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(RGET(pc->b)->m_payload.integer
                         << CV(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(BSHR) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(RGET(pc->b)->m_payload.integer >>
                         RGET(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(BSHRK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(RGET(pc->b)->m_payload.integer >>
                         CV(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(BNOT) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(~RGET(pc->b)->m_payload.integer));
      DISPATCH();
    }
    CASE(BNOTK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<INT>(CV(pc->b)->m_payload.integer));
      DISPATCH();
    }
    CASE(AND) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.boolean &&
                          RGET(pc->c)->m_payload.boolean));
      DISPATCH();
    }
    CASE(ANDK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.boolean &&
                          CV(pc->c)->m_payload.boolean));
      DISPATCH();
    }
    CASE(OR) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.boolean ||
                          RGET(pc->c)->m_payload.boolean));
      DISPATCH();
    }
    CASE(ORK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.boolean ||
                          CV(pc->c)->m_payload.boolean));
      DISPATCH();
    }
    CASE(IEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.integer ==
                          RGET(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(IEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.integer ==
                          CV(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(FEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.float_ ==
                          RGET(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(FEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.float_ ==
                          CV(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(BEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.boolean ==
                          RGET(pc->c)->m_payload.boolean));
      DISPATCH();
    }
    CASE(BEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.boolean ==
                          CV(pc->c)->m_payload.boolean));
      DISPATCH();
    }
    CASE(SEQ) {
      CSE_A();
      RFREE(a)
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.string ==
                          RGET(pc->c)->m_payload.string));
      DISPATCH();
    }
    CASE(SEQK) {
      CSE_A();
      RFREE(a)
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.string ==
                          CV(pc->c)->m_payload.string));
      DISPATCH();
    }
    CASE(INEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.integer !=
                          RGET(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(INEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.integer !=
                          CV(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(FNEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.float_ !=
                          RGET(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(FNEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.float_ !=
                          CV(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(BNEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.boolean !=
                          RGET(pc->c)->m_payload.boolean));
      DISPATCH();
    }
    CASE(BNEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.boolean !=
                          CV(pc->c)->m_payload.boolean));
      DISPATCH();
    }
    CASE(SNEQ) {
      CSE_A();
      RFREE(a)
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.string ==
                          RGET(pc->c)->m_payload.string));
      DISPATCH();
    }
    CASE(SNEQK) {
      CSE_A();
      RFREE(a)
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.string ==
                          CV(pc->c)->m_payload.string));
      DISPATCH();
    }
    CASE(IS) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b) == RGET(pc->c)));
      DISPATCH();
    }
    CASE(ILT) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.integer <
                          RGET(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(ILTK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.integer <
                          CV(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(FLT) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.float_ <
                          RGET(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(FLTK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.float_ <
                          CV(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(IGT) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.integer >
                          RGET(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(IGTK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.integer >
                          CV(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(FGT) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.float_ >
                          RGET(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(FGTK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.float_ >
                          CV(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(ILTEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.integer <=
                          RGET(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(ILTEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.integer <=
                          CV(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(FLTEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.float_ <=
                          RGET(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(FLTEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.float_ <=
                          CV(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(IGTEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.integer >=
                          RGET(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(IGTEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.integer >=
                          CV(pc->c)->m_payload.integer));
      DISPATCH();
    }
    CASE(FGTEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.float_ >=
                          RGET(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(FGTEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(RGET(pc->b)->m_payload.float_ >=
                          CV(pc->c)->m_payload.float_));
      DISPATCH();
    }
    CASE(NOT) {
      CSE_A();
      RFREE(a);
      RSET(a, value<BOOL>(!RGET(pc->b)->m_payload.boolean));
      DISPATCH();
    }
    CASE(JMP) {
      JFWD(pack_halves<uint32_t>(pc->a, pc->b));
      DISPATCH();
    }
    CASE(JMPIF) {
      if (RGET(pc->a)->template as_c<BOOL>()) {
        JFWD(pack_halves<uint32_t>(pc->b, pc->c));
        DISPATCH();
      }
      DISPATCH();
    }
    CASE(JMPIFX) {
      if (!RGET(pc->a)->template as_c<BOOL>()) {
        JFWD(pack_halves<uint32_t>(pc->b, pc->c));
        DISPATCH();
      }
      DISPATCH();
    }
    CASE(JMPBACK) {
      JBACK(pack_halves<uint32_t>(pc->a, pc->b));
      DISPATCH();
    }
    CASE(JMPBACKIF) {
      if (RGET(pc->a)->template as_c<BOOL>()) {
        JBACK(pack_halves<uint32_t>(pc->b, pc->c));
        DISPATCH();
      }
      DISPATCH();
    }
    CASE(JMPBACKIFX) {
      if (!RGET(pc->a)->template as_c<BOOL>()) {
        JBACK(pack_halves<uint32_t>(pc->b, pc->c));
        DISPATCH();
      }
      DISPATCH();
    }
    CASE(SAVE) {
      m_save_stack();
      DISPATCH();
    }
    CASE(RESTORE) {
      m_restore_stack();
      DISPATCH();
    }
    CASE(PUSH) {
      auto* val = RGET(pc->a);
      val->m_rc++;
      push_local(ValueRef(val));
      DISPATCH();
    }
    CASE(PUSHK) {
      push_local(CV_REF(pc->a));
      DISPATCH();
    }
    CASE(GETTOP) {
      auto* val = reinterpret_cast<Value*>(stack.top());
      val->m_rc++;
      RSET(pc->a, val);
      DISPATCH();
    }
    CASE(GETARG)
    CASE(GETARGREF)
    CASE(SETARG) { goto trap__unimplemented_opcode; }
    CASE(GETLOCAL) {
      CSE_A();
      RFREE(a);
      RSET(a, LGET(pc->b)->clone());
      DISPATCH();
    }
    CASE(GETLOCALREF) {
      CSE_A();
      auto* local = LGET(pc->b);
      local->m_rc++;
      RFREE(a);
      RSET(a, local);
      DISPATCH();
    }
    CASE(SETLOCAL) {
      CSE_AB()
      LFREE(b);
      LSET(b, RGET(a));
      DISPATCH();
    }
    CASE(CALL) {
      call(ValueRef(RGET(pc->a)));
      DISPATCH();
    }
    CASE(PCALL) {
      call(ValueRef(RGET(pc->a)), CallFlags::PROTECT);
      DISPATCH();
    }
    CASE(RET) {
      return_(ValueRef(RGET(pc->a)));
      DISPATCH();
    }
    CASE(RETNONE) {
      return_(none);
      DISPATCH();
    }
    CASE(RETTRUE) {
      return_(value<BOOL>(true));
      DISPATCH();
    }
    CASE(RETFALSE) {
      return_(value<BOOL>(false));
      DISPATCH();
    }
    CASE(RETK) {
      return_(CV_REF(pc->a));
      DISPATCH();
    }
    CASE(TOINT) {
      RSET(pc->a, RGET(pc->b)->as<INT>().unwrap());
      DISPATCH();
    }
    CASE(TOFLOAT) {
      RSET(pc->a, RGET(pc->b)->as<FLOAT>().unwrap());
      DISPATCH();
    }
    CASE(TOBOOL) {
      RSET(pc->a, RGET(pc->b)->as<BOOL>().unwrap());
      DISPATCH();
    }
    CASE(TOSTRING) {
      RSET(pc->a, RGET(pc->b)->as<STRING>().unwrap());
      DISPATCH();
    }
    CASE(GETIMPORT) {
      CSE_A();
      auto import = get_import(pc->b, pc->c);
      import->m_rc++;
      RFREE(a);
      RSET(a, import.unwrap());
      DISPATCH();
    }
#ifdef HAS_CGOTO
  }
#else
    default:
      goto trap__unknown_opcode;
  }
#endif

  // clang-format off
[[maybe_unused]] trap__unknown_opcode:
    VIA_PANIC(std::format("trap: unknown opcode 0x{:x} ({})", (uint16_t) pc->op, to_string(pc->op)));
[[maybe_unused]] trap__reserved_opcode:
    VIA_PANIC(std::format("trap: reserved opcode 0x{:x} ({})", (uint16_t) pc->op, to_string(pc->op)));
[[maybe_unused]] trap__unimplemented_opcode:
    VIA_PANIC(std::format("trap: unimplemented opcode 0x{:x} ({})", (uint16_t) pc->op, to_string(pc->op)));
    DISPATCH();
  // clang-format on

exit:
  return error;
}

via::ExecutionError* via::VirtualMachine::execute() {
  return m_execute<false, false>();
}

via::ExecutionError* via::VirtualMachine::execute_once() {
  return m_execute<true, false>();
}
