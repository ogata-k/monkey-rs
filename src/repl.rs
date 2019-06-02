use std::io::{BufRead, BufReader, LineWriter, Read, Write};

use crate::lexer::Lexer;
use crate::token::TokenType;

/// 入力促進メッセージ
const PROMPT: &str = ">> ";
/// REPL終了用の入力記号
const FINISH_KEY: &str = "\u{4}";

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
    }
}
