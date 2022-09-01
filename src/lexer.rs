
use logos::Logos;

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
pub enum Token<'a> {
    #[regex(" +")]
    Whitespace,

    #[regex("[_A-Za-z][_A-Za-z0-9]*")]
    Ident(&'a str),

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Num(f64),

    #[token("true")]
    True,
    #[token("false")]
    False,

    #[token("#")]
    Hash,

    #[token("=")]
    Assign,
    #[token(":")]
    Colon,
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

#[cfg(test)]
mod tests {
    use super::{*, Token::*};

    #[test]
    fn lex_ident() {
        assert_eq!(lex("hello_world"), vec![Ident("hello_world")]);
    }

    #[test]
    fn lex_assignment() {

    }
}
