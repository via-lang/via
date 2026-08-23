use rowan::{Checkpoint, GreenNode, GreenNodeBuilder, Language, TextRange, TextSize};
use salsa::Accumulator;

use super::{
    SyntaxKind::{self, *},
    diag::{Diagnostic, Expected},
};
use crate::{
    db::{Db, IntoDiagnostic, SourceProgram},
    lex::{Token, TokenStream},
    syntax::Lang,
};

#[must_use]
pub struct NodeGuard<'db> {
    parser: *mut Parser<'db>,
    completed: bool,
}

impl<'db> NodeGuard<'db> {
    pub fn new(parser: &mut Parser<'db>) -> Self {
        Self {
            parser,
            completed: false,
        }
    }

    pub fn finish(mut self) {
        unsafe { &mut *self.parser }.builder.finish_node();
        self.completed = true;
    }
}

impl<'db> Drop for NodeGuard<'db> {
    fn drop(&mut self) {
        if !self.completed {
            unsafe { &mut *self.parser }.builder.finish_node();
        }
    }
}

pub struct Parser<'db> {
    db: &'db dyn Db,
    text: &'db str,
    tokens: &'db [Token],
    text_offset: usize,
    token_offset: usize,
    builder: GreenNodeBuilder<'db>,
}

impl<'db> Parser<'db> {
    pub fn new(db: &'db dyn Db, program: SourceProgram, token_stream: TokenStream<'db>) -> Self {
        Self {
            db,
            text: program.contents(db),
            tokens: token_stream.tokens(db),
            text_offset: 0,
            token_offset: 0,
            builder: Default::default(),
        }
    }

    fn report<D>(&mut self, diag: D, range: rowan::TextRange)
    where
        D: IntoDiagnostic,
    {
        diag.into_diagnostic(range).accumulate(self.db);
    }

    fn peek(&self) -> Option<(Token, usize)> {
        let mut token_idx = self.token_offset;
        let mut current_text_offset = self.text_offset;

        while token_idx < self.tokens.len() {
            let token = &self.tokens[token_idx];
            match token.kind {
                WHITESPACE | TAB | COMMENT => {
                    current_text_offset += token.len;
                    token_idx += 1;
                }
                _ => return Some((token.clone(), current_text_offset)),
            }
        }
        None
    }

    fn peek_kind(&self, kind: impl Into<SyntaxKind>) -> bool {
        self.peek()
            .is_some_and(|(token, _)| token.kind == kind.into())
    }

    fn bump(&mut self) {
        while self.token_offset < self.tokens.len() {
            let token = &self.tokens[self.token_offset];
            match token.kind {
                WHITESPACE | TAB | COMMENT => self.push_token(token),
                _ => break,
            }
        }

        if self.token_offset < self.tokens.len() {
            let token = &self.tokens[self.token_offset];
            self.push_token(token);
        }
    }

    fn expect(&mut self, kind: SyntaxKind) -> Option<()> {
        let expected = Expected::new(&[kind]);

        match self.peek() {
            Some((token, _)) if token.kind == kind => {
                self.bump();
                return Some(());
            }
            Some((token, offset)) => self.report(
                Diagnostic::UnexpectedToken {
                    expected,
                    got: token.kind,
                },
                TextRange::at(
                    TextSize::new(offset as u32),
                    TextSize::new(token.len as u32),
                ),
            ),
            None => self.report(
                Diagnostic::UnexpectedEof { expected },
                TextRange::at(TextSize::new(self.text_offset as u32), TextSize::new(0)),
            ),
        }
        None
    }

    fn option(&mut self, kind: SyntaxKind) -> bool {
        if self.peek_kind(kind) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn parse_scope(&mut self) -> Option<()> {
        let guard = self.start_node(SCOPE);
        self.expect(L_BRACE);

        while self.peek().is_some() && !self.peek_kind(R_BRACE) {
            self.parse_stat();
        }

        self.expect(R_BRACE);
        guard.finish();
        Some(())
    }

    fn parse_path(&mut self) -> Option<()> {
        let guard = self.start_node(PATH);

        if self.peek_kind(COLON_COLON) {
            let guard = self.start_node(PATH_HEAD);
            self.bump();
            guard.finish();
        }

        while let Some((token, _)) = self.peek()
            && matches!(token.kind, IDENT)
        {
            let guard = self.start_node(PATH_SEGMENT);
            self.bump();
            guard.finish();

            if !self.option(COLON_COLON) {
                break;
            }
        }

        guard.finish();
        Some(())
    }

    fn parse_delimited<F>(&mut self, end: SyntaxKind, sep: SyntaxKind, mut f: F) -> Option<()>
    where
        F: FnMut(&mut Self) -> Option<()>,
    {
        let mut first = true;
        while let Some((token, offset)) = self.peek() {
            if token.kind == end {
                break;
            }

            if !first {
                if token.kind == sep {
                    self.bump();
                    if self.peek()?.0.kind == end {
                        break;
                    }
                } else {
                    self.report(
                        Diagnostic::UnexpectedToken {
                            expected: Expected::new(&[sep, end]),
                            got: token.kind,
                        },
                        TextRange::at(
                            TextSize::new(offset as u32),
                            TextSize::new(token.len as u32),
                        ),
                    );
                    break;
                }
            }

            first = false;

            if f(self).is_none() {
                self.recover(&[sep, end]);
            }
        }

        Some(())
    }

    fn push_token(&mut self, token: &Token) {
        let token_text = &self.text[self.text_offset..self.text_offset + token.len];

        let kind = Lang::kind_to_raw(token.kind);
        self.builder.token(kind, token_text);

        self.text_offset += token.len;
        self.token_offset += 1;
    }

    fn recover(&mut self, tokens: &[SyntaxKind]) {
        if self.peek().is_none() {
            return;
        }

        let guard = self.start_node(ILLEGAL);
        self.bump();

        while let Some((token, _)) = self.peek() {
            if matches!(
                token.kind,
                SEMI | R_BRACE | KW_LET | KW_CONST | KW_TYPE | KW_FN
            ) || tokens.contains(&token.kind)
            {
                break;
            }
            self.bump();
        }

        guard.finish();
    }

    fn start_node(&mut self, kind: SyntaxKind) -> NodeGuard<'db> {
        self.builder.start_node(Lang::kind_to_raw(kind));
        NodeGuard::new(self)
    }

    fn start_node_at(&mut self, cp: Checkpoint, kind: SyntaxKind) -> NodeGuard<'db> {
        self.builder.start_node_at(cp, Lang::kind_to_raw(kind));
        NodeGuard::new(self)
    }

    fn parse_pat(&mut self) -> Option<()> {
        match self.peek()? {
            (token, _) if token.kind == KW_WILDCARD => {
                let guard = self.start_node(PAT_WILDCARD);
                self.bump();
                guard.finish();
            }

            (token, _) if token.kind == IDENT => {
                let guard = self.start_node(PAT_IDENT);
                self.bump();
                guard.finish();
            }

            (token, _) if token.kind == L_PAREN => {
                let guard = self.start_node(PAT_TUPLE);
                self.bump();

                self.parse_pat()?;
                self.expect(COMMA)?;
                self.parse_pat()?;

                self.expect(R_PAREN)?;
                guard.finish();
            }

            (token, offset) => {
                self.report(
                    Diagnostic::UnexpectedToken {
                        expected: Expected::new(&[IDENT, KW_MUT, L_PAREN]),
                        got: token.kind,
                    },
                    TextRange::at(
                        TextSize::new(offset as u32),
                        TextSize::new(token.len as u32),
                    ),
                );
                self.recover(&[EQ, COLON, COMMA, R_PAREN]);
            }
        }

        Some(())
    }

    fn parse_basic_ty(&mut self) -> Option<()> {
        let cp = self.builder.checkpoint();

        let (token, offset) = self.peek()?;
        match token.kind {
            L_PAREN => {
                self.bump();
                self.parse_ty()?;

                if self.option(COMMA) {
                    let guard = self.start_node_at(cp, TY_TUPLE);
                    self.parse_delimited(R_PAREN, COMMA, Self::parse_ty)?;
                    guard.finish();
                }

                self.expect(R_PAREN);
            }

            L_BRACKET => {
                self.bump();
                self.parse_ty()?;

                let guard = if self.option(SEMI) {
                    let guard = self.start_node_at(cp, TY_ARRAY);
                    self.parse_expr()?;
                    guard
                } else {
                    self.start_node_at(cp, TY_VECTOR)
                };

                self.expect(R_BRACKET)?;
                guard.finish();
            }

            HASH => {
                let guard = self.start_node(TY_MAP);

                self.bump();
                self.expect(L_BRACE)?;
                self.parse_ty()?;
                self.expect(COLON)?;
                self.parse_ty()?;
                self.expect(R_BRACE)?;

                guard.finish();
            }

            _ if token.kind.is_path_start() => {
                let guard = self.start_node_at(cp, TY_QUAL);
                self.parse_path();
                guard.finish();
            }

            _ => {
                self.report(
                    Diagnostic::UnexpectedToken {
                        expected: Expected::new(&[IDENT, L_BRACKET, L_PAREN, HASH]),
                        got: token.kind,
                    },
                    TextRange::at(
                        TextSize::new(offset as u32),
                        TextSize::new(token.len as u32),
                    ),
                );
                self.recover(&[EQ, SEMI, COMMA, R_PAREN, R_BRACKET, PIPE]);
            }
        }

        Some(())
    }

    fn parse_ty(&mut self) -> Option<()> {
        let mut cp = self.builder.checkpoint();

        self.parse_basic_ty()?;

        while self.peek_kind(PIPE) {
            let guard = self.start_node_at(cp, TY_UNION);

            self.bump();
            self.parse_basic_ty()?;

            guard.finish();

            cp = self.builder.checkpoint();
        }

        Some(())
    }

    fn parse_expr_if(&mut self) -> Option<()> {
        let guard = self.start_node(EXPR_IF);
        self.bump();
        self.parse_expr()?;
        self.parse_scope()?;

        let mut cp = self.builder.checkpoint();

        while self.option(KW_ELSE) {
            if self.option(KW_IF) {
                let guard = self.start_node_at(cp, ELSE_IF_CLAUSE);
                self.parse_expr()?;
                self.parse_scope()?;
                guard.finish();
            } else {
                let guard = self.start_node_at(cp, ELSE_CLAUSE);
                self.parse_scope()?;
                guard.finish();
                break;
            }
            cp = self.builder.checkpoint();
        }

        guard.finish();
        Some(())
    }

    fn parse_expr_for(&mut self) -> Option<()> {
        let guard = self.start_node(EXPR_FOR);

        self.expect(KW_FOR)?;
        self.parse_pat()?;

        self.expect(KW_IN)?;

        self.parse_expr()?;
        self.parse_scope()?;

        guard.finish();
        Some(())
    }

    fn parse_basic_expr(&mut self) -> Option<()> {
        match self.peek()? {
            (token, _) if token.kind.is_path_start() => {
                let guard = self.start_node(EXPR_QUAL);
                self.parse_path();
                guard.finish();
            }

            (token, _) if token.kind == L_PAREN => {
                let cp = self.builder.checkpoint();
                self.bump(); // Consume L_PAREN

                if self.option(R_PAREN) {
                    self.start_node_at(cp, EXPR_UNIT).finish();
                    return Some(());
                }

                self.parse_expr()?;

                if self.peek_kind(COMMA) {
                    todo!("Tuple expression syntax")
                    // let guard = self.start_node_at(cp, EXPR_TUPLE);
                    //
                    // while self.option(COMMA) {
                    //     if self.peek_kind(R_PAREN) {
                    //         break;
                    //     }
                    //     self.parse_expr()?;
                    // }
                    //
                    // self.expect(R_PAREN);
                } else if self.option(R_PAREN) {
                    self.start_node_at(cp, EXPR_GROUP).finish();
                } else {
                    return None;
                }
            }

            (token, _) if token.kind.is_bool_literal() => {
                let guard = self.start_node(EXPR_BOOL);
                self.bump();
                guard.finish();
            }

            (token, _) if token.kind.is_int_literal() => {
                let guard = self.start_node(EXPR_INT);
                self.bump();
                guard.finish();
            }

            (token, _) if token.kind == LIT_FLOAT => {
                let guard = self.start_node(EXPR_FLOAT);
                self.bump();
                guard.finish();
            }

            (token, _) if token.kind == LIT_STRING => {
                let guard = self.start_node(EXPR_STRING);
                self.bump();
                guard.finish();
            }

            (token, _) if token.kind == L_BRACKET => {
                let guard = self.start_node(EXPR_ARRAY);

                self.bump();
                self.parse_delimited(R_BRACKET, COMMA, Self::parse_expr)?;
                self.expect(R_BRACKET)?;

                guard.finish();
            }

            (token, _) if token.kind == HASH => {
                let guard = self.start_node(EXPR_MAP);

                self.bump();
                self.expect(L_BRACE)?;
                self.parse_delimited(R_BRACE, COMMA, |parser| {
                    let inner_guard = parser.start_node(MAP_PAIR);

                    parser.expect(L_BRACKET)?;
                    parser.parse_expr()?;
                    parser.expect(R_BRACKET)?;
                    parser.expect(COLON)?;
                    parser.parse_expr()?;

                    inner_guard.finish();
                    Some(())
                })?;

                self.expect(R_BRACE)?;
                guard.finish();
            }

            (token, _) if token.kind == KW_IF => self.parse_expr_if()?,
            (token, _) if token.kind == KW_FOR => self.parse_expr_for()?,

            (token, offset) => {
                self.report(
                    Diagnostic::UnexpectedToken {
                        expected: Expected::new(&[KW_TRUE, KW_FALSE, LIT_INT, LIT_FLOAT]),
                        got: token.kind,
                    },
                    TextRange::at(
                        TextSize::new(offset as u32),
                        TextSize::new(token.len as u32),
                    ),
                );
                self.recover(&[SEMI, COMMA, R_PAREN, R_BRACKET, R_BRACE]);
            }
        };

        Some(())
    }

    fn parse_postfix_expr(&mut self) -> Option<()> {
        let cp = self.builder.checkpoint();

        self.parse_basic_expr()?;

        while let Some((op, _)) = self.peek() {
            match op.kind {
                L_BRACKET => {
                    let guard = self.start_node_at(cp, EXPR_INDEX);

                    self.bump();
                    self.parse_expr()?;
                    self.expect(R_BRACKET)?;

                    guard.finish();
                }
                L_PAREN => {
                    let guard = self.start_node_at(cp, EXPR_CALL);

                    self.bump();
                    self.parse_delimited(R_PAREN, COMMA, Self::parse_expr)?;
                    self.expect(R_PAREN)?;

                    guard.finish();
                }
                _ => break,
            }
        }

        Some(())
    }

    fn parse_unary_expr(&mut self) -> Option<()> {
        if self.peek()?.0.kind.is_unary_op() {
            let guard = self.start_node(EXPR_UNARY);

            self.bump();
            self.parse_postfix_expr()?;

            guard.finish();
            return Some(());
        }

        self.parse_postfix_expr()
    }

    fn parse_binary_expr(&mut self, min_prec: u8) -> Option<()> {
        let mut cp = self.builder.checkpoint();

        self.parse_unary_expr()?;

        while let Some((op, _)) = self.peek() {
            let prec = match op.kind.precedence() {
                Some(prec) if prec >= min_prec => prec,
                _ => break,
            };

            let guard = self.start_node_at(cp, EXPR_BINARY);

            self.bump();
            self.parse_binary_expr(prec + !(op.kind.is_right_assoc()) as u8)?;

            guard.finish();
            cp = self.builder.checkpoint();
        }

        Some(())
    }

    fn parse_expr(&mut self) -> Option<()> {
        self.parse_binary_expr(0)
    }

    fn parse_def_const(&mut self, cp: Checkpoint) -> Option<()> {
        let guard = self.start_node_at(cp, DEF_CONST);

        self.expect(KW_CONST)?;
        self.expect(IDENT)?;
        self.expect(COLON)?;
        self.parse_ty()?;
        self.expect(EQ)?;
        self.parse_expr()?;
        self.expect(SEMI)?;

        guard.finish();
        Some(())
    }

    fn parse_def_type(&mut self, cp: Checkpoint) -> Option<()> {
        let guard = self.start_node_at(cp, DEF_TYPE);

        self.expect(KW_TYPE)?;
        self.expect(IDENT)?;
        self.expect(EQ)?;
        self.parse_ty()?;
        self.expect(SEMI)?;

        guard.finish();
        Some(())
    }

    fn parse_def_fn(&mut self, cp: Checkpoint) -> Option<()> {
        let guard = self.start_node_at(cp, DEF_FN);

        self.expect(KW_FN)?;
        self.expect(IDENT)?;
        self.expect(L_PAREN)?;
        self.parse_delimited(R_PAREN, COMMA, |parser| {
            let inner_guard = parser.start_node(PARAMETER);

            parser.parse_pat()?;
            parser.expect(COLON)?;
            parser.parse_ty()?;

            inner_guard.finish();
            Some(())
        })?;

        self.expect(R_PAREN)?;

        if self.option(ARROW) {
            self.parse_ty()?;
        }

        self.parse_scope()?;
        guard.finish();
        Some(())
    }

    fn parse_def(&mut self) -> Option<()> {
        let cp = self.builder.checkpoint();

        let (token, offset) = self.peek()?;
        match token.kind {
            KW_CONST => self.parse_def_const(cp),
            KW_TYPE => self.parse_def_type(cp),
            KW_FN => self.parse_def_fn(cp),

            _ => {
                self.report(
                    Diagnostic::UnexpectedToken {
                        expected: Expected::new(&[KW_CONST, KW_TYPE, KW_FN]),
                        got: token.kind,
                    },
                    TextRange::at(
                        TextSize::new(offset as u32),
                        TextSize::new(token.len as u32),
                    ),
                );

                self.recover(&[]);
                Some(())
            }
        }
    }

    fn parse_let(&mut self) -> Option<()> {
        let guard = self.start_node(STAT_LET);

        self.expect(KW_LET)?;
        self.parse_pat()?;

        if self.option(COLON) {
            self.parse_ty()?;
        }

        self.expect(EQ)?;
        self.parse_expr()?;
        self.expect(SEMI)?;

        guard.finish();
        Some(())
    }

    fn parse_stat(&mut self) -> Option<()> {
        let (token, offset) = self.peek()?;
        match token.kind {
            KW_LET => self.parse_let(),

            KW_CONST | KW_TYPE | KW_FN => {
                let guard = self.start_node(STAT_DEF);

                self.parse_def()?;

                guard.finish();
                Some(())
            }

            kind if kind.is_expr_start() => {
                let cp = self.builder.checkpoint();
                self.parse_expr()?;

                let guard = if self.option(SEMI) {
                    self.start_node_at(cp, STAT_DISCARD)
                } else {
                    self.start_node_at(cp, STAT_CONSUME)
                };

                guard.finish();
                Some(())
            }

            _ => {
                self.report(
                    Diagnostic::UnexpectedToken {
                        expected: Expected::new(&[KW_LET]),
                        got: token.kind,
                    },
                    TextRange::at(
                        TextSize::new(offset as u32),
                        TextSize::new(token.len as u32),
                    ),
                );

                self.recover(&[]);
                Some(())
            }
        }
    }

    pub fn parse(mut self) -> GreenNode {
        let guard = self.start_node(ROOT);

        while self.peek().is_some() {
            self.parse_def();
        }

        guard.finish();
        self.builder.finish()
    }
}
