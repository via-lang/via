/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use anyhow::Result;
use clap::{Parser, Subcommand};
use clio::Input;

use crate::stub;

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
        Some(Command::Run { input }) => stub::run(input.path())?,
        Some(Command::Check { input: _ }) => todo!(),
        Some(Command::Format { input: _ }) => todo!(),
        Some(Command::Repl) | None => todo!(),
    };
    Ok(())
}
