/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include "vm/value.hpp"

#include <catch2/catch_all.hpp>
#include <via/via.hpp>

#include "fixture.hpp"

using via::none;
using via::Value;
using enum via::ValueKind;

TEST_CASE_METHOD(VMFixture, "none value has no payload", "[value][none]") {
  REQUIRE(none.kind() == NONE);
  REQUIRE(none.to_string() == "none");

  REQUIRE(none.as<BOOL>() == false);
  REQUIRE(none.as<STRING>() == "none");

  REQUIRE(none.as_c<BOOL>() == false);
  REQUIRE(!none.as_c<INT>());
  REQUIRE(!none.as_c<FLOAT>());
  REQUIRE(none.as_c<STRING>() == "none");

  REQUIRE(none.clone() == none);
}

TEST_CASE_METHOD(VMFixture, "bool value has boolean payload", "[value][bool]") {
  auto value = Value(false);

  REQUIRE(value.kind() == BOOL);
  REQUIRE(value.to_string() == "false");

  REQUIRE(value.as<INT>() == 0);
  REQUIRE(value.as<STRING>() == "none");

  REQUIRE(!value.as_c<BOOL>());
  REQUIRE(value.as_c<INT>() == 1);
  REQUIRE(!value.as_c<FLOAT>());
  REQUIRE(value.as_c<STRING>() == "false");

  REQUIRE(value.clone() == false);
}
