use std::io::{BufRead, BufReader, BufWriter, LineWriter, Read, Write};

use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

/// 入力促進メッセージ
const PROMPT: &str = ">> ";
/// REPL終了用の入力記号
const FINISH_KEY: &str = "\u{4}";

/// 入力を受けて改行区切りのトークン列に変換する関数
pub fn start(reader: impl Read, writer: impl Write) {
    let mut r = BufReader::new(reader);
    let mut w = LineWriter::new(writer);

    loop {
        write!(w, "{}", PROMPT);
        w.flush();
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
            if tok == Token::new(TokenType::EOF, "") {
                break;
            }
            write!(w, "{:?}\n", tok);
        }
    }
}
