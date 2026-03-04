use std::env;

use rscc::frontend::lexer::lexer_rules;
use rscc::frontend::grammar::grammar;

fn main() {
    // let input = "10. + 20.0 + .30\n30 - 20";

    // Read the file name from the command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        std::process::exit(1);
    }

    let input = std::fs::read_to_string(&args[1]).expect("File error");

    let lexer_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules, &input).expect("Error in lexing");

    let grammar = grammar();
    let _parse_tree = santiago::parser::parse(&grammar, &lexemes).expect("Error in parsing");

    // for lexeme in lexemes {
    //     // println!("Lexeme: {}, Value: {}, Line: {}", lexeme.kind, lexeme.raw, lexeme.position.line);
    //     // println!(" {}", lexeme.raw);
    // }

    // println!("Hello, world!");
}
