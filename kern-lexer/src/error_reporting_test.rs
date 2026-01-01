use kern_lexer::{Lexer, Token, TokenType};
use kern_lexer::token::{LexerError, LexerErrorType};

fn test_lexer_error_reporting() {
    println!("Testing lexer error reporting...\n");

    // Test 1: Invalid character
    println!("Test 1: Invalid character");
    let input1 = "entity Test { id @location }";  // @ is invalid
    let mut lexer1 = Lexer::new(input1);
    let (tokens1, errors1) = lexer1.tokenize_with_errors();

    println!("Input: {}", input1);
    println!("Tokens: {:?}", tokens1.iter().map(|t| &t.token_type).collect::<Vec<_>>());
    println!("Errors: {}", errors1.len());
    for error in &errors1 {
        println!("  Error: {} at line {}, column {}", error.message, error.line, error.column);
    }
    println!();

    // Test 2: Unterminated string (which is invalid in KERN)
    println!("Test 2: Invalid string character");
    let input2 = r#"entity Test { id "unterminated }"#;  // " is invalid
    let mut lexer2 = Lexer::new(input2);
    let (tokens2, errors2) = lexer2.tokenize_with_errors();

    println!("Input: {}", input2);
    println!("Tokens: {:?}", tokens2.iter().map(|t| &t.token_type).collect::<Vec<_>>());
    println!("Errors: {}", errors2.len());
    for error in &errors2 {
        println!("  Error: {} at line {}, column {}", error.message, error.line, error.column);
    }
    println!();

    // Test 3: Valid input for comparison
    println!("Test 3: Valid input (no errors)");
    let input3 = "entity Test { id location }";
    let mut lexer3 = Lexer::new(input3);
    let (tokens3, errors3) = lexer3.tokenize_with_errors();

    println!("Input: {}", input3);
    println!("Tokens: {:?}", tokens3.iter().map(|t| &t.token_type).collect::<Vec<_>>());
    println!("Errors: {}", errors3.len());
    println!();
}

fn main() {
    test_lexer_error_reporting();
}