use crate::compiler::parser::Parser;
use printpdf::*;

mod compiler;
mod util;

fn main() {
    let src = r#"hello world :3"#;
    let mut parser = Parser::new(src);
    let ast = parser.parse();
    println!("{:#?}", ast);

    // let metadata = PdfData {
    //     name: "pdf",
    //     path: "./output.pdf",
    //     width: Mm(inches!(8.5)),
    //     height: Mm(inches!(11)),
    // };
}
