use std::env;
use std::fs;
use libronoa_lexer::lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("incorrect arguments");
    }

    let filepath = &args[1];
    let contents = fs::read_to_string(filepath).expect("unable to read file");
    let mut lexer = lexer::Lexer::new();
    lexer.tokens_from(contents).iter().for_each(|token| println!("{}", token));
}
