use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub(crate) enum Token {
    #[regex(r"\s*/\*.*\*/\s", priority = 3)]
    Comment,
    #[regex(r"\s*/\*.*", priority = 2)]
    BeginComment,
    #[regex(r".*\*/\s*", priority = 1)]
    EndComment,
    #[regex(r".+", priority = 0)]
    Line,
    #[token(r"\n")]
    NewLine,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}
