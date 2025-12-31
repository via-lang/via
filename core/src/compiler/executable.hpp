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
#include <diagnostics.hpp>
#include <iostream>
#include <optional>

#include "ir-tree.hpp"
#include "register.hpp"
#include "stack.hpp"
#include "type.hpp"
#include "value.hpp"

namespace via {
namespace config {

constexpr uint32_t MAGIC = 0x2E766961;  // .via

}

class Executable;
class BytecodeLocal final {
 public:
  struct Ref {
    uint16_t id;
    BytecodeLocal* local;

    Ref() = default;
    Ref(uint16_t id, BytecodeLocal* local) : id(id), local(local) {}
  };

 public:
  BytecodeLocal() = default;
  BytecodeLocal(SymbolId symbol, size_t version)
    : m_symbol(symbol), m_version(version) {}

 public:
  [[nodiscard]] auto symbol() const { return m_symbol; }
  [[nodiscard]] auto version() const { return m_version; }

 protected:
  SymbolId m_symbol;
  size_t m_version;
};

enum class ExeFlags : uint64_t {
  NONE = 0,
};

class Module;
class Executable final {
 public:
  explicit Executable(Module& module) : m_module(module) { m_stack.emplace(); }

  [[nodiscard]] static Executable* build(Module& module, const IRTree& ir_tree,
                                         ExeFlags flags = ExeFlags::NONE);

  [[nodiscard]] static Executable* build(Module& module, std::ostream& bytes,
                                         ExeFlags flags = ExeFlags::NONE);

 public:
  [[nodiscard]] auto flags() const { return m_flags; }
  [[nodiscard]] auto& constants() const { return m_constants; }
  [[nodiscard]] auto& bytecode() const { return m_bytecode; }
  [[nodiscard]] std::string to_string() const;

 private:
  [[nodiscard]] size_t pc() const { return m_bytecode.size() - 1; }
  [[nodiscard]] size_t constant_id() const { return m_constants.size() - 1; }
  [[nodiscard]] size_t label(size_t id);
  [[nodiscard]] size_t push(OpCode op, std::array<uint16_t, 3> ops = {});
  void push(ConstValue cvalue);
  void modify(size_t pc, OpCode op, std::array<uint16_t, 3> ops = {});
  void lower(const ir::Expr* expr, std::optional<uint16_t> dst);
  void lower(const ir::Stmt* stat);
  void lower(const ir::Term* term);
  void lower_jumps();

  template <derived_from<ir::Expr> Expr>
  void lower_expr(const Expr* expr, std::optional<uint16_t> dst) {
    VIA_PANIC(VIA_TYPENAME(Expr));
  }

  template <derived_from<ir::Stmt> Stat>
  void lower_stat(const Stat* stat) {
    VIA_PANIC(VIA_TYPENAME(Stat));
  }

  template <derived_from<ir::Term> Term>
  void lower_term(const Term* term) {
    VIA_PANIC(VIA_TYPENAME(Term));
  }

 private:
  Module& m_module;
  ExeFlags m_flags;
  RegisterState m_reg_state;
  StackState<BytecodeLocal> m_stack;
  std::vector<Instruction> m_bytecode;
  std::vector<ConstValue> m_constants;
  std::unordered_map<size_t, size_t> m_labels;
};

}  // namespace via
