use std::cmp::{Eq, PartialEq};
use std::env::args;
use std::fmt::Debug;
use std::hash::Hash;
use std::string::String;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() >= 2 {
        let _file_name = args[1].clone();
    } else {
        println!("\x1b[0;31mError: \x1b[0mPlease Specify a file\n\x1b[1;32mUsage:\x1b[0m py_compiler [filename]");
    }
    let lexed = lexer(String::from("True False hiii True g"));
    println!("Lexed Data: {:?}", lexed);
}

fn lexer(line: String) -> Vec<(Token, String)> {
    let mut lexed_information = Vec::new();
    let tokens: Vec<&str> = line.split_whitespace().collect();
    let mut values = vec![];
    for i in &tokens {
        &values.push(i);
    }
    for &token in values.clone() {
        if token == "True" {
            lexed_information.push((Token::Boolean, String::from("True")));
        } else if token == "False" {
            lexed_information.push((Token::Boolean, String::from("False")));
        } else {
            lexed_information.push((Token::Unknown, String::from(token)));
        }
    }
    return lexed_information;
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Token {
    Boolean,
    Byte,
    ByteArray,
    CloseDelimiter,
    Comment,
    Complex,
    Dictionary,
    Float,
    FrozenSet,
    Identifier,
    Integer,
    Keyword,
    List,
    NoneType,
    Operator,
    OpenDelimiter,
    Set,
    Str,
    Tuple,
    Unknown,
}
