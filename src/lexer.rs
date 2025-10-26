use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // keywords
    #[token("make")]
    Make,
    #[token("be")]
    Be,
    #[token("do")]
    Do,
    #[token("change")]
    Change,
    #[token("repeat")]
    Repeat,
    #[token("times")]
    Times,
    #[token("if")]
    If,
    #[token("say")]
    Say,

    // operators
    #[token("+")] Plus,
    #[token("-")] Minus,
    #[token("*")] Star,
    #[token("/")] Slash,
    #[token("%")] Percent,
    #[token("(")] LParen,
    #[token(")")] RParen,

    // literals
    #[regex(r"[0-9]+")]
    Number,
    #[regex(r#""[^"]*""#)]
    String,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    // skip whitespace
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}
