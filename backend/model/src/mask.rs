use derive_builder::Builder;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Builder, Getters, Serialize, Deserialize, Clone)]
pub struct Mask {
    #[getset(get = "pub")]
    id: String,
    #[getset(get = "pub")]
    mask: String,
    #[getset(get = "pub")]
    text: String,
}
