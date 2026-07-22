use crate::text_fragment::TextFragment;
use derive_builder::Builder;
use getset::{Getters, Setters};

#[derive(Getters, Setters, Builder, Default, Debug)]
struct RecognizedFragmentList {
    #[getset(get = "pub", set = "pub")]
    list: Vec<TextFragment>,

    #[getset(get = "pub")]
    category: String,
}

impl RecognizedFragmentList {
    pub fn text(&self) -> String {
        self.list
            .iter()
            .map(|entry| entry.fragment().to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn position(&self) -> Option<(i32, i32)> {
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
            .fragment("Hello")
            .position((0, 5))
            .build()
            .unwrap();
        let another_fragment = TextFragmentBuilder::default()
            .fragment("World")
            .position((6, 11))
            .build()
            .unwrap();

        let mut recognized_fragement_list = RecognizedFragmentListBuilder::default()
            .list(vec![fragment.clone()])
            .category(String::from("TEST"))
            .build()
            .unwrap();

        assert_eq!(recognized_fragement_list.text(), "Hello");
        assert_eq!(recognized_fragement_list.position(), Some((0, 5)));

        recognized_fragement_list.set_list(vec![fragment, another_fragment]);

        assert_eq!(recognized_fragement_list.text(), "Hello World");
        assert_eq!(recognized_fragement_list.position(), Some((0, 11)));
    }
}
