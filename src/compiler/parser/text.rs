use crate::compiler::parser::layout::Justify;

#[derive(Debug)]
pub struct Text {
    content: String,
    justify: Justify,
}

impl Text {
    pub fn new<S: ToString>(content: S) -> Self {
        Self {
            content: content.to_string(),
            justify: Justify::default(),
        }
    }

    pub fn justify(self, justify: Justify) -> Self {
        Self {
            content: self.content,
            justify,
        }
    }
}