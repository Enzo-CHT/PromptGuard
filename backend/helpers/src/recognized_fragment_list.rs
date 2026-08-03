use crate::text_fragment::TextFragment;
use derive_builder::Builder;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Getters, Setters, Builder, Default, Debug, Deserialize, Serialize)]
pub struct RecognizedFragmentList {
    #[getset(get = "pub", set = "pub")]
    list: Vec<TextFragment>,
}

impl RecognizedFragmentList {
    pub fn text(&self) -> String {
        self.list
            .iter()
            .map(|entry| entry.text().to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn position(&self) -> Option<(u32, u32)> {
        if self.list.is_empty() {
            return None;
        }

        let start = self.list[0].position().0;
        let end = self.list.last().unwrap().position().1;
        Some((start, end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text_fragment::TextFragmentBuilder;

    #[test]
    fn test_get_full_text() {
        let fragment = TextFragmentBuilder::default()
            .text("Hello")
            .position((0, 5))
            .build()
            .unwrap();
        let another_fragment = TextFragmentBuilder::default()
            .text("World")
            .position((6, 11))
            .build()
            .unwrap();

        let mut recognized_fragment_list = RecognizedFragmentListBuilder::default()
            .list(vec![fragment.clone()])
            .build()
            .unwrap();

        assert_eq!(recognized_fragment_list.text(), "Hello");
        assert_eq!(recognized_fragment_list.position(), Some((0, 5)));

        recognized_fragment_list.set_list(vec![fragment, another_fragment]);

        assert_eq!(recognized_fragment_list.text(), "Hello World");
        assert_eq!(recognized_fragment_list.position(), Some((0, 11)));
    }
}
