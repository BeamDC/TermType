use std::ops::{Index, IndexMut};
use crate::parser::text::Text;
use crate::parser::title::Title;

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
pub struct Ast(Vec<AstNode>);

impl Ast {
    pub fn new() -> Ast {
        Ast(vec![])
    }

    /// append a node to the ['Ast']
    #[inline]
    pub fn push(&mut self, node: AstNode) {
        self.0.push(node);
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