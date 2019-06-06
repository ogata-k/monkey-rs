use std::io::{stdin, stdout, Write};

use monkey_rs::repl::start;

fn main() {
    let r = stdin();
    let mut w = stdout();

    writeln!(
        w,
        "Hello! This is the Monkey programming language written by Rust."
    )
        .unwrap();
    writeln!(w, "Feel free to type in commands.").unwrap();
    writeln!(
        w,
        "If you want to finish REPL, input Ctrl+D and push Enter."
    )
        .unwrap();
    start(r, w);
}
