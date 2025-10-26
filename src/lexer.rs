use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    #[regex(r"#.*")]
    Comment,

    #[regex(r"[0-9]+")]
    Number,

    #[regex(r#""[^"]*""#)]
    Text,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[token("make")] Make,
    #[token("be")] Be,
    #[token("do")] Do,
    #[token("change")] Change,
    #[token("to")] To,
    #[token("say")] Say,
    #[token("if")] If,
    #[token("repeat")] Repeat,
    #[token("times")] Times,

    #[token("+")] Plus,
    #[token("-")] Minus,
    #[token("*")] Star,
    #[token("/")] Slash,
    #[token("%")] Percent,
}
