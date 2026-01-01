use std::path::Path;
use crate::compiler::parser::ast::Ast;
use printpdf::*;

pub struct PdfData<'pd, P: AsRef<Path>> {
    pub name: &'pd str,
    pub path: P,
}

pub struct PdfRenderer {
    ast: Ast,
}

impl PdfRenderer {
    /// construct a new [`PdfRenderer`] from the given ast.
    pub fn new(ast: Ast) -> Self {
        Self {
            ast
        }
    }

    /// constructs a `.pdf` file from `self`, returning the bytes of the file
    pub fn render<P: AsRef<Path>>(&mut self,  meta: PdfData<P>) {
        let mut doc = PdfDocument::new(meta.name);
        todo!()
    }
}