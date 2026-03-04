use santiago::lexer::LexerRules;

pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        "DEFAULT" | "INT" = pattern r"[0-9]+";
        "DEFAULT" | "FLOAT" = pattern r"([0-9]+\.)|([0-9]*\.[0-9]+)";
        "DEFAULT" | "STRING" = pattern r#""([^"\\]|\\.)*""#;
        "DEFAULT" | "NAME" = pattern r"[a-zA-Z_]*";
        "DEFAULT" | "PLUS" = string "+";
        "DEFAULT" | "MINUS" = string "-";
        "DEFAULT" | "MULT" = string "*";
        "DEFAULT" | "DIV" = string "/";
        "DEFAULT" | "LEFT_CURLY_BRACKET" = string "{";
        "DEFAULT" | "RIGHT_CURLY_BRACKET" = string "}";
        "DEFAULT" | "LEFT_ROUND_BRACKET" = string "(";
        "DEFAULT" | "RIGHT_ROUND_BRACKET" = string ")";
        "DEFAULT" | "COMMENT" = pattern "//.*" => |lexer| lexer.skip();
        "DEFAULT" | "WS" = pattern r"\s" => |lexer| lexer.skip();
    )
}
