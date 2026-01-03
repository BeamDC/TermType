use std::collections::VecDeque;
use std::ops::{Index, IndexMut};
use crate::compiler::parser::text::Text;
use crate::compiler::parser::title::Title;

#[derive(Debug)]
pub struct NodeIndex(pub usize);

#[derive(Debug)]
pub enum AstNode {
    Text(Text),
    Block {
        title: Title,
        content: NodeIndex,
    }
}

#[derive(Debug)]
pub struct Ast(VecDeque<AstNode>);

impl Ast {
    pub fn new() -> Ast {
        Ast(VecDeque::new())
    }

    /// append a node to the ['Ast']
    #[inline]
    pub fn push(&mut self, node: AstNode) {
        self.0.push_back(node);
    }

    /// remove and return the next node in the [`Ast`]
    #[inline]
    pub fn pop(&mut self) -> Option<AstNode> {
        self.0.pop_front()
    }
}

impl Index<NodeIndex> for Ast {
    type Output = AstNode;

    fn index(&self, index: NodeIndex) -> &Self::Output {
        &self.0[index.0]
    }
}

impl IndexMut<NodeIndex> for Ast {
    fn index_mut(&mut self, index: NodeIndex) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}