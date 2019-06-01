use std::io::{Read, Write, stdin, stdout};
use monkey_rs::repl::start;

fn main() {
    let r = stdin();
    let mut w = stdout();

    writeln!(w, "Hello! This is the Monkey programming language written by Rust.");
    writeln!(w, "Feel free to type in commands.");
    writeln!(w, "If you want to finish REPL, input Ctrl+D and push Enter.");
    start(r, w);
}
