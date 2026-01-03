use std::fs;
use std::path::Path;
use crate::compiler::parser::ast::{Ast, AstNode};
use printpdf::*;
use crate::{inches, load_fonts};
use crate::compiler::parser::text::Text;

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
    pages: Vec<PdfPage>,
    fonts: FontData,
}

impl<'pd, P: AsRef<Path>> PdfRenderer<'pd, P> {
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
            pages: vec![],
            fonts,
        }
    }

    /// renders plain text nodes
    fn render_text(&mut self, text: Text) {
        let width = Mm(inches!(8.5));
        let height = Mm(inches!(11));

        let text_pos = Point {
            x: Mm(10.0).into(),
            y: (height - Mm(10.0)).into(),
        }; // from bottom left

        // todo : check if there is room for the text on the current page, if so write it all.
        // otherwise, cut it off as far as it can go, create another page, and continue.
        // also handle text wrapping.

        let mut pages = vec![PdfPage::new(width, height, vec![
            Op::SetLineHeight { lh: Pt(12.0) },
            Op::SetWordSpacing { pt: Pt(0.0) }, // probably not needed, but ill leave it here
            Op::SetCharacterSpacing { multiplier: 1.0 },
            Op::SetFontSize { font: self.fonts.primary.clone(), size: Pt(12.0) },

            Op::StartTextSection,
            Op::SetTextCursor { pos: text_pos },
            Op::WriteText {
                items: vec![TextItem::Text("Lorem ipsum".to_string())],
                font: self.fonts.primary.clone(),
            },
            Op::AddLineBreak,

            Op::WriteText {
                items: vec![TextItem::Text("dolor sit amet".to_string())],
                font: self.fonts.primary.clone(),
            },
            Op::AddLineBreak,
            Op::EndTextSection,
        ])];
    }

    /// constructs a `.pdf` file from `self`, returning the bytes of the file
    pub fn render(&mut self) -> Vec<String> {
        while let Some(node) = self.ast.pop() {
            match node  {
                AstNode::Text(t) => self.render_text(t),
                AstNode::Block { .. } => todo!(),
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