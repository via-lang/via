use std::sync::Arc;

use rowan::Language;
use via_macros::{Syntax, syntax_tree};

use crate::{
    db::{Db, SourceProgram},
    lex::tokenize_program,
};

use parser::Parser;

pub mod diag;
mod parser;

pub type GreenNode = rowan::GreenNode;
pub type SyntaxNode = rowan::SyntaxNode<Lang>;
pub type SyntaxToken = rowan::SyntaxToken<Lang>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lang;

#[rustfmt::skip]
#[allow(non_camel_case_types)]
#[derive(Syntax, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    /*----------[ TOKEN TREE ]----------*/

    // Primitives
    ILLEGAL,
    COMMENT,
    TAB,
    WHITESPACE,
    LIT_INT,
    LIT_XINT,
    LIT_BINT,
    LIT_FLOAT,
    LIT_STRING,
    IDENT,

    // Keywords
    #[keyword("_")]      KW_WILDCARD,
    #[keyword("mut")]    KW_MUT,
    #[keyword("let")]    KW_LET,
    #[keyword("fn")]     KW_FN,
    #[keyword("type")]   KW_TYPE,
    #[keyword("const")]  KW_CONST,
    #[keyword("if")]     KW_IF,
    #[keyword("else")]   KW_ELSE,
    #[keyword("for")]    KW_FOR,
    #[keyword("in")]     KW_IN,
    #[keyword("true")]   KW_TRUE,
    #[keyword("false")]  KW_FALSE,

    // Operators
    #[operator(".")]    DOT,
    #[operator(",")]    COMMA,
    #[operator(";")]    SEMI,
    #[operator(":")]    COLON,
    #[operator("::")]   COLON_COLON,
    #[operator("->")]   ARROW,
    #[operator("=>")]   FAT_ARROW,
    #[operator("?")]    QUESTION,
    #[operator("(")]    L_PAREN,
    #[operator(")")]    R_PAREN,
    #[operator("[")]    L_BRACKET,
    #[operator("]")]    R_BRACKET,
    #[operator("{")]    L_BRACE,
    #[operator("}")]    R_BRACE,
    #[operator("+")]    PLUS,
    #[operator("-")]    MINUS,
    #[operator("*")]    STAR,
    #[operator("/")]    SLASH,
    #[operator("**")]   STAR_STAR,
    #[operator("%")]    PERCENT,
    #[operator("&")]    AMP,
    #[operator("~")]    TILDE,
    #[operator("^")]    CARET,
    #[operator("|")]    PIPE,
    #[operator("<<")]   LT_LT,
    #[operator(">>")]   GT_GT,
    #[operator("#")]    HASH,
    #[operator("!")]    BANG,
    #[operator("\"")]   QUOTE,
    #[operator("<")]    LT,
    #[operator(">")]    GT,
    #[operator("..")]   DOT_DOT,
    #[operator("..=")]  DOT_DOT_EQ,
    #[operator("&&")]   AMP_AMP,
    #[operator("||")]   PIPE_PIPE,
    #[operator("=")]    EQ,
    #[operator("==")]   EQ_EQ,
    #[operator("+=")]   PLUS_EQ,
    #[operator("-=")]   MINUS_EQ,
    #[operator("*=")]   STAR_EQ,
    #[operator("/=")]   SLASH_EQ,
    #[operator("**=")]  STAR_STAR_EQ,
    #[operator("%=")]   PERCENT_EQ,
    #[operator("&=")]   AMP_EQ,
    #[operator("^=")]   CARET_EQ,
    #[operator("|=")]   PIPE_EQ,
    #[operator("<<=")]  LT_LT_EQ,
    #[operator(">>=")]  GT_GT_EQ,
    #[operator("!=")]   BANG_EQ,
    #[operator("<=")]   LT_EQ,
    #[operator(">=")]   GT_EQ,

    /*----------[ SYNTAX TREE ]----------*/

    ROOT,
    SCOPE,
    MAP_PAIR,
    ELSE_CLAUSE,
    ELSE_IF_CLAUSE,
    PARAMETER,
    PATH_HEAD,
    PATH_SEGMENT,
    PATH,

    // Pattern
    PAT_WILDCARD,
    PAT_IDENT,
    PAT_TUPLE,

    // Type
    TY_UNIT,
    TY_BOOL,
    TY_INT,
    TY_FLOAT,
    TY_STRING,
    TY_QUAL,
    TY_TUPLE,
    TY_VECTOR,
    TY_ARRAY,
    TY_MAP,
    TY_UNION,

    // Expression
    EXPR_GROUP,
    EXPR_QUAL,
    EXPR_UNIT,
    EXPR_BOOL,
    EXPR_INT,
    EXPR_FLOAT,
    EXPR_STRING,
    EXPR_RANGE,
    EXPR_ARRAY,
    EXPR_MAP,
    EXPR_UNARY,
    EXPR_BINARY,
    EXPR_CALL,
    EXPR_INDEX,
    EXPR_IF,
    EXPR_FOR,

    // Visibility
    VIS_PRIV,
    VIS_PUB,
    VIS_RESTRICTED,

    // Definition
    DEF_CONST,
    DEF_FN,
    DEF_TYPE,
    DEF_TRAIT,
    DEF_MOD,

    // Statement
    STAT_LET,
    STAT_DISCARD,
    STAT_CONSUME,
    STAT_DEF,

}

syntax_tree! {
    /*----------[ PATTERN ]----------*/

    struct Wildcard in Pat {}

    struct Ident in Pat {
        refer: Leaf(AMP)?,
        mutable: Leaf(KW_MUT)?,
        ident: Leaf(IDENT),
    }

    struct Tuple in Pat {
        pats: Delimited(Branch(Pat), L_PAREN, R_PAREN),
    }

    /*----------[ TYPE ]----------*/

    struct Qual in Ty {
        path: Branch(Path),
    }

    struct Unit in Ty {}

    struct Vector in Ty {
        inner: Branch(Ty),
    }

    struct Array in Ty {
        ty: Branch(Ty),
        size: Branch(Expr),
    }

    struct Map in Ty {
        key: Branch(Ty),
        value: Branch(Ty),
    }

    /*----------[ EXPRESSION ]----------*/

    struct Group in Expr {
        inner: Branch(Expr),
    }

    struct Qual in Expr {
        path: Branch(Path),
    }

    struct Unit in Expr {}

    struct Bool in Expr {
        literal: Group(is_bool_literal),
    }

    struct Int in Expr {
        literal: Group(is_int_literal),
    }

    struct Float in Expr {
        literal: Leaf(LIT_FLOAT),
    }

    struct String in Expr {
        literal: Leaf(LIT_STRING),
    }

    struct Array in Expr {
        exprs: Delimited(Branch(Expr), L_BRACKET, R_BRACKET),
    }

    struct MapPair {
        key: Branch(Expr),
        value: Branch(Expr),
    }

    struct Map in Expr {
        pairs: Delimited(Branch(MapPair), L_BRACE, R_BRACE),
    }

    struct Call in Expr {
        callee: Branch(Expr),
        args: Delimited(Branch(Expr), L_PAREN, R_PAREN),
    }

    struct Index in Expr {
        outer: Branch(Expr),
        inner: Branch(Expr),
    }

    struct Unary in Expr {
        op: Group(is_unary_op),
        expr: Branch(Expr),
    }

    struct Binary in Expr {
        op: Group(is_binary_op),
        lhs: Branch(Expr),
        rhs: Branch(Expr),
    }

    struct ElseClause {
        scope: Branch(Scope),
    }

    struct ElseIfClause {
        cond: Branch(Expr),
        scope: Branch(Scope),
    }

    struct If in Expr {
        cond: Branch(Expr),
        scope: Branch(Scope),
        else_clause: Branch(ElseClause)?,
        else_if_clauses: Branch(ElseIfClause)*,
    }

    struct For in Expr {
        pat: Branch(Pat),
        iter: Branch(Expr),
        scope: Branch(Scope),
    }

    /*----------[ VISIBILITY ]----------*/

    struct Priv in Vis {}
    struct Pub in Vis {}

    /*----------[ DEFINITION ]----------*/

    struct Const in Def {
        vis: Branch(Vis)?,
        name: Leaf(IDENT),
        ty: Branch(Ty),
        init: Branch(Expr),
    }

    struct Type in Def {
        vis: Branch(Vis)?,
        name: Leaf(IDENT),
        ty: Branch(Ty),
    }

    struct Fn in Def {
        vis: Branch(Vis)?,
        name: Leaf(IDENT),
        params: Delimited(Branch(Parameter), L_PAREN, R_PAREN),
        result: Branch(Ty)?,
        body: Branch(Scope),
    }

    /*----------[ STATEMENT ]----------*/

    struct Let in Stat {
        pat: Branch(Pat),
        ty: Branch(Ty)?,
        init: Branch(Expr)?,
    }

    struct Parameter {
        pat: Branch(Pat),
        ty: Branch(Ty),
    }

    struct Discard in Stat {
        expr: Branch(Expr),
    }

    struct Consume in Stat {
        expr: Branch(Expr),
    }

    struct Def in Stat {
        def: Branch(Def),
    }

    /*----------[ SPECIAL ]----------*/

    struct PathHead {
        token: Group(is_path_head),
    }

    struct PathSegment {
        ident: Leaf(IDENT),
    }

    struct Path {
        head: Branch(PathHead)?,
        segments: Branch(PathSegment)*,
    }

    struct Scope {
        stats: Branch(Stat)*,
    }

    struct Root {
        defs: Branch(Def)*,
    }

}

#[salsa::tracked(debug)]
pub struct Ast<'db> {
    #[tracked]
    pub root: GreenNode,
}

use SyntaxKind::*;

#[salsa::tracked]
pub fn parse_program<'db>(db: &'db dyn Db, program: SourceProgram) -> Arc<Ast<'db>> {
    let token_stream = tokenize_program(db, program);

    let parser = Parser::new(db, program, *token_stream);
    let root = parser.parse();

    Arc::new(Ast::new(db, root))
}

impl Language for Lang {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        // SAFETY: SyntaxKind is repr(u16)
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind as u16)
    }
}

impl SyntaxKind {
    pub fn is_bool_literal(self) -> bool {
        matches!(self, KW_TRUE | KW_FALSE)
    }

    pub fn is_int_literal(self) -> bool {
        matches!(self, LIT_INT | LIT_XINT | LIT_BINT)
    }

    pub fn is_unary_op(self) -> bool {
        matches!(self, MINUS | TILDE | BANG)
    }

    pub fn is_generic_argument(self) -> bool {
        Ty::is(self) || matches!(self, EXPR_BOOL | EXPR_INT)
    }

    pub fn is_binary_op(self) -> bool {
        self.precedence().is_some()
    }

    pub fn is_range_op(self) -> bool {
        matches!(self, DOT_DOT | DOT_DOT_EQ)
    }

    pub fn is_path_head(self) -> bool {
        matches!(self, COLON_COLON)
    }

    pub fn is_path_start(self) -> bool {
        self.is_path_head() || matches!(self, IDENT)
    }

    pub fn is_expr_start(self) -> bool {
        matches!(
            self,
            KW_TRUE
                | KW_FALSE
                | LIT_INT
                | LIT_XINT
                | LIT_BINT
                | LIT_FLOAT
                | LIT_STRING
                | L_PAREN
                | L_BRACKET
                | HASH
                | MINUS
                | BANG
                | TILDE
                | IDENT
                | KW_IF
                | KW_FOR
        )
    }

    pub fn is_right_assoc(self) -> bool {
        matches!(self, STAR_STAR)
    }

    pub fn precedence(self) -> Option<u8> {
        let prec_map: [&[SyntaxKind]; _] = [
            &[EQ],                   // Assignment
            &[PIPE_PIPE],            // Logical OR
            &[AMP_AMP],              // Logical AND
            &[DOT_DOT, DOT_DOT_EQ],  // Range
            &[EQ_EQ, BANG_EQ],       // Equality (==, !=)
            &[PIPE],                 // Bitwise OR
            &[CARET],                // Bitwise XOR
            &[AMP],                  // Bitwise AND
            &[PLUS, MINUS],          // Add, Sub
            &[STAR, SLASH, PERCENT], // Mul, Div, Rem
            &[LT_LT, GT_GT],         // Bitwise Shifts (<<, >>)
            &[STAR_STAR],            // Exponentiation
        ];

        let mut prec = 1u8;
        for tokens in prec_map {
            if tokens.contains(&self) {
                return Some(prec);
            }
            prec += 1;
        }
        None
    }
}
