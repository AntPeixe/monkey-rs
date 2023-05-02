use std::io::{self, stdin, stdout, Write};

use crate::lexer::Lexer;

const PROMPT: &str = ">> ";

pub fn start() -> Result<(), io::Error> {
    // If it fails to read or write it'll just propagate the error
    let in_handle = stdin();
    let mut out_handle = stdout();
    println!("REPL starting...");

    loop {
        print!("{}", PROMPT);
        out_handle.flush()?;
        let mut input = String::new();
        let scanned = in_handle.read_line(&mut input);
        if scanned.is_ok() {
            let lex = Lexer::from(String::from(input.trim()));
            lex.into_iter()
                .map(|token| {
                    println!("{:?}", token);
                })
                .for_each(drop);
        } else {
            return Ok(());
        };
    }
}
