use derive_builder::Builder;
use getset::Getters;
use helpers::filterlist::FilterList;

#[derive(Getters, Builder, Clone, Default)]
pub struct TextRequestEntity {
    #[getset(get = "pub")]
    text: String,

    #[getset(get = "pub")]
    mapping_id: String,

    #[getset(get = "pub")]
    to_revert: bool,

    #[getset(get = "pub")]
    filtermap: FilterList,
}
