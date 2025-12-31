/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include "initialize.hpp"

#include <mimalloc.h>

#include <format>
#include <utility.hpp>

#ifdef NDEBUG
#define DEBUG 0
#else
#define DEBUG 1
#endif

static void mimalloc_error_handler(int err, void* arg) {
  VIA_PANIC(std::format("mimalloc: {}", err));
}

static void init_mimalloc(uint8_t verbosity) {
  mi_option_set(mi_option_reserve_os_memory, 0x8000000ULL);

  mi_option_set(mi_option_large_os_pages, 0);
  mi_option_set(mi_option_reserve_huge_os_pages, 0);
  mi_option_set(mi_option_reserve_huge_os_pages_at, -1);  // any NUMA node

  mi_option_set(mi_option_eager_commit, 0);
  mi_option_set(mi_option_eager_commit_delay,
                4);  // commit lazily in 4-page steps

  mi_option_set(mi_option_reset_delay, 0);  // Disable page reset delay
  mi_option_set(mi_option_show_errors, DEBUG);
  mi_option_set(mi_option_show_stats, verbosity > 1);
  mi_option_set(mi_option_verbose, verbosity > 2);

  mi_register_error(mimalloc_error_handler, nullptr);

  if (verbosity > 1) {
    mi_stats_print(nullptr);
  }
}

static void trap_call() {
  static bool called = false;
  VIA_ASSERT(!called, "via::init() called twice");
  called = true;
}

void via::init(uint8_t verbosity) {
  trap_call();
  init_mimalloc(verbosity);
}
