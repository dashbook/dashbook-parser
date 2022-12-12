use logos::Logos;
use tokens::Token;

mod tokens;

wit_bindgen_guest_rust::generate!("dashbook-parser.wit");

export_dashbook_parser!(Component);
struct Component;

impl dashbook_parser::DashbookParser for Component {
    fn parse(input: String) -> Vec<dashbook_parser::Cell> {
        let mut lexer = Token::lexer(&input);
        let mut output = Vec::new();
        let mut code = String::new();
        while let Some(token) = lexer.next() {
            match token {
                Token::Comment => {
                    if !code.is_empty() {
                        output.push(dashbook_parser::Cell::Code(code));
                        code = String::new();
                    }
                    let mut comment = lexer
                        .slice()
                        .trim_start_matches(" ")
                        .trim_start_matches("/*")
                        .trim_end_matches(" ")
                        .trim_end_matches("\n")
                        .trim_end_matches("*/")
                        .to_owned();
                    comment.push_str("\n");
                    output.push(dashbook_parser::Cell::Comment(comment))
                }
                Token::BeginComment => {
                    if !code.is_empty() {
                        output.push(dashbook_parser::Cell::Code(code));
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
                    output.push(dashbook_parser::Cell::Comment(comment))
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
            output.push(dashbook_parser::Cell::Code(code));
        }
        output
    }
    fn generate(cells: Vec<dashbook_parser::Cell>) -> String {
        cells
            .into_iter()
            .map(|x| match x {
                dashbook_parser::Cell::Comment(comment) => {
                    "/*".to_string() + &comment.trim_end_matches("\n") + "*/" + "\n"
                }
                dashbook_parser::Cell::Code(code) => code,
            })
            .fold(String::new(), |mut acc, x| {
                acc.push_str(&x);
                acc
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dashbook_parser::{self, DashbookParser},
        Component,
    };

    #[test]
    fn test_cells() {
        let input = "/*# Markdown headline
Some cool comment
Another cool comment*/
let a = 5;
a * 6
"
        .to_owned();

        let cells = Component::parse(input.clone());
        match &cells[0] {
            dashbook_parser::Cell::Comment(comment) => {
                assert_eq!(
                    comment,
                    "# Markdown headline\nSome cool comment\nAnother cool comment\n"
                )
            }
            _ => panic!(),
        }
        match &cells[1] {
            dashbook_parser::Cell::Code(comment) => {
                assert_eq!(comment, "let a = 5;\na * 6\n")
            }
            _ => panic!(),
        }

        let output = Component::generate(cells);

        assert_eq!(input, output)
    }
}
