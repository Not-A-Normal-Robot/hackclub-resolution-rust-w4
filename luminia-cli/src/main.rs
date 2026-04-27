use std::{
    fs,
    io::Write,
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
    Repl,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Run { path } => {
            run(&path);
        }
        Command::Repl => {
            repl();
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

fn repl() {
    const PREFIX: &str = "L: ";
    const LINE_CAPACITY: usize = 16384;

    let mut line = String::with_capacity(LINE_CAPACITY);
    let parser = ProgramParser::new();
    let mut int = Interpreter::new();

    loop {
        eprint!("{PREFIX}");
        let _ = std::io::stderr().flush();

        line.clear();
        line.shrink_to(LINE_CAPACITY);

        if let Err(e) = std::io::stdin().read_line(&mut line) {
            eprintln!("Failed to read from stdin: {e}");
            std::process::exit(1);
        }

        if line.is_empty() {
            eprintln!();
            break;
        }

        let stmts = match parser.parse(&line) {
            Ok(stmts) => stmts,
            Err(e) => {
                eprintln!("Syntax error: {e}");
                continue;
            }
        };

        if let Err(e) = int.run(stmts) {
            eprintln!("Runtime error: {e}");
            continue;
        }
    }
}
