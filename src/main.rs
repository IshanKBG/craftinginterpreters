use std::env;
use std::io::{self, BufRead, Write};
use std::process;
use tree::scanner::{Scanner, ScannerError, Token};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        eprintln!("Usage: jlox [script]");
        process::exit(64);
    } else {
        run_prompt().unwrap();
    }
}

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = stdin.lock();

    loop {
        print!("> ");
        stdout.flush()?;

        let mut line = String::new();

        if reader.read_line(&mut line)? == 0 {
            break;
        }
        if let Err(e) = run(&line) {
            eprintln!("Error: {:#?}", e);
        }
    }

    Ok(())
}

fn run(source: &str) -> Result<(), ScannerError> {
    let tokens = scan_tokens(source.to_string())?;
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

fn scan_tokens(input: String) -> Result<Vec<Token>, ScannerError> {
    let mut scanner = Scanner::new();

    scanner.scan_tokens(input);

    match scanner.err {
        Some(err) => Err(err),
        None => Ok(scanner.tokens),
    }
}
