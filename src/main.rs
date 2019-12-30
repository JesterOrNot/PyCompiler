use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::env::args;
use std::hash::Hash;
use std::string::String;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() >= 2 {
        let _file_name = args[1].clone();
    } else {
        println!("\x1b[0;31mError: \x1b[0mPlease Specify a file\n\x1b[1;32mUsage:\x1b[0m py_compiler [filename]");
    }
}

#[allow(dead_code, unused_variables)]
fn lexer(line: &str) -> HashMap<Token, String> {
    let lexed_information: HashMap<Token, String> = HashMap::new();
    return lexed_information;
}

#[derive(Hash, Eq, PartialEq)]
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
}
