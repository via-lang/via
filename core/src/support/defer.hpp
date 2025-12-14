/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#pragma once

#include <functional>

#define defer ::via::detail::BindingerImpl _ = [&]

namespace via {
namespace detail {

using BindingerCallback = std::function<void()>;

class BindingerImpl final {
 public:
  ~BindingerImpl() { m_callback(); }
  BindingerImpl(BindingerCallback callback) : m_callback(std::move(callback)) {}

 private:
  BindingerCallback m_callback;
};

}  // namespace detail
}  // namespace via
