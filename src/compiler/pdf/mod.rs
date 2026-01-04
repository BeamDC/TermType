use std::fs;
use std::path::Path;
use crate::compiler::parser::ast::{Ast, AstNode};
use printpdf::*;
use crate::{inches, load_fonts};

pub struct PdfData<'pd, P: AsRef<Path>> {
    pub name: &'pd str,
    pub path: P,
    pub width: Mm,
    pub height: Mm,
}

pub struct FontData {
    /// the `id` of the primary font used in rendering
    primary: FontId,
    /// the `id` of the bold font used in rendering
    bold: FontId,
    /// the `id` of the italic font used in rendering
    italic: FontId,
}

pub struct PdfRenderer<'pd, P: AsRef<Path>> {
    ast: Ast,
    meta: PdfData<'pd, P>,
    doc: PdfDocument,
    doc_cursor: (f32,f32), // (x, y) pair for the current coords of the drawing cursor
    pages: Vec<PdfPage>,
    fonts: FontData,
}

impl<'pd, P: AsRef<Path>> PdfRenderer<'pd, P> {
    const WIDTH: Mm = Mm(inches!(8.5));
    const HEIGHT: Mm = Mm(inches!(11));
    const MARGINS: Mm = Mm(10.0);

    /// construct a new [`PdfRenderer`] from the given ast.
    pub fn new(ast: Ast, meta: PdfData<'pd, P>) -> Self {
        let mut doc = PdfDocument::new(meta.name);
        let fonts = load_fonts!(
            load to: doc,
            primary: "../../../assets/fonts/victor_mono/VictorMono-Regular.ttf",
            bold: "../../../assets/fonts/victor_mono/VictorMono-Bold.ttf",
            italic: "../../../assets/fonts/victor_mono/VictorMono-Italic.ttf",
        );

        Self {
            ast,
            meta,
            doc,
            doc_cursor: (0.0, 0.0),
            pages: vec![],
            fonts,
        }
    }

    /// constructs a `.pdf` file from `self`, returning the bytes of the file
    pub fn render(&mut self) -> Vec<String> {
        while let Some(node) = self.ast.pop() {
            match node  {
                AstNode::Text { .. } => todo!(),
                AstNode::Block { .. } => todo!(),
                AstNode::Header { .. } => todo!(),
            }
        }

        let save_options = PdfSaveOptions {
            subset_fonts: true, // auto-subset fonts on save
            ..Default::default()
        };

        let mut warnings = Vec::new();
        let pdf_bytes: Vec<u8> = self.doc
            .with_pages(self.pages.clone())
            .save(&save_options, &mut warnings);

        fs::write("./output.pdf", &pdf_bytes)
            .expect("Unable to write file");

        warnings.iter().map(|w| w.msg.clone()).collect::<Vec<_>>()
    }
}