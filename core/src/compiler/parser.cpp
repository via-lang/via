/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include "parser.hpp"

#include <diagnostics.hpp>

#include "syntax-tree.hpp"
#include "token.hpp"
#include "type.hpp"

#define SAVE()          \
  auto* first = peek(); \
  auto first_loc = m_source.location(*first);

#define SAVE_AND_ADVANCE() \
  auto* first = advance(); \
  auto first_loc = m_source.location(*first);

using enum via::TokenKind;

struct ParserError {
 public:
  via::Diagnosis diag;

  template <typename... Args>
    requires std::is_constructible_v<via::Diagnosis, via::Level, Args...>
  explicit ParserError(Args&&... args) : diag(via::Level::ERROR, args...) {}
};

static bool is_expr_start(via::TokenKind kind) {
  switch (kind) {
    case IDENTIFIER:
    case LIT_INT:
    case LIT_BINT:
    case LIT_XINT:
    case KW_NONE:
    case LIT_FLOAT:
    case LIT_STRING:
    case OP_BANG:
    case KW_FN:
    case PAREN_OPEN:
    case OP_MINUS:
    case OP_TILDE:
    case OP_AMP:
      return true;
    default:
      return false;
  }
}

static int bin_prec(via::TokenKind kind) {
  switch (kind) {
    case OP_PIPE_PIPE:
      return 0;
    case OP_AMP_AMP:
      return 1;
    case OP_EQ_EQ:
    case OP_BANG_EQ:
    case OP_LT:
    case OP_LT_EQ:
    case OP_GT:
    case OP_GT_EQ:
      return 2;
    case OP_AMP:
      return 3;
    case OP_CARET:
      return 4;
    case OP_PIPE:
      return 5;
    case OP_SHL:
    case OP_SHR:
      return 6;
    case OP_PLUS:
    case OP_MINUS:
      return 7;
    case OP_STAR:
    case OP_SLASH:
    case OP_PERCENT:
      return 8;
    case OP_STAR_STAR:
      return 9;
    default:
      return -1;
  }
}

const via::Token* via::Parser::peek(int ahead) { return m_cursor[ahead]; }
const via::Token* via::Parser::advance() { return *(m_cursor++); }
bool via::Parser::match(TokenKind kind, int ahead) {
  return peek(ahead)->kind == kind;
}

bool via::Parser::optional(TokenKind kind) {
  if (match(kind)) {
    advance();
    return true;
  }
  return false;
}

const via::Token* via::Parser::expect(TokenKind kind, const char* task) {
  if (!match(kind)) {
    const Token& unexp = *peek();
    throw ParserError(
        m_source.location(unexp),
        std::format("unexpected token '{}' ({}) while {}", unexp.to_string(),
                    to_string(unexp.kind), task));
  }
  return advance();
}

const via::ast::Path* via::Parser::parse_static_path() {
  auto* sp = m_alloc.emplace<ast::Path>();

  while (true) {
    sp->path.push_back(expect(IDENTIFIER, "parsing static path"));

    if (match(COLON_COLON)) {
      advance();
    } else {
      break;
    }
  }

  sp->loc = {m_source.location(*sp->path.front()).begin,
             m_source.location(*sp->path.front()).end};
  return sp;
}

const via::ast::Expr* via::Parser::parse_lvalue() {
  const ast::Expr* expr = parse_expr();
  if (is_lvalue(expr)) {
    return expr;
  } else {
    throw ParserError(expr->loc, "unexpected expression while parsing lvalue");
  }
}

const via::ast::Parameter* via::Parser::parse_parameter() {
  SAVE_AND_ADVANCE();

  auto* par = m_alloc.emplace<ast::Parameter>();
  par->symbol = first;

  if (optional(COLON)) {
    par->type = parse_type();
    par->loc = {first_loc.begin, par->type->loc.end};
  } else {
    par->loc = first_loc;
  }
  return par;
}

const via::ast::Scope* via::Parser::parse_scope() {
  SAVE_AND_ADVANCE();

  auto scope = m_alloc.emplace<ast::Scope>();

  if (first->kind == COLON) {
    scope->stats.push_back(parse_stat());
    scope->loc = {first_loc.begin, scope->stats.back()->loc.end};
  } else if (first->kind == BRACE_OPEN) {
    while (!match(BRACE_CLOSE)) {
      scope->stats.push_back(parse_stat());
    }

    auto* last = advance();
    scope->loc = {
        m_source.location(*first).begin,
        m_source.location(*last).end,
    };
  } else {
    throw ParserError(first_loc,
                      std::format("unexpected token '{}' while parsing scope",
                                  first->to_string()),
                      Note(Note::HINT, "Expected ':' | '{'"));
  }
  return scope;
}

const via::ast::ExprLiteral* via::Parser::parse_expr_literal() {
  auto* lit = m_alloc.emplace<ast::ExprLiteral>();
  lit->tok = advance();
  lit->loc = m_source.location(*lit->tok);
  return lit;
}

const via::ast::ExprSymbol* via::Parser::parse_expr_symbol() {
  auto* symbol = m_alloc.emplace<ast::ExprSymbol>();
  symbol->symbol = advance();
  symbol->loc = m_source.location(*symbol->symbol);
  return symbol;
}

const via::ast::Expr* via::Parser::parse_expr_group_or_tuple() {
  auto first_loc = m_source.location(*advance());
  auto* first = parse_expr();

  if (match(COMMA)) {
    std::vector<const ast::Expr*> vals;
    vals.push_back(first);

    while (match(COMMA)) {
      advance();
      vals.push_back(parse_expr());
    }

    expect(PAREN_CLOSE, "parsing tuple expression");

    auto* tup = m_alloc.emplace<ast::ExprTuple>();
    tup->values = std::move(vals);
    tup->loc = {first_loc.begin, m_source.location(*peek(-1)).end};
    return reinterpret_cast<const ast::Expr*>(tup);
  }

  expect(PAREN_CLOSE, "parsing grouping expression");

  auto* group = m_alloc.emplace<ast::ExprGroup>();
  group->expr = first;
  group->loc = {first_loc.begin, m_source.location(*peek(-1)).end};
  return reinterpret_cast<const ast::Expr*>(group);
}

const via::ast::ExprUnary* via::Parser::parse_expr_unary(
    const ast::Expr* expr) {
  auto* un = m_alloc.emplace<ast::ExprUnary>();
  un->op = advance();
  un->expr = parse_expr_affix();
  un->loc = {m_source.location(*un->op).begin, un->expr->loc.end};
  return un;
}

const via::ast::ExprDynAccess* via::Parser::parse_expr_dyn_access(
    const ast::Expr* expr) {
  advance();  // consume '.'

  auto* da = m_alloc.emplace<ast::ExprDynAccess>();
  da->root = expr;
  da->index = expect(IDENTIFIER, "parsing dynamic access specifier");
  da->loc = {da->root->loc.begin, m_source.location(*da->index).end};
  return da;
}

const via::ast::ExprStaticAccess* via::Parser::parse_expr_st_access(
    const ast::Expr* expr) {
  advance();  // consume '::'

  auto* sa = m_alloc.emplace<ast::ExprStaticAccess>();
  sa->root = expr;
  sa->index = expect(IDENTIFIER, "parsing static access specifier");
  sa->loc = {sa->root->loc.begin, m_source.location(*sa->index).end};
  return sa;
}

const via::ast::ExprCall* via::Parser::parse_expr_call(const ast::Expr* expr) {
  advance();  // consume '('

  std::vector<const ast::Expr*> args;

  if (!match(PAREN_CLOSE)) {
    do args.push_back(parse_expr());
    while (match(COMMA) && advance());

    expect(PAREN_CLOSE, "parsing function call");
  } else {
    advance();  // consume ')'
  }

  auto* call = m_alloc.emplace<ast::ExprCall>();
  call->callee = expr;
  call->args = std::move(args);
  call->loc = {expr->loc.begin, m_source.location(*peek(-1)).end};
  return call;
}

const via::ast::ExprSubscript* via::Parser::parse_expr_subscript(
    const ast::Expr* expr) {
  advance();  // consume '['

  auto* idx = parse_expr();

  expect(BRACKET_CLOSE, "parsing subscript expression");

  auto* subs = m_alloc.emplace<ast::ExprSubscript>();
  subs->lhs = expr;
  subs->rhs = idx;
  subs->loc = {expr->loc.begin, m_source.location(*peek(-1)).end};
  return subs;
}

const via::ast::ExprCast* via::Parser::parse_expr_cast(const ast::Expr* expr) {
  advance();

  auto* cast = m_alloc.emplace<ast::ExprCast>();
  cast->expr = expr;
  cast->type = parse_type();
  cast->loc = {expr->loc.begin, cast->type->loc.end};
  return cast;
}

const via::ast::ExprTernary* via::Parser::parse_expr_ternary(
    const ast::Expr* expr) {
  advance();

  auto* tern = m_alloc.emplace<ast::ExprTernary>();
  tern->lhs = expr;
  tern->cond = parse_expr();

  expect(KW_ELSE, "parsing ternary expression");

  tern->rhs = parse_expr();
  tern->loc = {expr->loc.begin, tern->rhs->loc.end};
  return tern;
}

const via::ast::ExprArray* via::Parser::parse_expr_array() {
  auto first_loc = m_source.location(*peek());
  auto* arr = m_alloc.emplace<ast::ExprArray>();

  if (!match(BRACKET_CLOSE)) {
    while (true) {
      arr->values.push_back(parse_expr());

      if (match(BRACKET_CLOSE)) {
        optional(COMMA);  // trailing comma
        break;
      } else {
        expect(COMMA, "parsing array initializer");
      }
    }
  }

  auto* last = expect(BRACKET_CLOSE, "terminating array initializer");
  arr->loc = {first_loc.begin, m_source.location(*last).end};
  return arr;
}

const via::ast::ExprLambda* via::Parser::parse_expr_lambda() {
  auto first_loc = m_source.location(*peek());
  auto* fn = m_alloc.emplace<ast::ExprLambda>();

  expect(PAREN_OPEN, "parsing lambda parameter list");

  if (!match(PAREN_CLOSE)) {
    while (true) {
      fn->parms.push_back(parse_parameter());

      if (match(PAREN_CLOSE)) {
        break;
      } else {
        expect(COMMA, "parsing lambda parameter list");
      }
    }

    expect(PAREN_CLOSE, "terminating lambda parameter list");
  }

  fn->body = parse_scope();
  fn->loc = {first_loc.begin, fn->body->loc.end};
  return fn;
}

const via::ast::Expr* via::Parser::parse_expr_primary() {
  SAVE();

  switch (first->kind) {
    // Literal expression
    case LIT_INT:
    case LIT_BINT:
    case LIT_XINT:
    case KW_NONE:
    case LIT_FLOAT:
    case KW_TRUE:
    case KW_FALSE:
    case LIT_STRING:
      return (const ast::Expr*)parse_expr_literal();
    case IDENTIFIER:
      return (const ast::Expr*)parse_expr_symbol();
    case PAREN_OPEN:
      return parse_expr_group_or_tuple();
    case BRACKET_OPEN:
      return (const ast::Expr*)parse_expr_array();
    case KW_FN:
      return (const ast::Expr*)parse_expr_lambda();
    default:
      throw ParserError(
          first_loc,
          std::format(
              "unexpected token '{}' ({}) while parsing primary expression",
              first->to_string(), to_string(first->kind)),
          Note(Note::HINT,
               "Expected INT | BINARY_INT | HEX_INT | 'nil' | FLOAT | 'true' "
               "| 'false' | "
               "STRING | IDENTIFIER | '(' | ')' | 'fn'"));
  }
}

const via::ast::Expr* via::Parser::parse_expr_affix() {
  const ast::Expr* expr = nullptr;

  switch (peek()->kind) {
    case OP_BANG:
    case OP_MINUS:
    case OP_TILDE:
    case OP_AMP:
      expr = (const ast::Expr*)parse_expr_unary(expr);
      break;
    default:
      expr = parse_expr_primary();
      break;
  }

  while (true) {
    switch (peek()->kind) {
      case KW_AS:
        expr = (const ast::Expr*)parse_expr_cast(expr);
        break;
      case KW_IF:
        expr = (const ast::Expr*)parse_expr_ternary(expr);
        break;
      case PAREN_OPEN:
        expr = (const ast::Expr*)parse_expr_call(expr);
        break;
      case BRACKET_OPEN:
        expr = (const ast::Expr*)parse_expr_subscript(expr);
        break;
      case PERIOD:
        expr = (const ast::Expr*)parse_expr_dyn_access(expr);
        break;
      case COLON_COLON:
        expr = (const ast::Expr*)parse_expr_st_access(expr);
        break;
      default:
        return expr;
    }
  }
}

const via::ast::Expr* via::Parser::parse_expr(int min_prec) {
  int prec;
  auto* lhs = parse_expr_affix();

  while ((prec = bin_prec(peek()->kind), prec >= min_prec)) {
    auto bin = m_alloc.emplace<ast::ExprBinary>();
    bin->op = advance();
    bin->lhs = lhs;
    bin->rhs = parse_expr(prec + 1);
    bin->loc = {lhs->loc.begin, bin->rhs->loc.end};
    lhs = (const ast::Expr*)bin;
  }
  return lhs;
}

const via::ast::TypeBuiltin* via::Parser::parse_type_builtin() {
  SAVE_AND_ADVANCE();

  auto* bt = m_alloc.emplace<ast::TypeBuiltin>();
  bt->token = first;
  bt->loc = first_loc;
  return bt;
}

const via::ast::TypeArray* via::Parser::parse_type_array() {
  SAVE_AND_ADVANCE();

  auto* at = m_alloc.emplace<ast::TypeArray>();
  at->type = parse_type();
  auto* end = expect(BRACKET_CLOSE, "terminating array type");
  at->loc = {first_loc.begin, m_source.location(*end).end};
  return at;
}

const via::ast::TypeMap* via::Parser::parse_type_map() {
  SAVE_AND_ADVANCE();

  auto* dt = m_alloc.emplace<ast::TypeMap>();
  dt->key = parse_type();
  expect(COLON, "parsing map type");
  dt->value = parse_type();

  auto* end = expect(BRACE_CLOSE, "terminating map type");
  dt->loc = {m_source.location(*first).begin, m_source.location(*end).end};
  return dt;
}

const via::ast::TypeFunc* via::Parser::parse_type_function() {
  SAVE_AND_ADVANCE();
  expect(PAREN_OPEN, "parsing function type parameter list");

  auto* fn = m_alloc.emplace<ast::TypeFunc>();

  while (!match(PAREN_CLOSE)) {
    fn->parms.push_back(parse_parameter());
    expect(COMMA, "terminating function type parameter");
  }

  expect(ARROW, "parsing function type return type");

  fn->ret = parse_type();
  fn->loc = {first_loc.begin, fn->ret->loc.end};
  return fn;
}

const via::ast::Type* via::Parser::parse_type_primary() {
  auto* tok = peek();
  switch (tok->kind) {
    case KW_NONE:
    case KW_BOOL:
    case KW_INT:
    case KW_FLOAT:
    case KW_STRING:
      return (const ast::Type*)parse_type_builtin();
    case BRACKET_OPEN:
      return (const ast::Type*)parse_type_array();
    case BRACE_OPEN:
      return (const ast::Type*)parse_type_map();
    case KW_FN:
      return (const ast::Type*)parse_type_function();
    default:
      throw ParserError(
          m_source.location(*tok),
          std::format("unexpected token '{}' ({}) while parsing type",
                      tok->to_string(), to_string(tok->kind)),
          Note(Note::HINT,
               "expected 'nil' | 'bool' | 'int' | 'float' | "
               "'string' | '[' | '{' | 'fn'"));
  }
}

const via::ast::Type* via::Parser::parse_type() {
  SAVE();

  auto quals = TypeQualifier::NONE;

  while (true) {
    auto* tok = peek();
    switch (tok->kind) {
      case KW_CONST:
        if (quals & TypeQualifier::CONST)
          m_diags.report<Level::WARNING>(
              m_source.location(*tok),
              "duplicate 'const' qualifier will be ignored",
              Note(Note::SUGGESTION, "remove 'const'"));
        quals |= TypeQualifier::CONST;
        advance();
        break;
      case KW_STRONG:
        if (quals & TypeQualifier::STRONG)
          m_diags.report<Level::WARNING>(
              m_source.location(*tok),
              "duplicate 'strong' qualifier will be ignored",
              Note(Note::SUGGESTION, "remove 'strong'"));
        quals |= TypeQualifier::STRONG;
        advance();
        break;
      case OP_AMP:
        if (quals & TypeQualifier::REFERENCE)
          throw ParserError(m_source.location(*tok),
                            "nested reference qualifier not allowed",
                            Note(Note::SUGGESTION, "remove '&'"));
        quals |= TypeQualifier::REFERENCE;
        advance();
        break;
      default:
        goto done;
    }
  }

done:
  auto* primary = const_cast<ast::Type*>(parse_type_primary());
  primary->loc = {first_loc.begin, primary->loc.end};
  primary->quals = quals;
  return primary;
}

const via::ast::StatVarDecl* via::Parser::parse_stat_var_decl(bool semicolon) {
  SAVE_AND_ADVANCE();

  auto vars = m_alloc.emplace<ast::StatVarDecl>();
  vars->decl = first;
  vars->lval = parse_lvalue();

  if (optional(COLON)) {
    vars->type = parse_type();
  } else {
    vars->type = nullptr;
  }

  expect(OP_EQ, "parsing variable declaration");

  vars->rval = parse_expr();
  vars->loc = {first_loc.begin, vars->rval->loc.end};

  if (semicolon) {
    optional(SEMICOLON);
  }

  return vars;
}

const via::ast::StatFor* via::Parser::parse_stat_for() {
  SAVE_AND_ADVANCE();

  auto fors = m_alloc.emplace<ast::StatFor>();
  fors->init = parse_stat_var_decl(false);

  if (fors->init->decl->kind == KW_CONST) {
    throw ParserError(m_source.location(*fors->init->decl),
                      "'const' variable not allowed in ranged for loop");
  }

  expect(COMMA, "parsing ranged for loop");

  fors->target = parse_expr();

  if (match(COMMA)) {
    advance();
    fors->step = parse_expr();
  }

  fors->body = parse_scope();
  fors->loc = {first_loc.begin, fors->body->loc.end};
  return fors;
}

const via::ast::StatForEach* via::Parser::parse_stat_for_each() {
  SAVE_AND_ADVANCE();

  auto fors = m_alloc.emplace<ast::StatForEach>();
  fors->name = parse_lvalue();

  expect(KW_IN, "parsing for each statement");

  fors->expr = parse_expr();
  fors->body = parse_scope();
  fors->loc = {first_loc.begin, fors->body->loc.end};
  return fors;
}

const via::ast::StatIf* via::Parser::parse_stat_if() {
  using Branch = ast::StatIf::Branch;

  SAVE_AND_ADVANCE();

  Branch br;
  br.cond = parse_expr();
  br.body = parse_scope();

  auto* ifs = m_alloc.emplace<ast::StatIf>();
  ifs->branches.push_back(br);

  while (match(KW_ELSE)) {
    advance();

    Branch br;

    if (match(KW_IF)) {
      advance();
      br.cond = parse_expr();
    } else {
      br.cond = nullptr;
    }

    br.body = parse_scope();
    ifs->branches.push_back(br);
  }

  ifs->loc = {first_loc.begin, br.body->loc.end};
  return ifs;
}

const via::ast::StatWhile* via::Parser::parse_stat_while() {
  SAVE_AND_ADVANCE();

  auto* whs = m_alloc.emplace<ast::StatWhile>();
  whs->cond = parse_expr();
  whs->body = parse_scope();
  whs->loc = {first_loc.begin, whs->body->loc.end};
  return whs;
}

const via::ast::StatAssign* via::Parser::parse_stat_assign(
    const ast::Expr* expr) {
  auto as = m_alloc.emplace<ast::StatAssign>();
  as->lval = expr;
  as->op = advance();
  as->rval = parse_expr();
  as->loc = {as->lval->loc.begin, as->rval->loc.end};
  optional(SEMICOLON);
  return as;
}

const via::ast::StatReturn* via::Parser::parse_stat_return() {
  SAVE_AND_ADVANCE();

  auto* ret = m_alloc.emplace<ast::StatReturn>();

  if (is_expr_start(peek()->kind)) {
    ret->expr = parse_expr();
    ret->loc = {first_loc.begin, ret->expr->loc.end};
  } else {
    ret->expr = nullptr;
    ret->loc = first_loc;
  }

  optional(SEMICOLON);
  return ret;
}

const via::ast::StatEnum* via::Parser::parse_stat_enum() {
  SAVE_AND_ADVANCE();

  auto ens = m_alloc.emplace<ast::StatEnum>();
  ens->symbol = advance();

  if (optional(KW_OF)) {
    ens->type = parse_type();
  }

  expect(BRACE_OPEN, "parsing enumerator list");

  while (!match(BRACE_CLOSE)) {
    auto* symbol = advance();
    expect(OP_EQ, "parsing enumerator pair");

    ens->pairs.push_back({
        .symbol = symbol,
        .expr = parse_expr(),
    });

    expect(COMMA, "parsing enumerator pair");
  }

  ens->loc = {first_loc.begin, m_source.location(*advance()).end};
  return ens;
}

const via::ast::StatImport* via::Parser::parse_stat_import() {
  SAVE_AND_ADVANCE();

  size_t end;
  auto imp = m_alloc.emplace<ast::StatImport>();

  while (true) {
    auto* tok = expect(IDENTIFIER, "parsing import path");
    imp->path.push_back(tok);

    if (match(COLON_COLON)) {
      advance();
    } else {
      end = m_source.location(*tok).end;
      break;
    }
  }

  imp->loc = {first_loc.begin, end};
  optional(SEMICOLON);
  return imp;
}

const via::ast::StatFunctionDecl* via::Parser::parse_stat_func_decl() {
  SAVE_AND_ADVANCE();

  auto* fn = m_alloc.emplace<ast::StatFunctionDecl>();
  fn->name = expect(IDENTIFIER, "parsing function name");

  expect(PAREN_OPEN, "parsing function parameter list");

  while (!match(PAREN_CLOSE)) {
    fn->parms.push_back(parse_parameter());

    if (match(PAREN_CLOSE)) {
      optional(COMMA);
      break;
    } else {
      expect(COMMA, "terminating function parameter");
    }
  }

  expect(PAREN_CLOSE, "terminating function parameter list");

  if (optional(ARROW)) {
    fn->ret = parse_type();
  } else {
    fn->ret = nullptr;
  }

  fn->body = parse_scope();
  fn->loc = {first_loc.begin, fn->body->loc.end};
  return fn;
}

const via::ast::StatStructDecl* via::Parser::parse_stat_struct_decl() {
  SAVE_AND_ADVANCE();

  auto* strc = m_alloc.emplace<ast::StatStructDecl>();
  strc->name = expect(IDENTIFIER, "parsing struct name");
  strc->body = parse_scope();
  strc->loc = {first_loc.begin, strc->loc.end};
  return strc;
}

const via::ast::StatTypeDecl* via::Parser::parse_stat_type_decl() {
  SAVE_AND_ADVANCE();

  auto* ty = m_alloc.emplace<ast::StatTypeDecl>();
  ty->symbol = advance();

  expect(OP_EQ, "parsing type declaration");

  ty->type = parse_type();
  ty->loc = {first_loc.begin, ty->type->loc.end};

  optional(SEMICOLON);
  return ty;
}

const via::ast::Stat* via::Parser::parse_stat() {
  switch (peek()->kind) {
    case KW_IF:
      return (const ast::Stat*)parse_stat_if();
    case KW_WHILE:
      return (const ast::Stat*)parse_stat_while();
    case KW_VAR:
    case KW_CONST:
      return (const ast::Stat*)parse_stat_var_decl(true);
    case KW_DO: {
      SAVE_AND_ADVANCE();
      ;
      auto* scope = m_alloc.emplace<ast::StatScope>();
      scope->body = parse_scope();
      scope->loc = {first_loc.begin, scope->body->loc.end};
      return (const ast::Stat*)scope;
    }
    case KW_FOR:
      if (match(KW_VAR, 1))  // generic for loop
        return (const ast::Stat*)parse_stat_for();
      return (const ast::Stat*)parse_stat_for_each();
    case KW_RETURN:
      return (const ast::Stat*)parse_stat_return();
    case KW_ENUM:
      return (const ast::Stat*)parse_stat_enum();
    case KW_IMPORT:
      return (const ast::Stat*)parse_stat_import();
    case KW_FN:
      return (const ast::Stat*)parse_stat_func_decl();
    case KW_STRUCT:
      return (const ast::Stat*)parse_stat_struct_decl();
    case KW_TYPE:
      return (const ast::Stat*)parse_stat_type_decl();
    case SEMICOLON: {
      auto empty = m_alloc.emplace<ast::StatEmpty>();
      empty->loc = m_source.location(*advance());
      return (const ast::Stat*)empty;
    }
    default:
      break;
  }

  const Token* first = peek();
  if (!is_expr_start(first->kind)) {
  unexpected_token:
    throw ParserError(
        m_source.location(*first),
        std::format("unexpected token '{}' ({}) while parsing statement",
                    first->to_string(), to_string(first->kind)));
  }

  auto* expr = parse_expr();

  switch (peek()->kind) {
    case OP_EQ:
    case OP_PLUS_EQ:
    case OP_MINUS_EQ:
    case OP_STAR_EQ:
    case OP_SLASH_EQ:
    case OP_STAR_STAR_EQ:
    case OP_PERCENT_EQ:
    case OP_PIPE_EQ:
    case OP_AMP_EQ:
      return (const ast::Stat*)parse_stat_assign(expr);
    default: {
      auto empty = m_alloc.emplace<ast::StatExpr>();
      empty->expr = expr;
      empty->loc = empty->expr->loc;

      if TRY_COERCE (const ast::ExprCall, _, expr) {
        goto valid_expr_stat;
      } else {
        goto unexpected_token;
      }

    valid_expr_stat:
      optional(SEMICOLON);
      return (const ast::Stat*)empty;
    }
  }
}

via::SyntaxTree via::Parser::parse() {
  SyntaxTree nodes;
  while (!match(ENDOFFILE)) {
    try {
      auto* stat = parse_stat();
      nodes.push_back(stat);
    } catch (const ParserError& e) {
      m_diags.report(e.diag);
      break;
    }
  }
  return nodes;
}
