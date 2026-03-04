use rs_clp::frontend::lexer::lexer_rules;

fn main() {
    let input = "10. + 20.0 + .30";

    let lexer_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules, &input).expect("Error in lexing");

    for lexeme in lexemes {
        println!("Lexeme: {}, Value: {}", lexeme, lexeme.raw);
        // println!(" {}", lexeme.raw);
    }

    println!("Hello, world!");
}
