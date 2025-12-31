/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include <catch2/catch_all.hpp>
#include <via/via.hpp>

TEST_CASE("stack push", "[stack][push]") {
  via::ScopedAllocator alloc;
  via::Stack<uintptr_t> stack(alloc);
  stack.push(0xDEADBEEF);
  stack.push(0xCAFEBABE);

  REQUIRE(stack.at(0) == 0xDEADBEEF);
  REQUIRE(stack.at(1) == 0xCAFEBABE);
  REQUIRE(stack.top() == 0xCAFEBABE);
}

TEST_CASE("stack pop", "[stack][pop]") {
  via::ScopedAllocator alloc;
  via::Stack<uintptr_t> stack(alloc);
  stack.push(0xDEADBEEF);
  stack.push(0xCAFEBABE);
  stack.pop();

  REQUIRE(stack.at(0) == 0xDEADBEEF);
  REQUIRE(stack.top() == 0xDEADBEEF);
}

TEST_CASE("stack size", "[stack][size]") {
  via::ScopedAllocator alloc;
  via::Stack<uintptr_t> stack(alloc);
  stack.push(0xDEADBEEF);
  stack.push(0xCAFEBABE);

  REQUIRE(stack.size() == 2);
}
