use helpers::recognized_fragment_list::RecognizedFragmentList;
use helpers::text_fragment::{TextFragment, TextFragmentBuilder};

use crate::service::mask_service::MaskService;

pub struct AnonymizationService {
    mask_service: MaskService,
}

impl AnonymizationService {
    pub fn new(mask_service: MaskService) -> Self {
        Self { mask_service }
    }

    pub async fn mask_text(
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

            let mask = self
                .mask_service
                .get_or_save(user_id.clone(), fragment)
                .await?;
            let mask_length = mask.mask().len() as isize;

            let new_pos = (start - offset, end - offset);
            modified_text.replace_range(new_pos.0 as usize..new_pos.1 as usize, mask.mask());
            offset += fragment_length - mask_length;
        }

        let text_fragment = TextFragmentBuilder::default()
            .text(modified_text.as_str())
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

    use regex::Regex;
    use repository::sled_mask_repository::SledMaskRepository;
    use std::sync::Arc;

    use crate::service::mask_service::MaskService;

    use super::*;

    #[tokio::test]
    async fn mask_test() {
        let temp_dir = tempfile::tempdir().unwrap();
        let sled_repo =
            SledMaskRepository::new(sled::open(temp_dir.path().to_str().unwrap()).unwrap());
        let service = AnonymizationService::new(MaskService::new(Arc::new(sled_repo)));

        let frag1 = TextFragmentBuilder::default()
            .text("Henry")
            .position((12, 16))
            .category("TEST")
            .build()
            .unwrap();
        let frag2 = TextFragmentBuilder::default()
            .text("Cavill")
            .position((18, 23))
            .category("TEST")
            .build()
            .unwrap();
        let recognized_fragment_list = RecognizedFragmentList::new(vec![frag1, frag2]);

        let result_frag: TextFragment = service
            .mask_text(
                "Je m'appel Henry Cavill".into(),
                recognized_fragment_list,
                "00000000000".into(),
            )
            .await
            .unwrap();

        println!("{:?}", result_frag);

        let pattern =
            Regex::new(r"^Je m'appel \[TEST [0-9a-fA-F-]{5}\] \[TEST [0-9a-fA-F-]{5}\]$").unwrap();

        assert!(
            pattern.is_match(result_frag.text()),
            "Format inattendu : {}",
            result_frag.text()
        );
    }
}
