use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let _file_name = args[1].clone();
    } else {
        println!("\x1b[0;31mError: \x1b[0mPlease Specify a file\n\x1b[1;32mUsage:\x1b[0m py_compiler [filename]");
    }
}

#[allow(dead_code, unused_variables)]
fn lexer(line: &str) {
    //
}

pub enum Tokens {
    Keyword,
    Comment,
    Operator,
    Identifier,
    OpenDelimiter,
    CloseDelimiter,
}
