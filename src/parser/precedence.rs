use crate::lexer::token_kind::TokenKind;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    None,
    Assignment,    // =
    Or,            // ||
    And,           // &&
    Equality,      // == !=
    Comparison,    // < > <= >=
    Term,          // + -
    Factor,        // * / %
    Unary,         // ! -
    Call,          // . ()
    Primary,
}

impl Precedence {
    pub fn from_token(token: &TokenKind) -> Self {
        match token {
            TokenKind::Equal => Precedence::Assignment,
            TokenKind::PipePipe => Precedence::Or,
            TokenKind::AmpersandAmpersand => Precedence::And,
            TokenKind::EqualEqual | TokenKind::BangEqual => Precedence::Equality,
            TokenKind::Greater | TokenKind::GreaterEqual |
            TokenKind::Less | TokenKind::LessEqual => Precedence::Comparison,
            TokenKind::Plus | TokenKind::Minus => Precedence::Term,  // Both binary and unary, but binary has lower precedence
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Precedence::Factor,
            TokenKind::Bang => Precedence::Unary,  // Only unary operators
            TokenKind::Dot | TokenKind::LeftParen | TokenKind::LeftBracket => Precedence::Call,
            _ => Precedence::None,
        }
    }
}