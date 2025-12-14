/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#pragma once

#include <via/config.hpp>

#include "lexer/source_buffer.hpp"
#include "logger.hpp"
#include "support/utility.hpp"

namespace via {

enum class Level : uint8_t {
  INFO,
  WARNING,
  ERROR,
};

struct Note {
  enum Kind {
    NOTE,
    HINT,
    SUGGESTION,
  };

  const Kind kind = Kind::NOTE;
  const bool valid = false;
  const std::string msg;

  Note() = default;
  Note(Kind kind, std::string msg) : kind(kind), valid(true), msg(msg) {}
};

struct Diagnosis {
  const Level level;
  const SourceLoc location;   // Absolute location in the source buffer
  const std::string message;  // Human-readable message
  const Note note;
};

class Diagnostics final {
 public:
  explicit Diagnostics(std::string path, std::string name,
                       const SourceBuffer& source)
      : m_path(path), m_name(name), m_source(source) {}

  NO_COPY(Diagnostics);

 public:
  auto& collect() { return m_diags; }
  auto& source() const { return m_source; }
  void clear() noexcept { m_diags.clear(); }
  bool failed() const;
  void emit(Logger& logger = Logger::stdout_logger()) const;
  void report(Diagnosis diag) { m_diags.push_back(diag); }

  // clang-format off
    template <Level L>
    void report(SourceLoc loc, std::string msg, Note note = {})
        { m_diags.emplace_back(L, loc, msg, note); }
  // clang-format on

 private:
  std::string m_path, m_name;
  const SourceBuffer& m_source;
  std::vector<Diagnosis> m_diags;
};

std::string to_string(Note::Kind kind) noexcept;

}  // namespace via
