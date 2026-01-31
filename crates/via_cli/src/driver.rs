/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::io::Read;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use clio::Input;
use viac::module::Fixture;

use crate::stub;

#[derive(clap::ValueEnum, Clone, Debug)]
enum TreeType {
    None,
    Token,
    Syntax,
}

trait Dump<T> {
    fn dump(&self, t: T);
}

impl Dump<TreeType> for Fixture {
    fn dump(&self, tree: TreeType) {
        match tree {
            TreeType::Token => println!("Tokens: {:#?}", self.tt),
            TreeType::Syntax => println!("AST: {:#?}", self.ast),
            _ => {}
        }
    }
}

#[derive(Parser)]
#[command(
    name = "via",
    version,
    about = "The via Programming Language Interpreter",
    arg_required_else_help = false
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Run {
        #[arg(value_name = "FILE", default_value = "-")]
        input: Input,

        #[clap(long)]
        #[clap(value_enum, default_value_t=TreeType::None)]
        tree: TreeType,
    },
    Check {
        #[arg(value_name = "FILE", default_value = "-")]
        input: Input,
    },
    Format {
        #[arg(value_name = "FILE", default_value = "-")]
        input: Input,
    },
    Repl,
}

pub fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Run { input, tree }) => {
            let fixture = stub::run(input.path())?;
            fixture.dump(tree);
        }
        Some(Command::Check { input: _ }) => todo!(),
        Some(Command::Format { input: _ }) => todo!(),
        Some(Command::Repl) | None => todo!(),
    };
    Ok(())
}
