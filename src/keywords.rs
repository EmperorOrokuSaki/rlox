use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::tokens::TokenType;

pub static KEYWORDS: Lazy<HashMap<&str, TokenType>> = Lazy::new(|| {
    let mut keywords : HashMap<&str, TokenType>= HashMap::new();
    keywords.insert("and", TokenType::And);
    keywords.insert("class", TokenType::Class);
    keywords.insert("else", TokenType::Else);
    keywords.insert("for", TokenType::For);
    keywords.insert("fun", TokenType::Fun);
    keywords.insert("if", TokenType::If);
    keywords.insert("nil", TokenType::Nil);
    keywords.insert("or", TokenType::Or);
    keywords.insert("print", TokenType::Print);
    keywords.insert("return", TokenType::Return);
    keywords.insert("super", TokenType::Super);
    keywords.insert("this", TokenType::This);
    keywords.insert("true", TokenType::True);
    keywords.insert("var", TokenType::Var);
    keywords.insert("while", TokenType::While);
    keywords
});