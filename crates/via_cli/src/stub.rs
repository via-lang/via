/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::TreeType;
use anyhow::Result;
use std::io::Write;
use via_core::compiler::{
    ast::stmt::Stmt,
    lexer::{token::Token, tokenize},
    parser::{error::Error as ParserError, parse},
    source::Source,
};

pub struct Fixture {
    pub tokens: Vec<Token>,
    pub ast: Result<Vec<Stmt>, ParserError>,
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
    println!("Running program...");

    let source = Source::new(src.to_string());
    let tokens = tokenize(&source);
    let ast = parse(tokens.as_slice());

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
