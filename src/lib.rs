use logos::Logos;
use tokens::Token;

mod tokens;

wit_bindgen_guest_rust::generate!("dashbook-parser");

export_dashbook_parser!(Component);
struct Component;

#[derive(Debug)]
enum State {
    Comment(String),
    Code(String),
}

impl dashbook_parser::DashbookParser for Component {
    fn parse(input: String) -> Vec<dashbook_parser::Cell> {
        let lexer = Token::lexer(&input);
        let mut output = Vec::new();
        lexer.fold(State::Code(String::new()), |state, token| {
            match (state, token) {
                (State::Code(code), Token::BeginComment) => {
                    if !code.is_empty() {
                        output.push(dashbook_parser::Cell {
                            cell_type: dashbook_parser::CellType::Code,
                            size: 4,
                            source: code,
                            outputs: Vec::new(),
                        });
                    }
                    State::Comment(String::new())
                }
                (State::Code(mut code), Token::NonWhitespace(content)) => {
                    code.push_str(content);
                    State::Code(code)
                }
                (State::Code(mut code), Token::Whitespace(content)) => {
                    code.push_str(content);
                    State::Code(code)
                }
                (State::Comment(comment), Token::EndComment) => {
                    output.push(dashbook_parser::Cell {
                        cell_type: dashbook_parser::CellType::Markdown,
                        size: 4,
                        source: comment,
                        outputs: Vec::new(),
                    });
                    State::Code(String::new())
                }
                (State::Comment(mut comment), Token::NonWhitespace(content)) => {
                    comment.push_str(content);
                    State::Comment(comment)
                }
                (State::Comment(mut comment), Token::Whitespace(content)) => {
                    comment.push_str(content);
                    State::Comment(comment)
                }
                (state, _) => state,
            }
        });
        output
    }
    fn generate(cells: Vec<dashbook_parser::Cell>) -> String {
        cells
            .into_iter()
            .map(|x| match x.cell_type {
                dashbook_parser::CellType::Markdown => {
                    "/*".to_string() + &x.source.trim_end_matches("\n") + "*/" + "\n"
                }
                dashbook_parser::CellType::Code => x.source,
                dashbook_parser::CellType::Query => x.source,
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
        match &cells[0].cell_type {
            dashbook_parser::CellType::Markdown => {
                assert_eq!(
                    &cells[0].source,
                    "# Markdown headline\nSome cool comment\nAnother cool comment\n"
                )
            }
            _ => panic!(),
        }
        match &cells[1].cell_type {
            dashbook_parser::CellType::Code => {
                assert_eq!(&cells[1].source, "let a = 5;\na * 6\n")
            }
            _ => panic!(),
        }

        let output = Component::generate(cells);

        assert_eq!(input, output)
    }

    //     #[test]
    //     fn test_comment_config() {
    //         let input = "/*%%{size: 4}%%
    // #Markdown headline*/
    // "
    //         .to_owned();

    //         let cells = Component::parse(input.clone());
    //         match &cells[0].cell_type {
    //             dashbook_parser::CellType::Markdown => {
    //                 assert_eq!(
    //                     &cells[0].source,
    //                     "# Markdown headline\nSome cool comment\nAnother cool comment\n"
    //                 )
    //             }
    //             _ => panic!(),
    //         }
    //         match &cells[1].cell_type {
    //             dashbook_parser::CellType::Code => {
    //                 assert_eq!(&cells[1].source, "let a = 5;\na * 6\n")
    //             }
    //             _ => panic!(),
    //         }

    //         let output = Component::generate(cells);

    //         assert_eq!(input, output)
    //     }

    //     #[test]
    //     fn test_code_config() {
    //         let input = "/*%%{size: 4}%%*/
    // let a = 5;
    // a * 6
    // "
    //         .to_owned();

    //         let cells = Component::parse(input.clone());
    //         match &cells[0].cell_type {
    //             dashbook_parser::CellType::Markdown => {
    //                 assert_eq!(
    //                     &cells[0].source,
    //                     "# Markdown headline\nSome cool comment\nAnother cool comment\n"
    //                 )
    //             }
    //             _ => panic!(),
    //         }
    //         match &cells[1].cell_type {
    //             dashbook_parser::CellType::Code => {
    //                 assert_eq!(&cells[1].source, "let a = 5;\na * 6\n")
    //             }
    //             _ => panic!(),
    //         }

    //         let output = Component::generate(cells);

    //         assert_eq!(input, output)
    //     }
}
