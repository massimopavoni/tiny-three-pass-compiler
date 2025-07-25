use std::fmt::Display;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Var(i32),
    Num(i32),
    OpenBracket,
    ClosedBracket,
    OpenParen,
    ClosedParen,
    Plus,
    Dash,
    Star,
    Slash,
}

#[derive(Debug, Default)]
pub struct Parser<'a> {
    program: &'a str,
    tokens: Vec<Token>,
    current: usize,
    ast: Ast,
}

impl Display for Parser<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parser state:\nprogram: {}\ntokens: {:?}\nast: {}",
            self.program, self.tokens, self.ast
        )
    }
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self, program: &'a str) -> Ast {
        self.program = program;
        self.tokenize();
        self.current = self
            .tokens
            .iter()
            .position(|&t| t == ClosedBracket)
            .expect("No closed bracket found")
            + 1;
        self.ast = self.parse_expression();

        self.ast.clone()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut vars = vec![];
        let mut tokens = vec![];
        let mut chars = self.program.chars();

        while let Some(c) = chars.clone().next() {
            match c {
                'a'..='z' | 'A'..='Z' => {
                    let mut word = chars.as_str();

                    while chars
                        .clone()
                        .next()
                        .map_or(false, |c| c.is_ascii_alphabetic())
                    {
                        chars.next();
                    }

                    word = &word[..word.len() - chars.as_str().len()];

                    tokens.push(if let Some(index) = vars.iter().position(|&v| v == word) {
                        Var(index as i32)
                    } else {
                        vars.push(word);
                        Var(vars.len() as i32 - 1)
                    });
                }
                '0'..='9' => {
                    let number = chars.as_str();

                    while chars.clone().next().map_or(false, |c| c.is_ascii_digit()) {
                        chars.next();
                    }

                    tokens.push(Num(number[..number.len() - chars.as_str().len()]
                        .parse()
                        .unwrap()));
                }
                ' ' => {
                    chars.next();
                }
                _ => {
                    tokens.push(match &chars.as_str()[..chars.next().unwrap().len_utf8()] {
                        "[" => OpenBracket,
                        "]" => ClosedBracket,
                        "(" => OpenParen,
                        ")" => ClosedParen,
                        "+" => Plus,
                        "-" => Dash,
                        "*" => Star,
                        "/" => Slash,
                        _ => unreachable!("Invalid character at this point in the program source"),
                    });
                }
            }
        }

        self.tokens = tokens;

        self.tokens.iter().copied().collect()
    }

    fn next_token(&self) -> Option<Token> {
        self.tokens.get(self.current).copied()
    }

    fn consume(&mut self) {
        self.current += 1;
    }

    fn parse_expression(&mut self) -> Ast {
        let mut node = self.parse_factor();

        while let Some(token) = self.next_token() {
            match token {
                Plus => {
                    self.consume();
                    node = binop!(+, node, self.parse_factor());
                }
                Dash => {
                    self.consume();
                    node = binop!(-, node, self.parse_factor());
                }
                _ => break,
            }
        }

        node
    }

    fn parse_factor(&mut self) -> Ast {
        let mut node = self.parse_atom();

        while let Some(token) = self.next_token() {
            match token {
                Star => {
                    self.consume();
                    node = binop!(*, node, self.parse_atom());
                }
                Slash => {
                    self.consume();
                    node = binop!(/, node, self.parse_atom());
                }
                _ => break,
            }
        }

        node
    }

    fn parse_atom(&mut self) -> Ast {
        match self.next_token().unwrap() {
            Var(i) => {
                self.consume();
                arg!(i)
            }
            Num(n) => {
                self.consume();
                imm!(n)
            }
            OpenParen => {
                self.consume();
                let expression = self.parse_expression();
                self.consume();
                expression
            }
            _ => unreachable!("Invalid token at this point in the program tokens"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let prog = "[ x y z ] ( 2*3*x + 5*y - 3*z ) / (1 + 3 + 2*2)";
        let expected = binop!(/, binop!(-, binop!(+, binop!(*, binop!(*, imm!(2), imm!(3)), arg!(0)), binop!(*, imm!(5), arg!(1))), binop!(*, imm!(3), arg!(2))), binop!(+, binop!(+, imm!(1), imm!(3)), binop!(*, imm!(2), imm!(2))));

        let mut parser = Parser::default();
        let actual = parser.parse(prog);
        
        assert_eq!(actual, expected, "Parse");
    }
}
