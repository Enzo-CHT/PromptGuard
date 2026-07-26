use derive_builder::Builder;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Getters, Setters, Builder, Default, PartialEq, Debug, Clone, Deserialize, Serialize)]
#[builder(setter(into))]
pub struct TextFragment {
    #[getset(get = "pub")]
    fragment: String,

    #[getset(get = "pub")]
    position: (u32, u32),

    #[getset(get = "pub", set = "pub")]
    #[builder(default)]
    category: String,

    #[getset(get = "pub")]
    #[builder(default)]
    size: (u32, u32),
}
