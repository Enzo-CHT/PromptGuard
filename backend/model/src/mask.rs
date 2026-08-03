use std::{any, format, os::raw, print};

use derive_builder::Builder;
use getset::Getters;
use helpers::text_fragment::TextFragment;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Builder, Getters, Serialize, Deserialize, Clone)]
pub struct Mask {
    #[getset(get = "pub")]
    id: String,
    #[getset(get = "pub")]
    mask: String,
    #[getset(get = "pub", set = "pub")]
    text: Option<String>,
}

impl Mask {
    pub fn from_mask(mask: String) -> anyhow::Result<Mask> {
        let mask_length = mask.len() as usize;
        let raw_mask: String = mask.chars().skip(1).take(mask_length - 2).collect();
        let id = raw_mask
            .split(' ')
            .nth(1)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Format invalide : impossible d'extraire l'id depuis \"{raw_mask}\""
                )
            })?
            .to_string();

        Ok(Mask {
            id,
            mask,
            text: None,
        })
    }

    pub fn from_text(fragment: &TextFragment) -> anyhow::Result<Mask> {
        let entity_categorie = fragment.category();
        let mask_id: String = Uuid::new_v4().to_string().chars().take(5).collect();
        let mask = MaskBuilder::default()
            .id(mask_id.clone())
            .text(Some(fragment.text().to_string()))
            .mask(format!("[{entity_categorie} {}]", mask_id))
            .build()?;

        Ok(mask)
    }
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, println};

    use helpers::text_fragment::{self, TextFragmentBuilder};

    use crate::mask::{Mask, MaskBuilder};

    #[test]
    fn from_mask() {
        let mask_string = String::from("[TEST 1234]");
        let mask = Mask::from_mask(mask_string.clone()).unwrap();

        assert_eq!(*mask.id(), String::from("1234"));
        assert_eq!(*mask.mask(), mask_string);
        assert_eq!(*mask.text(), None);
    }

    #[test]
    fn from_text() {
        let text_fragment = TextFragmentBuilder::default()
            .category("TEST")
            .text("Hello")
            .position((0, 0))
            .build()
            .unwrap();
        let mask = Mask::from_text(&text_fragment).unwrap();

        assert!(!mask.id().is_empty());
        assert_eq!(*mask.text(), Some(String::from("Hello")));
    }
}
