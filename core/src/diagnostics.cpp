/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include "diagnostics.hpp"

#include <algorithm>
#include <cmath>

#include "ansi.hpp"
#include "logger.hpp"

std::string via::to_string(Note::Kind kind) {
  switch (kind) {
    case Note::HINT:
      return ansi::format("HINT", ansi::Foreground::GREEN,
                          ansi::Background::NONE, ansi::Style::BOLD);
    case Note::NOTE:
      return ansi::format("NOTE", ansi::Foreground::BLUE,
                          ansi::Background::NONE, ansi::Style::BOLD);
    case Note::SUGGESTION:
      return ansi::format("SUGGESTION", ansi::Foreground::MAGENTA,
                          ansi::Background::NONE, ansi::Style::BOLD);
  }
  VIA_PANIC();
}

bool via::Diagnostics::failed() const {
  return std::ranges::any_of(
      m_diags, [](const auto& diag) { return diag.level == Level::ERROR; });
}

void via::Diagnostics::emit(Logger& logger) const {
  for (const auto& diag : m_diags) {
    auto foreground = ansi::Foreground::NONE;
    LogLevel level;

    switch (diag.level) {
      case Level::INFO:
        level = LogLevel::INFO;
        foreground = ansi::Foreground::CYAN;
        break;
      case Level::WARNING:
        level = LogLevel::WARN;
        foreground = ansi::Foreground::YELLOW;
        break;
      case Level::ERROR:
        level = LogLevel::ERROR;
        foreground = ansi::Foreground::RED;
        break;
    }

    if (!m_source.is_valid_range(diag.location)) {
      logger.log(level, "{}", diag.message);
      return;
    }

    const char* begin = m_source.begin();
    const char* end = m_source.end();
    const char* ptr = begin + diag.location.begin;
    const char* line_begin = ptr;
    while (line_begin > begin && line_begin[-1] != '\n' &&
           line_begin[-1] != '\r') {
      --line_begin;
    }

    const char* line_end = ptr;
    while (line_end < end && *line_end != '\n' && *line_end != '\r') {
      ++line_end;
    }

    uint64_t line = 1 + std::count(begin, line_begin, '\n');
    uint64_t col = static_cast<uint64_t>(ptr - line_begin) + 1;

    std::string_view line_view(line_begin,
                               static_cast<size_t>(line_end - line_begin));

    logger.log(level, "{} {} {}", diag.message,
               ansi::format("at", ansi::Foreground::NONE,
                            ansi::Background::NONE, ansi::Style::FAINT),
               ansi::format(std::format("[{}:{}:{}]", m_path, line, col),
                            ansi::Foreground::CYAN));

    size_t span_begin = std::min(
        static_cast<size_t>(diag.location.begin - (line_begin - begin)),
        line_view.size());
    size_t span_end =
        std::min(static_cast<size_t>(diag.location.end - (line_begin - begin)),
                 line_view.size());

    std::string hl_line;
    if (span_begin < span_end) {
      hl_line.reserve(line_view.size() + 32);
      hl_line.append(line_view.substr(0, span_begin));
      hl_line.append(ansi::format(
          std::string(line_view.substr(span_begin, span_end - span_begin)),
          foreground, ansi::Background::NONE, ansi::Style::BOLD));
      hl_line.append(line_view.substr(span_end));
    } else {
      hl_line = std::string(line_view);
    }

    size_t line_width = static_cast<size_t>(std::log10(line)) + 1;
    logger.log(LogLevel::NONE, " {} | {}", line, hl_line);

    std::string caret(line_view.size(), ' ');
    if (span_begin < span_end) {
      std::fill(caret.begin() + span_begin, caret.begin() + span_end, '^');
    } else if (col > 0 && col - 1 < caret.size()) {
      caret[col - 1] = '^';
    }

    caret.erase(std::find_if(caret.rbegin(), caret.rend(),
                             [](unsigned char ch) { return !std::isspace(ch); })
                    .base(),
                caret.end());

    logger.log(
        LogLevel::NONE, " {0} | {1}{2}\n {0} |", std::string(line_width, ' '),
        ansi::format(caret, foreground, ansi::Background::NONE,
                     ansi::Style::BOLD),
        diag.note.valid ? std::format("-=[{}]: {}", to_string(diag.note.kind),
                                      diag.note.msg)
                        : "");
  }
}
