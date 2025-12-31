/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#pragma once

#include <via/via.hpp>

struct VMFixture {
  via::ModuleManager manager;
  via::Module module;
  via::Executable exe;
  via::VirtualMachine vm;

  VMFixture() : module(manager), exe(module), vm(module, exe) {}
};
