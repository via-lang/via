/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include <cmath>
#include <via/via.hpp>

VIA_MODULE_ENTRY(math, manager) {
  via::ModuleBuilder b(*manager);

  b.function("sin")
      .returns(b.float_t())
      .parameter(b.float_t())
      .implement(VIA_MODULE_LAMBDA(vm, ci) {
        auto x = ci.args.at(0);
        auto r = std::sin(x->unwrap<via::FLOAT>());
        return via::ValueRef(vm, r);
      });

  return b.build();
}
