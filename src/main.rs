use std::env;
use std::fs;

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("incorrect arguments");
    }

    let filepath = &args[1];
    let contents = fs::read_to_string(filepath).expect("unable to read file");
    let mut lexer = lexer::Lexer::new();
    for token in lexer.tokens_from(contents) {
        println!("{:12} {:12} {}:{}", token.data, token.kind.to_string(), token.row, token.col);
    }
}
