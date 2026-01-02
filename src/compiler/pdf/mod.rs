use std::fs;
use std::path::Path;
use crate::compiler::parser::ast::Ast;
use printpdf::*;
use crate::inches;

pub struct PdfData<'pd, P: AsRef<Path>> {
    pub name: &'pd str,
    pub path: P,
}

pub struct PdfRenderer {
    ast: Ast,
    // font: ParsedFont
}

impl PdfRenderer {
    /// construct a new [`PdfRenderer`] from the given ast.
    pub fn new(ast: Ast) -> Self {
        Self {
            ast
        }
    }

    /// constructs a `.pdf` file from `self`, returning the bytes of the file
    pub fn render<P: AsRef<Path>>(&mut self,  meta: PdfData<P>) -> Vec<String> {
        let mut doc = PdfDocument::new(meta.name);

        let bytes = include_bytes!("../../../assets/fonts/victor_mono/VictorMono-Regular.ttf");
        let font_index = 0;
        let mut warnings = Vec::new();
        let font = ParsedFont::from_bytes(&bytes.as_slice(), font_index, &mut warnings).unwrap();
        let font_id = doc.add_font(&font);

        let text_pos = Point {
            x: Mm(10.0).into(),
            y: Mm(100.0).into(),
        }; // from bottom left

        let mut pages = vec![PdfPage::new(Mm(inches!(8.5)), Mm(inches!(11)), vec![
            Op::SetLineHeight { lh: Pt(33.0) },
            Op::SetWordSpacing { pt: Pt(33.0) },
            Op::SetCharacterSpacing { multiplier: 10.0 },
            Op::SetTextCursor { pos: text_pos },
            Op::WriteText {
                items: vec![TextItem::Text("Lorem ipsum".to_string())],
                font: font_id.clone(),
            },
            Op::AddLineBreak,
            Op::WriteText {
                items: vec![TextItem::Text("dolor sit amet".to_string())],
                font: font_id.clone(),
            },
            Op::AddLineBreak,
        ])];

        let save_options = PdfSaveOptions {
            subset_fonts: true, // auto-subset fonts on save
            ..Default::default()
        };

        let mut warnings = Vec::new();
        let pdf_bytes: Vec<u8> = doc
            .with_pages(pages)
            .save(&save_options, &mut warnings);

        fs::write("./output.pdf", &pdf_bytes)
            .expect("Unable to write file");

        warnings.iter().map(|w| w.msg.clone()).collect::<Vec<_>>()
    }
}