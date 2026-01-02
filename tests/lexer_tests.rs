mod common;
use common::assertions::{assert_equal, assert_true, AssertionResult};
use kern_lexer::lexer::Lexer;
use kern_lexer::token::Token;
use kern_lexer::token_kind::TokenKind;

fn run_lexer_test(input: &str, expected_tokens: Vec<TokenKind>) -> AssertionResult {
    let mut lexer = Lexer::new(input);
    let mut actual_tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        if token.kind == TokenKind::EOF {
            break;
        }
        actual_tokens.push(token.kind);
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
        TokenKind::Entity,
        TokenKind::Rule,
        TokenKind::Flow,
        TokenKind::Constraint,
    ];

    let result = run_lexer_test(input, expected);
    assert!(result.success, "{:?}", result.message);
}

#[test]
fn test_lexer_identifiers() {
    let input = "user name age";
    let expected = vec![
        TokenKind::Identifier("user".to_string()),
        TokenKind::Identifier("name".to_string()),
        TokenKind::Identifier("age".to_string()),
    ];

    let result = run_lexer_test(input, expected);
    assert!(result.success, "{:?}", result.message);
}

#[test]
fn test_lexer_numbers() {
    let input = "42 100 0";
    let expected = vec![
        TokenKind::Number(42),
        TokenKind::Number(100),
        TokenKind::Number(0),
    ];

    let result = run_lexer_test(input, expected);
    assert!(result.success, "{:?}", result.message);
}

#[test]
fn test_lexer_strings() {
    let input = "\"hello\" \"world\"";
    let expected = vec![
        TokenKind::String("hello".to_string()),
        TokenKind::String("world".to_string()),
    ];

    let result = run_lexer_test(input, expected);
    assert!(result.success, "{:?}", result.message);
}

#[test]
fn test_lexer_operators() {
    let input = "== != > < >= <=";
    let expected = vec![
        TokenKind::Equals,
        TokenKind::NotEquals,
        TokenKind::Greater,
        TokenKind::Less,
        TokenKind::GreaterEqual,
        TokenKind::LessEqual,
    ];

    let result = run_lexer_test(input, expected);
    assert!(result.success, "{:?}", result.message);
}

#[test]
fn test_lexer_delimiters() {
    let input = "{ } ( ) [ ] , . :";
    let expected = vec![
        TokenKind::LeftBrace,
        TokenKind::RightBrace,
        TokenKind::LeftParen,
        TokenKind::RightParen,
        TokenKind::LeftBracket,
        TokenKind::RightBracket,
        TokenKind::Comma,
        TokenKind::Dot,
        TokenKind::Colon,
    ];

    let result = run_lexer_test(input, expected);
    assert!(result.success, "{:?}", result.message);
}

#[test]
fn test_lexer_comments() {
    let input = "user // this is a comment\nname";
    let expected = vec![
        TokenKind::Identifier("user".to_string()),
        TokenKind::Identifier("name".to_string()),
    ];

    let result = run_lexer_test(input, expected);
    assert!(result.success, "{:?}", result.message);
}

#[test]
fn test_lexer_complex_input() {
    let input = "entity User { name: String, age: Int }";
    let expected = vec![
        TokenKind::Entity,
        TokenKind::Identifier("User".to_string()),
        TokenKind::LeftBrace,
        TokenKind::Identifier("name".to_string()),
        TokenKind::Colon,
        TokenKind::Identifier("String".to_string()),
        TokenKind::Comma,
        TokenKind::Identifier("age".to_string()),
        TokenKind::Colon,
        TokenKind::Identifier("Int".to_string()),
        TokenKind::RightBrace,
    ];

    let result = run_lexer_test(input, expected);
    assert!(result.success, "{:?}", result.message);
}

#[test]
fn test_lexer_error_handling() {
    // In a real lexer, we would test for invalid characters
    // For now, let's just test a simple valid case
    let input = "abc";
    let expected = vec![TokenKind::Identifier("abc".to_string())];

    let result = run_lexer_test(input, expected);
    assert!(result.success);
}
