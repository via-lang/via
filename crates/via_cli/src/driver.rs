use std::path::Path;

use clap::{Parser, Subcommand};
use clio::Input;
use via::__compile;

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

pub fn run(path: &Path) -> anyhow::Result<()> {
    __compile(path);
    Ok(())
}

pub fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Run { input }) => run(input.path())?,
        Some(Command::Check { input: _ }) => todo!(),
        Some(Command::Format { input: _ }) => todo!(),
        Some(Command::Repl) | None => todo!(),
    };
    Ok(())
}
