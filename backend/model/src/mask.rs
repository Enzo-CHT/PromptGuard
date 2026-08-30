use core::fmt;
use std::format;

use getset::Getters;
use helpers::text_fragment::TextFragment;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Getters, Serialize, Deserialize, Clone)]
/// Remplace un éléments de text tout en offrant une manière de l'identifier.
/// Un masque est un objet représentant une chaîne de caractère et dont le rôle
/// est de séparer l'information de context de l'information sensible.
///
/// ### Exemple
///
/// Si le masque couvre un prenom comme **Henry** alors il enregistre le text
/// et le remplace par ça catégorie, **I-PER** dans ce cas.
///
///
pub struct Mask {
    #[getset(get = "pub")]
    id: String,

    #[getset(get = "pub")]
    text: Option<String>,

    #[getset(get = "pub")]
    mask: String,
}

impl fmt::Display for Mask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.mask)
    }
}

impl Mask {
    /// Extrait l'identifiant contenu dans une chaîne de caractère représentant un masque.
    pub fn get_id_from_mask(mask: String) -> anyhow::Result<String> {
        let mask_length = mask.chars().count();
        let raw_mask: String = mask.chars().skip(1).take(mask_length - 2).collect();

        Ok(raw_mask
            .split(' ') // Sépare la catégorie de l'identifiant
            .nth(1) // Récupère l'identifiant
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Format invalide : impossible d'extraire l'id depuis \"{raw_mask}\""
                )
            })?
            .to_string())
    }

    /// Créer un objet `Mask` à partir des informations contenu dans l'objet `TextFragment`.
    /// La fonction créer un identifiant unique et l'ajoute à une chaine de caractère correctement formatée de sorte
    /// à représenter le masque.
    /// Le fragment en paramètre doit définir une catégorie.  
    pub fn from_fragment(fragment: &TextFragment) -> anyhow::Result<Mask> {
        let entity_categorie = fragment.category();
        let id: String = Uuid::new_v4().to_string().chars().take(5).collect();
        let text = Some(fragment.text().to_string());
        let mask = format!("[{entity_categorie} {}]", &id);

        Ok(Mask { id, text, mask })
    }
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, format};

    use helpers::text_fragment::TextFragmentBuilder;

    use crate::mask::Mask;

    /// Test la récupération d'un élément caché à partir d'un masque.
    /// Si le masque existe : les informations sont récupérées et comparées pour vérifier leurs valeurs
    /// Si le masque n'existe pas : la fonction renvoie une erreur en précisant les raisons de l'erreur
    #[test]
    fn test_get_id_from_mask() {
        let mask_string = String::from("[TEST 1234]");
        let id = Mask::get_id_from_mask(mask_string.clone()).unwrap();

        assert_eq!(*id, String::from("1234"));
    }

    /// Test de la création d'un nouveau masque à partir d'un fragment de text
    #[test]
    fn test_from_fragment() {
        let text_fragment = TextFragmentBuilder::default()
            .category("TEST")
            .text("Hello")
            .position((0, 0))
            .build()
            .unwrap();
        let mask = Mask::from_fragment(&text_fragment).unwrap();

        assert!(!mask.id().is_empty());
        assert_eq!(*mask.text(), Some(String::from("Hello")));
    }

    #[test]
    fn test_displau() {
        let mask = Mask {
            id: "123".into(),
            text: None,
            mask: "[TEST 123]".into(),
        };
        assert_eq!(format!("{mask}"), "[TEST 123]");
    }
}
