/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use clio::Input;
use std::io::Read;

mod stub;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum TreeType {
    None,
    Token,
    Syntax,
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

fn read_input(mut input: Input) -> Result<String> {
    let mut src = String::new();
    input
        .read_to_string(&mut src)
        .context("failed to read input")?;
    Ok(src)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Run { input, tree }) => {
            let src = read_input(input)?;
            let fixture = stub::run(&src)?;
            fixture.dump(tree);
        }
        Some(Command::Check { input }) => {
            let src = read_input(input)?;
            stub::check(&src)?;
        }
        Some(Command::Format { input }) => {
            let src = read_input(input)?;
            let formatted = stub::format(&src)?;
            print!("{formatted}");
        }
        Some(Command::Repl) | None => stub::repl()?,
    };
    Ok(())
}
