use std::collections::HashMap;
use crate::lexer::token_kind::TokenKind;

pub fn is_keyword(identifier: &str) -> Option<TokenKind> {
    match identifier {
        "entity" => Some(TokenKind::Entity),
        "rule" => Some(TokenKind::Rule),
        "flow" => Some(TokenKind::Flow),
        "constraint" => Some(TokenKind::Constraint),
        "if" => Some(TokenKind::If),
        "then" => Some(TokenKind::Then),
        "else" => Some(TokenKind::Else),
        "true" => Some(TokenKind::True),
        "false" => Some(TokenKind::False),
        "sym" => Some(TokenKind::Sym),
        "num" => Some(TokenKind::Num),
        "bool" => Some(TokenKind::Bool),
        "vec" => Some(TokenKind::Vec),
        "ref" => Some(TokenKind::Ref),
        "ctx" => Some(TokenKind::Ctx),
        _ => None,
    }
}

pub fn get_all_keywords() -> HashMap<String, TokenKind> {
    let mut keywords = HashMap::new();
    keywords.insert("entity".to_string(), TokenKind::Entity);
    keywords.insert("rule".to_string(), TokenKind::Rule);
    keywords.insert("flow".to_string(), TokenKind::Flow);
    keywords.insert("constraint".to_string(), TokenKind::Constraint);
    keywords.insert("if".to_string(), TokenKind::If);
    keywords.insert("then".to_string(), TokenKind::Then);
    keywords.insert("else".to_string(), TokenKind::Else);
    keywords.insert("true".to_string(), TokenKind::True);
    keywords.insert("false".to_string(), TokenKind::False);
    keywords.insert("sym".to_string(), TokenKind::Sym);
    keywords.insert("num".to_string(), TokenKind::Num);
    keywords.insert("bool".to_string(), TokenKind::Bool);
    keywords.insert("vec".to_string(), TokenKind::Vec);
    keywords.insert("ref".to_string(), TokenKind::Ref);
    keywords.insert("ctx".to_string(), TokenKind::Ctx);
    keywords
}