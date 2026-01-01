use crate::parser::Parser;

mod parser;

fn main() {
    let src = r#"hello world :3"#;
    let mut parser = Parser::new(src);
    let ast = parser.parse();
    println!("{:#?}", ast);
}
