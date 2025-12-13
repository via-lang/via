/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#pragma once

#include <cstddef>
#include <iostream>
#include <libassert/assert.hpp>
#include <optional>
#include <via/config.hpp>

#include "diagnostics.hpp"
#include "instruction.hpp"
#include "ir/tree.hpp"
#include "sema/const_value.hpp"
#include "sema/local_bc.hpp"
#include "sema/register.hpp"
#include "sema/stack.hpp"
#include "support/type.hpp"

namespace via {
namespace config {

constexpr uint32_t MAGIC = 0x2E766961;  // .via

}

class Executable;

namespace detail {

void set_null_dst_trap(Executable& exe,
                       const std::optional<uint16_t>& dst) noexcept;

}  // namespace detail

enum ExeFlags : uint64_t {
  NONE = 0,
};

class Module;
class Executable final {
 public:
  friend void detail::set_null_dst_trap(
      Executable&, const std::optional<uint16_t>& dst) noexcept;

 public:
  Executable(Diagnostics& diags) : m_reg_state(diags) { m_stack.emplace(); }

  static Executable* build(Module* module, Diagnostics& diags,
                           const IRTree& ir_tree,
                           ExeFlags flags = ExeFlags::NONE);

  static Executable* build(Module* module, Diagnostics& diags,
                           std::ostream& bytes,
                           ExeFlags flags = ExeFlags::NONE);

 public:
  auto flags() const noexcept { return m_flags; }
  auto& constants() const noexcept { return m_constants; }
  auto& bytecode() const noexcept { return m_bytecode; }
  std::string to_string() const;

 private:
  size_t program_counter() const noexcept { return m_bytecode.size() - 1; }
  size_t constant_id() const noexcept { return m_constants.size() - 1; }
  size_t set_label(size_t id) noexcept {
    m_labels[id] = program_counter();
    return m_labels.size() - 1;
  }

  void push_constant(ConstValue cvalue) noexcept {
    DEBUG_ASSERT(
        m_constants.size() < (size_t)std::numeric_limits<uint16_t>::max(),
        "Constant count exceeds limit");
    m_constants.push_back(std::move(cvalue));
  }

  size_t push_instruction(OpCode op,
                          std::array<uint16_t, 3> ops = {}) noexcept {
    m_bytecode.emplace_back(op, ops[0], ops[1], ops[2]);
    return program_counter();
  }

  void set_instruction(size_t pc, OpCode op,
                       std::array<uint16_t, 3> ops = {}) noexcept {
    auto& insn = m_bytecode[pc];
    insn.op = op;
    insn.a = ops[0];
    insn.b = ops[1];
    insn.c = ops[2];
  }

  void lower(const ir::Expr* expr, std::optional<uint16_t> dst);
  void lower(const ir::Stat* stat);
  void lower(const ir::Term* term);
  void lower_jumps();

  template <derived_from<ir::Expr> Expr>
  void lower_expr(const Expr* expr, std::optional<uint16_t> dst) {}

  template <derived_from<ir::Stat> Stat>
  void lower_stat(const Stat* stat) {}

  template <derived_from<ir::Term> Term>
  void lower_term(const Term* term) {}

 private:
  Module* m_module;
  ExeFlags m_flags;
  RegisterState m_reg_state;
  StackState<BytecodeLocal> m_stack;
  std::vector<Instruction> m_bytecode;
  std::vector<ConstValue> m_constants;
  std::unordered_map<size_t, size_t> m_labels;
};

}  // namespace via
