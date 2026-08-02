use std::error::Error;

use anyhow;
use helpers::recognized_fragment_list::{RecognizedFragmentList, RecognizedFragmentListBuilder};
use helpers::text_fragment::{TextFragment, TextFragmentBuilder};
use helpers::text_segmenter::TextSegmenter;
use rust_bert::pipelines::ner::NERModel;

pub struct RecognizerService {
    model: NERModel,
}

impl RecognizerService {
    pub fn new() -> anyhow::Result<RecognizerService> {
        Ok(RecognizerService {
            model: NERModel::new(Default::default())?,
        })
    }

    pub fn analyze(
        &self,
        segmenter: TextSegmenter,
        score_threshold: f64,
    ) -> Result<RecognizedFragmentList, Box<dyn Error + Send + Sync>> {
        let output = self.model.predict(&vec![segmenter.text()]);

        let mut list: Vec<TextFragment> = vec![];
        for entity in output[0].iter() {
            if entity.score < score_threshold {
                continue;
            }

            let fragment = TextFragmentBuilder::default()
                .fragment(entity.word.clone())
                .category(entity.label.clone())
                .position((entity.offset.begin, entity.offset.end))
                .build()?;

            list.push(fragment);
        }

        Ok(RecognizedFragmentListBuilder::default()
            .list(list)
            .build()?)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let segmenter =
            TextSegmenter::from_text(String::from("Bonjour, je m'appelle Théodore")).unwrap();
        let service = RecognizerService::new().unwrap();

        let output = service.analyze(segmenter, 0.1).unwrap();
        for label in output.list() {
            println!("{} - {}", label.fragment(), label.category());
        }
    }
}
