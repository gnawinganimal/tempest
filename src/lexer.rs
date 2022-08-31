
use logos::Logos;

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
pub enum Token {
    #[regex(" +")]
    Whitespace,

    #[regex("[A-Za-z][A-Za-z0-9]+")]
    Ident,

    #[regex("[0-9]+")]
    Num,

    #[token("#")]
    Hash,

    #[token("=")]
    Assign,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    #[error]
    Error,
}

pub fn lex(source: &str) -> Vec<Token> {
    Token::lexer(source).collect()
}
