/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include "control-flow.hpp"

#include <unordered_set>
#include <utility.hpp>

std::vector<const via::ir::Term*> via::get_control_paths(
    const ir::Stmt::Block* entry) {
  std::unordered_set<const ir::Stmt::Block*> visited;
  std::vector<const ir::Term*> terms;
  std::function<void(const ir::Stmt::Block*)> dfs =
      [&](const ir::Stmt::Block* block) {
        if (!block || !visited.insert(block).second) return;
        if VIA_TRY_COERCE (const ir::Term::Return, ret, block->term) {
          terms.push_back(ret);
        } else if VIA_TRY_COERCE (const ir::Term::Branch, br, block->term) {
          dfs(br->target);
        } else if VIA_TRY_COERCE (const ir::Term::CondBranch, cbr,
                                  block->term) {
          dfs(cbr->iftrue);
          dfs(cbr->iffalse);
        } else {
          VIA_PANIC("unmapped dfs block terminator");
        }
      };

  dfs(entry);
  return terms;
}
