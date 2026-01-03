
/// load the fonts needed for pdf rendering
#[macro_export]
macro_rules! load_fonts {
    (
        load to: $doc:expr,
        primary: $primary:expr,
        bold: $bold:expr,
        italic: $italic:expr,
    ) => {{
        let primary = printpdf::PdfDocument::add_font(
            &mut $doc,
            &ParsedFont::from_bytes(&include_bytes!($primary).as_slice(), 0, &mut vec![]).unwrap()
        );

        let bold = printpdf::PdfDocument::add_font(
            &mut $doc,
            &ParsedFont::from_bytes(&include_bytes!($bold).as_slice(), 1, &mut vec![]).unwrap()
        );

        let italic = printpdf::PdfDocument::add_font(
            &mut $doc,
            &ParsedFont::from_bytes(&include_bytes!($italic).as_slice(), 2, &mut vec![]).unwrap()
        );

        $crate::compiler::pdf::FontData {
            primary,
            bold,
            italic,
        }
    }};
}