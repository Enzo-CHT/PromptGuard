use std::{collections::HashMap, error::Error};

use crate::text_fragment::{TextFragment, TextFragmentBuilder, TextFragmentBuilderError};

use getset::{Getters, Setters};
use image::open;
use imageproc::contrast::{ThresholdType, threshold};
use rusty_tesseract::{Args, Image, image_to_data};
use serde::{Deserialize, Serialize};

#[derive(Debug, Getters, Setters, Clone, Deserialize, Serialize, Default)]
pub struct TextSegmenter {
    #[getset(get = "pub")]
    text: String,

    #[getset(get = "pub")]
    fragments: Vec<TextFragment>,
}

impl TextSegmenter {
    /// Créer un segment de text à partir du chemin pointant vers une image.
    /// L'image est importée et traitée via Tesseract OCR pour en extraire le text.
    pub fn from_image(path: &str) -> Result<TextSegmenter, Box<dyn Error>> {
        let img = Image::from_path(path)?;
        let args = Args {
            lang: "fra+eng".into(),
            config_variables: HashMap::new(),
            dpi: None,
            psm: Some(6),
            oem: Some(3),
        };

        let data = image_to_data(&img, &args)?;
        let mut fragments = Vec::<TextFragment>::new();
        for entry in &data.data {
            let text = entry.text.trim().to_string();

            if text.is_empty() {
                continue;
            }

            fragments.push(
                TextFragmentBuilder::default()
                    .text(text)
                    .position((entry.width as u32, entry.height as u32))
                    .build()?,
            );
        }

        
        Ok(TextSegmenter {
            text: data
                .data
                .iter()
                .map(|entry| entry.text.trim().to_owned()) // Supprime les espaces en trop
                .filter(|s| !s.is_empty()) // Supprime les éléments vides
                .collect::<Vec<_>>()
                .join(" "),
            fragments: fragments,
        })
    }

    pub fn from_text(text: String) -> Result<TextSegmenter, TextFragmentBuilderError> {
        let mut start_index: usize = 0;
        let mut end_index: usize = 0;
        let mut fragments: Vec<TextFragment> = vec![];

        for token in text.trim().split(" ") {
            end_index = start_index + token.len();
            fragments.push(
                TextFragmentBuilder::default()
                    .text(token)
                    .position((start_index as u32, end_index as u32))
                    .build()?,
            );

            // Ajout 1 pour prendre en compte l'espace
            start_index = end_index + 1;
        }

        return Ok(TextSegmenter {
            text: text,
            fragments: fragments,
        });
    }

    /// Applique un ensemble de préprocessus pour augmenter la qualité de détection du text dans l'image. 
    fn preprocess_image(path: &str, save_path: &str) -> Result<(), image::ImageError> {
        let img = open(path)?;
        let gray = img.to_luma8();
        let binary = threshold(&gray, 128, ThresholdType::Binary);
        binary.save(save_path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use config::AppConfig;

    use super::*;

    #[test]
    fn test_from_text() {
        let string_hello_world = String::from("Hello World");

        // Test de création
        let segmenter = TextSegmenter::from_text(string_hello_world.clone()).unwrap();
        assert_eq!(segmenter.text(), &string_hello_world);

        let hello = TextFragmentBuilder::default()
            .text("Hello")
            .position((0, 5))
            .build()
            .unwrap();
        let world = TextFragmentBuilder::default()
            .text("World")
            .position((6, 11))
            .build()
            .unwrap();

        // Vérification des données
        assert_eq!(segmenter.fragments()[0].text(), &String::from("Hello"));
        assert_eq!(segmenter.fragments()[1].text(), &String::from("World"));

        // Vérification des positions
        let hello_pos = segmenter.fragments()[0].position();
        let word_hello: String = string_hello_world
            .chars()
            .skip(hello_pos.0 as usize)
            .take(hello_pos.1 as usize)
            .collect();
        assert_eq!(word_hello, String::from("Hello"));

        let world_pos = segmenter.fragments()[1].position();
        let word_world: String = string_hello_world
            .chars()
            .skip(world_pos.0 as usize)
            .take(world_pos.1 as usize)
            .collect();

        assert_eq!(word_world, String::from("World"));

        // Vérification du contenu
        assert_eq!(segmenter.fragments(), &vec![hello, world]);
    }

    #[test]
    fn test_from_image() {
        let test_config = AppConfig::load("../config/properties.yml")
            .expect("impossible de charger la configuration");

        let img_path = PathBuf::from(std::env::var("PROJECT_HOME").unwrap())
            .join(test_config.resources.tests.assets.as_str())
            .join("images")
            .join("test_ocr_image.png");

        let img_preprocess_path = PathBuf::from(std::env::var("PROJECT_HOME").unwrap())
            .join(test_config.resources.tests.assets.as_str())
            .join("images")
            .join("test_ocr_image_save.png");

        TextSegmenter::preprocess_image(
            img_path.to_str().unwrap(),
            img_preprocess_path.to_str().unwrap(),
        )
        .unwrap();

        // Vérifie que l'OCR fonctionne correctement
        let segmenter = TextSegmenter::from_image(img_preprocess_path.to_str().unwrap()).unwrap();
        assert_eq!(segmenter.text(), "Noisy image to test Tesseract OCR");

        // Vérifie que les fragments sont bien définis
        for fragment in segmenter.fragments() {
            assert!(
                String::from("Noisy image to test Tesseract OCR")
                    .split(" ")
                    .any(|w| w == fragment.text())
            )
        }
    }
}
