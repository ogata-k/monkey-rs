use std::io::{BufRead, BufReader, LineWriter, Read, Write};

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::token::TokenType;

/// 入力促進メッセージ
const PROMPT: &str = ">> ";
/// REPL終了用の入力記号
const FINISH_KEY: &str = "\u{4}";
/// 区切りの繰り返し数
const REPEAT_COUNT: usize = 30;

/// 入力を受けて改行区切りのトークン列に変換する関数
pub fn start(reader: impl Read, writer: impl Write) {
    let mut r = BufReader::new(reader);
    let mut w = LineWriter::new(writer);

    loop {
        write!(w, "{}", PROMPT).unwrap();
        w.flush().unwrap();
        let mut line = "".to_string();
        let res = r.read_line(&mut line);
        if res.is_err() {
            break;
        }
        if line.trim() == FINISH_KEY {
            break;
        }

        writeln!(w, "start Lexer: {}", "-".repeat(REPEAT_COUNT)).unwrap();

        let mut lexer = Lexer::new(&line);
        while let tok = lexer.next_token() {
            if tok.token_type_is(TokenType::EOF) {
                break;
            }
            if tok.token_type_is(TokenType::ILLEGAL) {
                panic!("異常な入力を検知しました。終了します。");
            }
            write!(w, "{:?}\n", tok).unwrap();
        }
        writeln!(w, "end Lexer: {}", "-".repeat(REPEAT_COUNT)).unwrap();

        writeln!(w, "start parser: {}", "-".repeat(REPEAT_COUNT)).unwrap();
        let mut parser = Parser::new(Lexer::new(&line));
        let program_opt = parser.parse_program();
        if program_opt.is_none() {
            let errors = parser.get_errors();
            writeln!(
                w,
                "パースエラーが{}件発生しました。",
                errors.len()
            )
            .unwrap();
            for error in errors {
                writeln!(w, "{}", error).unwrap();
            }
        } else {
            let program = program_opt.unwrap();
            let program_str = program.to_string();
            writeln!(w, "Program string: {}", program_str).unwrap();
            writeln!(w, "AST: {:?}", program).unwrap();
        }
        writeln!(w, "end parser: {}", "-".repeat(REPEAT_COUNT)).unwrap();
    }
}
