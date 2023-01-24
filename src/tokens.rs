use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub(crate) enum Token<'a> {
    #[token("/*", priority = 2)]
    BeginComment,
    #[token("*/", priority = 1)]
    EndComment,
    #[regex(r"\S", priority = 0)]
    NonWhitespace(&'a str),
    #[token(r"\s*")]
    Whitespace(&'a str),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}
