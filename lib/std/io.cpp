/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include <iostream>
#include <via/via.hpp>

#include "module/module.hpp"
#include "vm/closure.hpp"
#include "vm/machine.hpp"

VIA_MODULE_ENTRY(io, manager) {
  via::ModuleBuilder b(*manager);

  b.function("print")
      .returns(b.nil_t())
      .parameter(b.string_t())
      .implement(VIA_MODULE_LAMBDA(vm, ci) {
        auto str = ci.args.at(0);
        std::cout << str->unwrap<via::STRING>();
        return via::ValueRef(vm);
      });

  b.function("printn")
      .returns(b.nil_t())
      .parameter(b.string_t())
      .implement(VIA_MODULE_LAMBDA(vm, ci) {
        auto str = ci.args.at(0);
        std::cout << str->unwrap<via::STRING>() << "\n";
        return via::ValueRef(vm);
      });

  b.function("input")
      .returns(b.string_t())
      .parameter(b.string_t())
      .implement(VIA_MODULE_LAMBDA(vm, ci) {
        auto& alloc = vm->allocator();
        auto str = ci.args.at(0);

        std::string in;
        std::cout << str->unwrap<via::STRING>();
        std::cin >> in;

        char* cstr = alloc.strdup(in.c_str());
        return via::ValueRef(vm, cstr);
      });

  return b.build();
}
