use derive_builder::Builder;
use getset::{Getters, Setters};

#[derive(Getters, Setters, Builder, Default, PartialEq, Debug, Clone)]
#[builder(setter(into))]
pub struct TextFragment {
    #[getset(get = "pub")]
    fragment: String,

    #[getset(get = "pub")]
    position: (i32, i32),

    #[getset(get = "pub")]
    #[builder(default)]
    size: (i32, i32),
}
