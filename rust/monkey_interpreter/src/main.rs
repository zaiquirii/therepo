use std::io::{BufReader, stdin, stdout};

mod lexer;
mod token;
mod repl;
mod parser;

fn main() -> std::io::Result<()> {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands\n");
    repl::start(stdin().lock(), stdout())
}
