use std::io::{Read, Write, BufReader, BufWriter, BufRead};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

/// 入力促進メッセージ
const PROMPT: &str = ">> ";

/// 入力を受けて改行区切りのトークン列に変換する関数
pub fn start(mut reader: impl Read, writer: impl Write) {
    let mut r = BufReader::new(reader);
    let mut w = BufWriter::new(writer);

    loop {
        write!(w, "{}",  PROMPT).expect("panic! writer error!");
        let mut line = "".to_string();
        let res = r.read_line(&mut line);
        if res.is_err(){
            break;
        }
        let mut lexer = Lexer::new(&line);

        while let tok = lexer.next_token() {
            if tok == Token::new(TokenType::EOF, "") {
                break;
            }
            write!(w, "{:?}\n", tok).expect("panic! writer error!");
        }
    }
}
