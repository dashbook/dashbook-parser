use cells::Cell;
use logos::Logos;
use tokens::Token;

mod cells;
mod tokens;

pub fn parse(input: &str) -> Vec<Cell> {
    let mut lexer = Token::lexer(input);
    let mut output = Vec::new();
    let mut code = String::new();
    while let Some(token) = lexer.next() {
        match token {
            Token::BeginComment => {
                if !code.is_empty() {
                    output.push(Cell::Code(code));
                    code = String::new();
                }
                let mut comment = lexer
                    .slice()
                    .trim_start_matches(" ")
                    .trim_start_matches("/*")
                    .to_owned();
                comment.push_str("\n");
                while let Some(token) = lexer.next() {
                    match token {
                        Token::Line => {
                            comment.push_str(lexer.slice());
                            comment.push_str("\n");
                        }
                        Token::NewLine => comment.push_str(lexer.slice()),
                        Token::EndComment => {
                            comment.push_str(
                                lexer
                                    .slice()
                                    .trim_end_matches(" ")
                                    .trim_end_matches("\n")
                                    .trim_end_matches("*/"),
                            );
                            comment.push_str("\n");
                            break;
                        }
                        _ => (),
                    }
                }
                output.push(Cell::Comment(comment))
            }
            Token::Line => {
                code.push_str(lexer.slice());
                code.push_str("\n");
            }
            Token::NewLine => code.push_str(lexer.slice()),
            _ => (),
        }
    }
    if !code.is_empty() {
        output.push(Cell::Code(code));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cells() {
        let input = "        /*# Markdown headline
        Some cool comment
        Another cool comment*/
        let a = 5;
        a * 6
        ";

        let cells = parse(input);
        match &cells[0] {
            Cell::Comment(comment) => {
                assert_eq!(
                    comment,
                    "# Markdown headline\nSome cool comment\nAnother cool comment\n"
                )
            }
            _ => panic!(),
        }
        match &cells[1] {
            Cell::Code(comment) => {
                assert_eq!(comment, "let a = 5;\na * 6\n")
            }
            _ => panic!(),
        }
    }
}
