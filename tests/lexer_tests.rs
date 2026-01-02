mod common;
use common::assertions::{assert_equal, assert_true, AssertionResult};
use kern_lexer::lexer::Lexer;
use kern_lexer::token::{Token, TokenType};

fn run_lexer_test(input: &str, expected_tokens: Vec<TokenType>) -> AssertionResult {
    let mut lexer = Lexer::new(input);
    let mut actual_tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        if matches!(token.token_type, TokenType::Eof) {
            break;
        }
        actual_tokens.push(token.token_type);
    }

    if actual_tokens.len() != expected_tokens.len() {
        return AssertionResult::fail(format!(
            "Token count mismatch. Expected {}, got {}",
            expected_tokens.len(),
            actual_tokens.len()
        ));
    }

    for (i, (actual, expected)) in actual_tokens.iter().zip(expected_tokens.iter()).enumerate() {
        if actual != expected {
            return AssertionResult::fail(format!(
                "Token mismatch at index {}. Expected {:?}, got {:?}",
                i, expected, actual
            ));
        }
    }

    AssertionResult::pass()
}

#[test]
fn test_lexer_keywords() {
    let input = "entity rule flow constraint";
    let expected = vec![
        TokenType::Entity,
        TokenType::Rule,
        TokenType::Flow,
        TokenType::Constraint,
    ];

    let result = run_lexer_test(input, expected);
    assert!(result.success, "{:?}", result.message);
}

#[test]
fn test_lexer_identifiers() {
    let input = "user name age";
    let expected = vec![
        TokenType::Identifier("user".to_string()),
        TokenType::Identifier("name".to_string()),
        TokenType::Identifier("age".to_string()),
    ];

    let result = run_lexer_test(input, expected);
    assert!(result.success, "{:?}", result.message);
}

#[test]
fn test_lexer_numbers() {
    let input = "42 100 0";
    let expected = vec![
        TokenType::Number(42),
        TokenType::Number(100),
        TokenType::Number(0),
    ];

    let result = run_lexer_test(input, expected);
    assert!(result.success, "{:?}", result.message);
}

#[test]
fn test_lexer_operators() {
    let input = "== != > < >= <=";
    let expected = vec![
        TokenType::Equal,
        TokenType::NotEqual,
        TokenType::Greater,
        TokenType::Less,
        TokenType::GreaterEqual,
        TokenType::LessEqual,
    ];

    let result = run_lexer_test(input, expected);
    assert!(result.success, "{:?}", result.message);
}

#[test]
fn test_lexer_delimiters() {
    let input = "{ } ( ) , . :";
    let expected = vec![
        TokenType::LeftBrace,
        TokenType::RightBrace,
        TokenType::LeftParen,
        TokenType::RightParen,
        TokenType::Comma,
        TokenType::Dot,
        TokenType::Colon,
    ];

    let result = run_lexer_test(input, expected);
    assert!(result.success, "{:?}", result.message);
}

#[test]
fn test_lexer_complex_input() {
    let input = "entity User { name age }";
    let expected = vec![
        TokenType::Entity,
        TokenType::Identifier("User".to_string()),
        TokenType::LeftBrace,
        TokenType::Identifier("name".to_string()),
        TokenType::Identifier("age".to_string()),
        TokenType::RightBrace,
    ];

    let result = run_lexer_test(input, expected);
    assert!(result.success, "{:?}", result.message);
}

#[test]
fn test_lexer_simple_identifier() {
    let input = "abc";
    let expected = vec![TokenType::Identifier("abc".to_string())];

    let result = run_lexer_test(input, expected);
    assert!(result.success);
}
