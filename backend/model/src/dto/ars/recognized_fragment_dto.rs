use axum::extract::FromRequest;
use helpers::filterlist::FilterList;
use helpers::text_segmenter::TextSegmenter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecognizedFragmentRequest {
    pub text_segmenter: TextSegmenter,
    pub entities_coordinates: Vec<(u32, u32)>,
    pub filterlist: FilterList,
    pub entities: Vec<String>,
    pub independant: bool,
}
