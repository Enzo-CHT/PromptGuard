use derive_builder::Builder;
use helpers::filterlist::FilterList;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Builder, Serialize, Deserialize)]
pub struct TextRequestDto {
    pub text: String,
    pub user_id: String,
    pub filterlist: FilterList,
}
