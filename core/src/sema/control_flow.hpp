/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#pragma once

#include <vector>
#include <via/config.hpp>

#include "ir/tree.hpp"

namespace via {

std::vector<const ir::Term*> get_control_paths(
    const ir::StatBlock* entry) noexcept;

}  // namespace via
