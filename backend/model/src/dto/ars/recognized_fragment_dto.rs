use derive_builder::Builder;
use helpers::filterlist::FilterList;
use helpers::text_segmenter::TextSegmenter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Builder, Default, Deserialize, Serialize)]
pub struct RecognizedFragmentRequest {
    pub text_segmenter: TextSegmenter,
    #[builder(default)]
    pub entities_coordinates: Vec<(u32, u32)>,
    #[builder(default)]
    pub filterlist: FilterList,
    #[builder(default)]
    pub entities: Vec<String>,
    pub independant: bool,
}
