/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include <argvx/parser.hpp>
#include <via/via.hpp>

#include "options.hpp"
#include "tools/utility.hpp"

static void translate_module_flags(via::ModuleFlags& flags,
                                   const via::cli::ProgramOptions& options) {
  using enum via::ModuleFlags;
  flags = (via::ModuleFlags)0;

  if (options.no_execute) flags |= NO_EXECUTION;
  if (options.debugger) flags |= LAUNCH_DEBUGGER;

  constexpr std::pair<std::string_view, via::ModuleFlags> dump_flags[] = {
      {"token-tree", DUMP_TTREE},
      {"ast", DUMP_AST},
      {"ir", DUMP_IR},
      {"executable", DUMP_EXE},
      {"def-table", DUMP_DEFTABLE},
  };

  for (auto&& [name, flag] : dump_flags)
    if (options.dump_all || options.dump.contains(std::string(name)))
      flags |= flag;
}

int main(int argc, char* argv[]) {
  using namespace via::cli;
  using namespace via::operators;

  ProgramOptions options;

  via::Logger& logger = via::Logger::stdout_logger();
  via::ModuleFlags flags = via::ModuleFlags::NONE;

  argvx::parser<> parser(argc, argv);
  parser.positional("input", options.input).required();
  parser.option({"--debugger", {}}, options.debugger);
  parser.option({"--no-execute", {}}, options.no_execute);
  parser.option({"--i-am-stupid", {}}, options.supress_missing_core_warning);
  parser.option({"--dump-all", {}}, options.dump_all);

  if (auto error = parser.parse()) {
    logger.error("{}", *error);
    return 1;
  }

  translate_module_flags(flags, options);

  via::init(options.verbosity);

  if (options.verbosity > 0) {
    std::cout << options.to_string();
  }

  via::ModuleManager manager;
  manager.push_import_path(options.input.parent_path());

  static auto lang_dir = via::get_lang_dir();

  if (std::filesystem::exists(lang_dir)) {
    manager.push_import_path(lang_dir / "lib");
  } else if (!options.supress_missing_core_warning) {
    logger.warn(
        "could not find language core directory, core libraries and tooling {} "
        "work "
        "as intended!",
        via::ansi::bold("WILL NOT"));
    logger.info(
        "pass in the `--i-am-stupid` flag to supress this warning "
        "(only if you are stupid or hacking the binary)");
  }

  for (const auto& path : options.imports) {
    manager.push_import_path(path);
  }

  // Instantiate root module
  auto module = via::Module::load_source_file(
      manager, nullptr, options.input.stem().c_str(), options.input, nullptr,
      via::ModulePerms::ALL, flags);

  if (!module.has_value()) {
    logger.error("{}", module.error());
    return 1;
  }

  if (options.dump_all || options.dump.contains("symbol-table")) {
    std::cout << manager.symbol_table().to_string();
  } else if (options.dump_all || options.dump.contains("import-dirs")) {
    std::cout << "(global) ";
    std::cout << via::ansi::format(
        "[import directories]:\n", via::ansi::Foreground::YELLOW,
        via::ansi::Background::NONE, via::ansi::Style::UNDERLINE);

    for (const auto& path : manager.get_import_paths()) {
      std::println(std::cout, "  {}", path.string());
    }
  }
  return 0;
}
