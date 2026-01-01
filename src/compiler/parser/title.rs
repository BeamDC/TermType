use crate::compiler::parser::layout::{Border, Justify};

#[derive(Debug)]
pub struct Title {
    text: String,
    border: Border,
    justify: Justify,
}

impl Title {
    #[inline]
    pub fn new<S: ToString>(text: S) -> Self {
        Self {
            text: text.to_string(),
            border: Border::default(),
            justify: Justify::default(),
        }
    }

    #[inline]
    pub fn border(self, border: Border) -> Self {
        Self { border, ..self }
    }

    #[inline]
    pub fn justify(self, justify: Justify) -> Self {
        Self { justify, ..self }
    }
}