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
use viac_ast::stmt::Stmt;
use viac_lexer::lexer;
use viac_lexer::token::Token;
use viac_parser::error::Error as ParseError;
use viac_parser::parser;
use viac_source::source::Source;

pub struct Fixture {
    pub tokens: Vec<Token>,
    pub ast: Result<Vec<Stmt>, ParseError>,
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
    let tokens = lexer::tokenize(&source);
    let ast = parser::parse(tokens.as_slice());

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
