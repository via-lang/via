/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#pragma once

#include <bitset>
#include <config.hpp>
#include <cstddef>
#include <diagnostics.hpp>
#include <limits>

namespace via {
namespace config {

constexpr size_t REGISTER_COUNT = std::numeric_limits<uint16_t>::max();

}

class RegisterState {
 public:
  inline uint16_t alloc() {
    for (size_t i = 0; i < config::REGISTER_COUNT; ++i) {
      if (!m_buffer.test(i)) {  // free register
        m_buffer.set(i);        // mark as occupied
        return static_cast<uint16_t>(i);
      }
    }
    return 0;
  }

  inline void free(uint16_t reg) {
    VIA_DEBUG_ASSERT(reg <= config::REGISTER_COUNT,
                     "free() called on invalid semantic register");
    m_buffer.reset(reg);  // mark as free
  }

  template <typename... Regs>
  inline void free_all(Regs... regs) {
    (free(regs), ...);
  }

 private:
  std::bitset<config::REGISTER_COUNT> m_buffer;
};

}  // namespace via
