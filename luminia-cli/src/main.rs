use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use luminia_runner::{interpreter::Interpreter, parser::ProgramParser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Run { path: PathBuf },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Run { path } => {
            run(&path);
        }
    }
}

fn run(path: &Path) {
    let script = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading the script: {e}");
            std::process::exit(1);
        }
    };

    let program = match ProgramParser::new().parse(&script) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Syntax error: {e}");
            std::process::exit(1);
        }
    };

    let mut int = Interpreter::new();

    if let Err(e) = int.run(program) {
        eprintln!("Runtime error: {e}");
        std::process::exit(1);
    }
}
