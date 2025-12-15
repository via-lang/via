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
#include <compiler/parser.hpp>
#include <compiler/source-buffer.hpp>
#include <config.hpp>
#include <expected>
#include <filesystem>
#include <memory.hpp>
#include <optional>
#include <os/dl.hpp>
#include <span>
#include <string>
#include <vm/machine.hpp>
#include <vm/reference.hpp>

#include "binding.hpp"

#define VIA_MODULE_ENTRY_PREFIX viainit_

#define VIA_MODULE_ENTRY(ID, MANAGER)                            \
  extern "C" VIA_EXPORT via::NativeModuleInfo* EXPAND_AND_PASTE( \
      VIA_MODULE_ENTRY_PREFIX, ID)(via::ModuleManager * MANAGER)

#define VIA_MODULE_FUNCTION(ID, VM, CALL_INFO) \
  via::ValueRef ID(via::VirtualMachine* VM, via::CallInfo& CALL_INFO)

#define VIA_MODULE_LAMBDA(VM, CALL_INFO) \
  [](via::VirtualMachine * VM, via::CallInfo & CALL_INFO)->via::ValueRef

namespace via {
namespace config {

constexpr const char MODULE_ENTRY_PREFIX[] =
    EXPAND_STRING(VIA_MODULE_ENTRY_PREFIX);

}  // namespace config

class ModuleManager;
class NativeModuleInfo {
 public:
  explicit NativeModuleInfo(const Binding** buffer, size_t size)
      : m_defs(buffer, size) {}

  auto* operator->() { return &m_defs; }
  auto& operator*() { return m_defs; }

 public:
  auto unwrap() const { return m_defs; }

 private:
  std::span<const Binding*> m_defs;
};

using NativeModuleEntry = NativeModuleInfo* (*)(ModuleManager*);

enum class ModuleKind : uint8_t {
  SOURCE,
  NATIVE,
};

enum class ModulePerms : uint32_t {
  NONE = 0,
  FREAD = 1 << 0,
  FWRITE = 1 << 1,
  NETWORK = 1 << 2,
  FFICALL = 1 << 3,
  IMPORT = 1 << 4,
  ALL = 0xFFFFFFFF,
};

enum class ModuleFlags : uint32_t {
  NONE = 0,
  DUMP_TTREE = 1 << 0,
  DUMP_AST = 1 << 1,
  DUMP_IR = 1 << 2,
  DUMP_EXE = 1 << 3,
  DUMP_DEFTABLE = 1 << 4,
  NO_EXECUTION = 1 << 5,
  LAUNCH_DEBUGGER = 1 << 6,
  ALL = 0xFFFFFFFF,
};

class Module final {
 public:
  friend class ModuleManager;

 public:
  explicit Module(ModuleManager& manager, SourceBuffer&& source)
      : m_source(source), m_manager(manager) {}

  static std::expected<Module*, std::string> load_source_file(
      ModuleManager& manager, Module* importee, const char* name,
      const std::filesystem::path& path, const ast::StatImport* decl,
      const ModulePerms perms = ModulePerms::NONE,
      const ModuleFlags flags = ModuleFlags::NONE);

  static std::expected<Module*, std::string> load_native_object(
      ModuleManager& manager, Module* importee, const char* name,
      const std::filesystem::path& path, const ast::StatImport* decl,
      const ModulePerms perms = ModulePerms::NONE,
      const ModuleFlags flags = ModuleFlags::NONE);

 public:
  auto name() const { return m_name; }
  auto kind() const { return m_kind; }
  auto& source() const { return m_source; }
  auto& allocator() { return m_alloc; }
  auto& manager() const { return m_manager; }
  auto ast_decl() const { return m_ast_decl; }

  std::optional<const Binding*> lookup(SymbolId symbol);
  std::expected<Module*, std::string> import(const QualName& path,
                                             const ast::StatImport* ast_decl);

 protected:
  ScopedAllocator m_alloc;
  Logger& m_logger = Logger::stdout_logger();  // TODO: Modularize
  ModuleKind m_kind;
  ModulePerms m_perms;
  ModuleFlags m_flags;
  std::string m_name;
  SourceBuffer m_source;
  std::filesystem::path m_path;
  IRTree m_ir;
  Executable* m_exe;
  std::vector<Module*> m_imports;
  std::unordered_map<SymbolId, const Binding*> m_defs;
  Module* m_importee = nullptr;
  ModuleManager& m_manager;
  os::DynamicLibrary m_dl;
  const ast::StatImport* m_ast_decl = nullptr;
};

}  // namespace via
