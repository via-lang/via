/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include <cstring>

#include "debug.hpp"
#include "machine.hpp"
#include "module/manager.hpp"
#include "reference.hpp"
#include "support/bit.hpp"
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

#define CV(ID)                         \
  ({                                   \
    auto cv = consts.at(ID);           \
    auto* val = Value::create(vm, cv); \
    val;                               \
  })

#define CV_REF(ID)                     \
  ({                                   \
    auto cv = consts.at(ID);           \
    auto* val = Value::create(vm, cv); \
    ValueRef(vm, val);                 \
  })

#define LGET(ID) reinterpret_cast<Value*>(stack.at(ID))
#define LSET(ID, VAL) stack.at(ID) = reinterpret_cast<uintptr_t>(VAL);
#define LFREE(ID)                                    \
  if (RGET(ID) != nullptr) {                         \
    reinterpret_cast<Value*>(stack.at(ID))->unref(); \
  }

#define RGET(ID) regs[ID]
#define RSET(ID, VAL) regs[ID] = VAL
#define RFREE(ID)                       \
  [[likely]] if (RGET(ID) != nullptr) { \
    RGET(ID)->unref();                  \
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
[[gnu::flatten]] void via::detail::execute(VirtualMachine* vm) {
#ifdef HAS_CGOTO
  [[gnu::aligned(64)]] static void* dispatch_table[] = {
#define DEFINE_DISPATCH_OP(OP) &&OP_##OP,
      FOR_EACH_OPCODE(DEFINE_DISPATCH_OP)
#undef DEFINE_DISPATCH_OP
  };
#endif

  /* Explicit VM stuff CSE */
  auto& stack = vm->m_stack;
  auto& regs = vm->m_registers;
  auto& consts = vm->m_exe->constants();

  /* Explicit module stuff CSE */
  auto& manager = vm->m_module->manager();
  auto& symtab = manager.symbol_table();

  const auto*& pc = vm->m_pc;

[[maybe_unused]] dispatch:

  [[unlikely]] if (vm->has_interrupt()) {
    auto action = vm->handle_interrupt();
    vm->set_interrupt(Interrupt::NONE, nullptr);

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
    CASE(LOADNIL) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm));
      DISPATCH();
    }
    CASE(LOADTRUE) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, true));
      DISPATCH();
    }
    CASE(LOADFALSE) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, false));
      DISPATCH();
    }
    CASE(LOADINT) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, static_cast<int64_t>(
                                    pack_halves<uint32_t>(pc->b, pc->c))));
      DISPATCH();
    }
    CASE(NEWSTR)
    CASE(NEWARR)
    CASE(NEWDICT)
    CASE(NEWTUPLE) { goto trap__unimplemented_opcode; }
    CASE(NEWCLOSURE) {
      auto closure = Closure::create(vm, vm->m_pc);
      RSET(pc->a, Value::create(vm, closure));
      JFWD(pack_halves<uint32_t>(pc->b, pc->c));
      DISPATCH();
    }
    CASE(IADD) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(RGET(pc->b)->m_data.integer +
                                        RGET(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(IADDK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(RGET(pc->b)->m_data.integer +
                                        CV(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(FADD) {
      CSE_A();
      RFREE(a);
      RSET(a,
           Value::create(vm, data_type_t<FLOAT>(RGET(pc->b)->m_data.float_ +
                                                RGET(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(FADDK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, data_type_t<FLOAT>(RGET(pc->b)->m_data.float_ +
                                                   CV(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(ISUB) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(RGET(pc->b)->m_data.integer -
                                        RGET(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(ISUBK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(RGET(pc->b)->m_data.integer -
                                        CV(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(FSUB) {
      CSE_A();
      RFREE(a);
      RSET(a,
           Value::create(vm, data_type_t<FLOAT>(RGET(pc->b)->m_data.float_ -
                                                RGET(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(FSUBK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, data_type_t<FLOAT>(RGET(pc->b)->m_data.float_ -
                                                   CV(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(IMUL) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(RGET(pc->b)->m_data.integer *
                                        RGET(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(IMULK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(RGET(pc->b)->m_data.integer *
                                        CV(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(FMUL) {
      CSE_A();
      RFREE(a);
      RSET(a,
           Value::create(vm, data_type_t<FLOAT>(RGET(pc->b)->m_data.float_ *
                                                RGET(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(FMULK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, data_type_t<FLOAT>(RGET(pc->b)->m_data.float_ *
                                                   CV(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(IDIV) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(RGET(pc->b)->m_data.integer /
                                        RGET(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(IDIVK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(RGET(pc->b)->m_data.integer /
                                        CV(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(FDIV) {
      CSE_A();
      RFREE(a);
      RSET(a,
           Value::create(vm, data_type_t<FLOAT>(RGET(pc->b)->m_data.float_ /
                                                RGET(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(FDIVK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, data_type_t<FLOAT>(RGET(pc->b)->m_data.float_ /
                                                   CV(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(INEG) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(-RGET(pc->b)->m_data.integer)));
      DISPATCH();
    }
    CASE(INEGK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(-CV(pc->b)->m_data.integer)));
      DISPATCH();
    }
    CASE(FNEG) {
      CSE_A();
      RFREE(a);
      RSET(a,
           Value::create(vm, data_type_t<FLOAT>(-RGET(pc->b)->m_data.float_)));
      DISPATCH();
    }
    CASE(FNEGK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, data_type_t<FLOAT>(-CV(pc->b)->m_data.float_)));
      DISPATCH();
    }
    CASE(BAND) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(RGET(pc->b)->m_data.integer &
                                        RGET(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(BANDK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(RGET(pc->b)->m_data.integer &
                                        CV(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(BOR) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(RGET(pc->b)->m_data.integer |
                                        RGET(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(BORK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(RGET(pc->b)->m_data.integer |
                                        CV(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(BXOR) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(RGET(pc->b)->m_data.integer ^
                                        RGET(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(BXORK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(RGET(pc->b)->m_data.integer ^
                                        CV(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(BSHL) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(RGET(pc->b)->m_data.integer
                                        << RGET(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(BSHLK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(RGET(pc->b)->m_data.integer
                                        << CV(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(BSHR) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(RGET(pc->b)->m_data.integer >>
                                        RGET(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(BSHRK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(RGET(pc->b)->m_data.integer >>
                                        CV(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(BNOT) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(~RGET(pc->b)->m_data.integer)));
      DISPATCH();
    }
    CASE(BNOTK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, int64_t(CV(pc->b)->m_data.integer)));
      DISPATCH();
    }
    CASE(AND) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.boolean &&
                                     RGET(pc->c)->m_data.boolean)));
      DISPATCH();
    }
    CASE(ANDK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.boolean &&
                                     CV(pc->c)->m_data.boolean)));
      DISPATCH();
    }
    CASE(OR) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.boolean ||
                                     RGET(pc->c)->m_data.boolean)));
      DISPATCH();
    }
    CASE(ORK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.boolean ||
                                     CV(pc->c)->m_data.boolean)));
      DISPATCH();
    }
    CASE(IEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.integer ==
                                     RGET(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(IEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.integer ==
                                     CV(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(FEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.float_ ==
                                     RGET(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(FEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.float_ ==
                                     CV(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(BEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.boolean ==
                                     RGET(pc->c)->m_data.boolean)));
      DISPATCH();
    }
    CASE(BEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.boolean ==
                                     CV(pc->c)->m_data.boolean)));
      DISPATCH();
    }
    CASE(SEQ) {
      CSE_A();
      RFREE(a)
      RSET(a, Value::create(
                  vm, bool(std::strcmp(RGET(pc->b)->m_data.string,
                                       RGET(pc->c)->m_data.string) == 0)));
      DISPATCH();
    }
    CASE(SEQK) {
      CSE_A();
      RFREE(a)
      RSET(a,
           Value::create(vm, bool(std::strcmp(RGET(pc->b)->m_data.string,
                                              CV(pc->c)->m_data.string) == 0)));
      DISPATCH();
    }
    CASE(INEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.integer !=
                                     RGET(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(INEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.integer !=
                                     CV(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(FNEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.float_ !=
                                     RGET(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(FNEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.float_ !=
                                     CV(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(BNEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.boolean !=
                                     RGET(pc->c)->m_data.boolean)));
      DISPATCH();
    }
    CASE(BNEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.boolean !=
                                     CV(pc->c)->m_data.boolean)));
      DISPATCH();
    }
    CASE(SNEQ) {
      CSE_A();
      RFREE(a)
      RSET(a, Value::create(
                  vm, bool(std::strcmp(RGET(pc->b)->m_data.string,
                                       RGET(pc->c)->m_data.string) != 0)));
      DISPATCH();
    }
    CASE(SNEQK) {
      CSE_A();
      RFREE(a)
      RSET(a,
           Value::create(vm, bool(std::strcmp(RGET(pc->b)->m_data.string,
                                              CV(pc->c)->m_data.string) != 0)));
      DISPATCH();
    }
    CASE(IS) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b) == RGET(pc->c))));
      DISPATCH();
    }
    CASE(ILT) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.integer <
                                     RGET(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(ILTK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.integer <
                                     CV(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(FLT) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.float_ <
                                     RGET(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(FLTK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.float_ <
                                     CV(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(IGT) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.integer >
                                     RGET(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(IGTK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.integer >
                                     CV(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(FGT) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.float_ >
                                     RGET(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(FGTK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.float_ >
                                     CV(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(ILTEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.integer <=
                                     RGET(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(ILTEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.integer <=
                                     CV(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(FLTEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.float_ <=
                                     RGET(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(FLTEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.float_ <=
                                     CV(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(IGTEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.integer >=
                                     RGET(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(IGTEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.integer >=
                                     CV(pc->c)->m_data.integer)));
      DISPATCH();
    }
    CASE(FGTEQ) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.float_ >=
                                     RGET(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(FGTEQK) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(RGET(pc->b)->m_data.float_ >=
                                     CV(pc->c)->m_data.float_)));
      DISPATCH();
    }
    CASE(NOT) {
      CSE_A();
      RFREE(a);
      RSET(a, Value::create(vm, bool(!RGET(pc->b)->m_data.boolean)));
      DISPATCH();
    }
    CASE(JMP) {
      JFWD(pack_halves<uint32_t>(pc->a, pc->b));
      DISPATCH();
    }
    CASE(JMPIF) {
      if (RGET(pc->a)->as_c<BOOL>()) {
        JFWD(pack_halves<uint32_t>(pc->b, pc->c));
        DISPATCH();
      }
      DISPATCH();
    }
    CASE(JMPIFX) {
      if (!RGET(pc->a)->as_c<BOOL>()) {
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
      if (RGET(pc->a)->as_c<BOOL>()) {
        JBACK(pack_halves<uint32_t>(pc->b, pc->c));
        DISPATCH();
      }
      DISPATCH();
    }
    CASE(JMPBACKIFX) {
      if (!RGET(pc->a)->as_c<BOOL>()) {
        JBACK(pack_halves<uint32_t>(pc->b, pc->c));
        DISPATCH();
      }
      DISPATCH();
    }
    CASE(SAVE) {
      vm->save_stack();
      DISPATCH();
    }
    CASE(RESTORE) {
      vm->restore_stack();
      DISPATCH();
    }
    CASE(PUSH) {
      auto* val = RGET(pc->a);
      val->m_rc++;
      vm->push_local(ValueRef(vm, val));
      DISPATCH();
    }
    CASE(PUSHK) {
      vm->push_local(CV_REF(pc->a));
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
      vm->call(ValueRef(vm, RGET(pc->a)));
      DISPATCH();
    }
    CASE(PCALL) {
      vm->call(ValueRef(vm, RGET(pc->a)), CallFlags::PROTECT);
      DISPATCH();
    }
    CASE(RET) {
      vm->return_(ValueRef(vm, RGET(pc->a)));
      DISPATCH();
    }
    CASE(RETNIL) {
      vm->return_(ValueRef(vm, Value::create(vm)));
      DISPATCH();
    }
    CASE(RETTRUE) {
      vm->return_(ValueRef(vm, Value::create(vm, true)));
      DISPATCH();
    }
    CASE(RETFALSE) {
      vm->return_(ValueRef(vm, Value::create(vm, false)));
      DISPATCH();
    }
    CASE(RETK) {
      vm->return_(CV_REF(pc->a));
      DISPATCH();
    }
    CASE(TOINT) {
      RSET(pc->a, RGET(pc->b)->as<INT>());
      DISPATCH();
    }
    CASE(TOFLOAT) {
      RSET(pc->a, RGET(pc->b)->as<FLOAT>());
      DISPATCH();
    }
    CASE(TOBOOL) {
      RSET(pc->a, RGET(pc->b)->as<BOOL>());
      DISPATCH();
    }
    CASE(TOSTRING) {
      RSET(pc->a, RGET(pc->b)->as<STRING>());
      DISPATCH();
    }
    CASE(GETIMPORT) {
      CSE_A();
      auto import = vm->get_import(pc->b, pc->c);
      import->m_rc++;
      RFREE(a);
      RSET(a, import.get());
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
    DBG_TRAP("trap: unknown opcode 0x{:x} ({})", (uint16_t) pc->op, to_string(pc->op));
[[maybe_unused]] trap__reserved_opcode:
    DBG_TRAP("trap: reserved opcode 0x{:x} ({})", (uint16_t) pc->op, to_string(pc->op));
[[maybe_unused]] trap__unimplemented_opcode:
    DBG_TRAP("trap: unimplemented opcode 0x{:x} ({})", (uint16_t) pc->op, to_string(pc->op));
    DISPATCH();
  // clang-format on

exit:;
}

void via::VirtualMachine::execute() { detail::execute<false, false>(this); }

void via::VirtualMachine::execute_once() { detail::execute<true, false>(this); }
