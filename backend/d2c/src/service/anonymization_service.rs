use std::ops::Range;

use derive_builder::Builder;
use getset::{Getters, Setters};
use helpers::recognized_fragment_list::RecognizedFragmentList;
use helpers::text_fragment::{TextFragment, TextFragmentBuilder};

#[derive(Debug, Default, Getters, Setters, Builder)]
pub struct AnonymizationService {}

impl AnonymizationService {
    pub fn mask_text(
        &self,
        text: String,
        recognized_fragment_list: RecognizedFragmentList,
        user_id: String,
    ) -> anyhow::Result<TextFragment> {
        let mut offset: isize = 0;
        let mut modified_text = text;
        for fragment in recognized_fragment_list.list() {
            let (start, end) = *fragment.position();
            let start = start as isize - 1;
            let end = end as isize;
            let fragment_length = end - start;

            let mask = "[MASK]"; // TODO: ajouter la création de masque 
            let mask_length = mask.len() as isize;

            let new_pos = (start - offset, end - offset);
            modified_text.replace_range(new_pos.0 as usize..new_pos.1 as usize, mask.into());
            offset += fragment_length - mask_length;
        }

        let text_fragment = TextFragmentBuilder::default()
            .fragment(modified_text.as_str())
            .position((0, modified_text.len() as u32))
            .build()?;

        Ok(text_fragment)
    }

    fn redact_image() {}

    fn unmask_text() {}
}

#[cfg(test)]
mod tests {

    use std::println;

    use helpers::recognized_fragment_list::RecognizedFragmentListBuilder;

    use super::*;

    #[test]
    fn mask_test() {
        let service = AnonymizationServiceBuilder::default().build().unwrap();

        let frag1 = TextFragmentBuilder::default()
            .fragment("Henry")
            .position((12, 16))
            .build()
            .unwrap();
        let frag2 = TextFragmentBuilder::default()
            .fragment("Cavill")
            .position((18, 23))
            .build()
            .unwrap();
        let recognized_fragment_list = RecognizedFragmentListBuilder::default()
            .list(vec![frag1, frag2])
            .build()
            .unwrap();

        let result_frag: TextFragment = service
            .mask_text(
                "Je m'appel Henry Cavill".into(),
                recognized_fragment_list,
                "00000000000".into(),
            )
            .unwrap();

        println!("{:?}", result_frag);

        assert_eq!(
            result_frag.fragment(),
            &"Je m'appel [MASK] [MASK]".to_string()
        );
    }
}
