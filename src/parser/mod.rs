use crate::parser::ast::{Ast, AstNode};
use crate::parser::text::Text;

mod ast;
mod title;
mod text;
mod layout;

#[derive(Debug)]
pub struct Parser<'p> {
    raw: &'p str,
    start: usize,
    pos: usize,
}

impl<'p> Parser<'p> {
    pub fn new(raw: &'p str) -> Self {
        Parser {
            raw,
            start: 0,
            pos: 0,
        }
    }

    /// peek the next char in the input
    #[inline]
    fn peek(&self) -> Option<char> {
        self.raw.chars().nth(self.pos)
    }

    /// continues to advance the parser until the predicate is false.
    /// returns the resulting substring of the input, and brings the
    /// start position to the current position in the input
    fn consume_while<F: Fn(char) -> bool>(&mut self, predicate: F) -> String {
        while let Some(c) = self.peek() {
            if !predicate(c) {
                break;
            }
            self.pos += 1;
        }
        let res = self.raw[self.start..self.pos].to_owned();
        self.start = self.pos;
        res
    }

    fn next(&mut self) -> Option<AstNode> {
        let next = match self.peek() {
            Some(_) => {
                AstNode::Text(Text::new(
                    self.consume_while(|c: char| c != '\\'),
                ))
            }
            None => return None
        };
        Some(next)
    }

    pub fn parse(&mut self) -> Ast {
        let mut ast = Ast::new();
        while let Some(node) = self.next() {
            ast.push(node);
        }
        ast
    }
}