//! Monkey言語の処理系に関するモジュールです。
//! Monkey言語に関してはREADMEを参照してください。

/// 字句解析が返しうるトークン列に関するモジュール
pub mod token;

/// 字句解析用モジュール
pub mod lexer;

/// パーサー(構文解析器)が返しうる木構造のノードに関するモジュール
pub mod ast;

/// パーサー(構文解析器)用モジュール
pub mod parser;

/// REPLを扱うためのモジュール
pub mod repl;

/// オブジェクトシステム用のモジュール
pub mod object;
