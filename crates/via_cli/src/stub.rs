/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::TreeType;
use anyhow::Result;
use std::io::Write;
use std::rc::Rc;
use viac::prelude::{
    ast::{node::Node, stmt::Stmt},
    diags::{context::Context as DiagContext, renderer::TermRenderer},
    lexer::{self, token::Token},
    parser,
    source::Source,
};

pub struct Fixture {
    pub tokens: Rc<[Token]>,
    pub ast: Rc<[Node<Stmt>]>,
}

impl Fixture {
    pub fn dump(&self, tree: TreeType) {
        match tree {
            TreeType::Token => println!("Tokens: {:#?}", self.tokens),
            TreeType::Syntax => println!("AST: {:#?}", self.ast),
            _ => {}
        }
    }
}

pub fn run(src: &str) -> Result<Fixture> {
    let source = Source::new(src.to_string());
    let renderer = TermRenderer::new(&source, None);
    let mut diag_ctxt = DiagContext::new(&source, renderer);

    let tokens = lexer::tokenize(&source);
    let ast = match parser::parse(&source, &tokens) {
        Ok(toks) => toks,
        Err(e) => {
            diag_ctxt.emit(e)?;
            return Err(anyhow::Error::msg("compilation failure"));
        }
    };

    Ok(Fixture { tokens, ast })
}

pub fn check(_: &str) -> Result<()> {
    println!("Checking program...");
    // parse → check
    Ok(())
}

pub fn format(src: &str) -> Result<String> {
    Ok(src.to_string()) // placeholder
}

pub fn repl() -> Result<()> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    loop {
        write!(stdout, "> ")?;
        stdout.flush()?;

        let mut line = String::new();
        if stdin.read_line(&mut line)? == 0 {
            break;
        }

        // evaluate line
        println!("=> {}", line.trim());
    }
    Ok(())
}
