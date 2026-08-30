use std::vec;

use crate::mask_repository_trait::MaskRepositoryTrait;
use model::mask::Mask;
use sled::Db;

#[derive(Debug, Clone)]
pub struct SledMaskRepository {
    db: Db,
}

#[async_trait::async_trait]
impl MaskRepositoryTrait for SledMaskRepository {
    async fn create(&self, id: String, mask: Mask) -> anyhow::Result<()> {
        let value = serde_json::to_vec(&vec![mask])?;
        self.db.insert(id.as_bytes(), value)?;
        Ok(())
    }

    async fn find_by_id(&self, id: String) -> anyhow::Result<Option<Vec<Mask>>> {
        match self.db.get(id.as_bytes())? {
            Some(bytes) => {
                let mask: Vec<Mask> = serde_json::from_slice(&bytes)?;
                Ok(Some(mask))
            }
            None => Ok(None),
        }
    }
}

impl SledMaskRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use model::mask::Mask;

    use crate::mask_repository_trait::MaskRepositoryTrait;
    use crate::sled_mask_repository::SledMaskRepository;

    #[tokio::test]
    async fn create_mask() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mask = Mask {
            id: "1".into(),
            text: Some(String::from("Hello")),
            mask: "[TEST]".into(),
        };

        let repository =
            SledMaskRepository::new(sled::open(temp_dir.path().to_str().unwrap()).unwrap());

        assert!(repository.create(String::from("1"), mask).await.is_ok());
    }

    #[tokio::test]
    async fn find_by_id() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mask = Mask {
            id: "1".into(),
            text: Some(String::from("Hello")),
            mask: "[TEST]".into(),
        };

        let repository =
            SledMaskRepository::new(sled::open(temp_dir.path().to_str().unwrap()).unwrap());

        assert!(repository.create(String::from("1"), mask).await.is_ok());

        let mask_option = repository.find_by_id(String::from("1")).await.unwrap();
        assert!(mask_option.is_some());
        let masks = mask_option.unwrap();
        assert_eq!(masks[0].mask(), "[TEST]");
        assert_eq!(*masks[0].text(), Some(String::from("Hello")));
    }
}
