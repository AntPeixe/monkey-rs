mod lexer;
mod repl;
mod parser;
mod ast;

fn main() {
    let _ = repl::start();
}
