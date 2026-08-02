use crate::mask_repository_trait::MaskRepositoryTrait;
use derive_builder::Builder;
use getset::Getters;
use model::mask::Mask;
use sled::Db;

#[derive(Debug, Builder, Getters, Clone)]
pub struct SledMaskRepository {
    #[getset(get = "pub")]
    db: Db,
}

#[async_trait::async_trait]
impl MaskRepositoryTrait for SledMaskRepository {
    async fn create(&self, mask: Mask) -> anyhow::Result<()> {
        let value = serde_json::to_vec(&mask)?;
        self.db.insert(mask.id().as_bytes(), value)?;
        Ok(())
    }
    async fn find_by_id(&self, id: String) -> anyhow::Result<Option<Mask>> {
        match self.db.get(id.as_bytes())? {
            Some(bytes) => {
                let mask: Mask = serde_json::from_slice(&bytes)?;
                Ok(Some(mask))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use model::mask::MaskBuilder;

    use crate::mask_repository_trait::MaskRepositoryTrait;
    use crate::sled_mask_repository::SledMaskRepositoryBuilder;

    #[tokio::test]
    async fn create_mask() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mask = MaskBuilder::default()
            .mask("[TEST]".into())
            .id("1".into())
            .text("Hello".into())
            .build()
            .unwrap();

        let repository = SledMaskRepositoryBuilder::default()
            .db(sled::open(temp_dir.path().to_str().unwrap()).unwrap())
            .build()
            .unwrap();

        assert!(repository.create(mask).await.is_ok());
    }

    #[tokio::test]
    async fn find_by_id() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mask = MaskBuilder::default()
            .mask("[TEST]".into())
            .id("1".into())
            .text("Hello".into())
            .build()
            .unwrap();

        let repository = SledMaskRepositoryBuilder::default()
            .db(sled::open(temp_dir.path().to_str().unwrap()).unwrap())
            .build()
            .unwrap();

        assert!(repository.create(mask).await.is_ok());

        let mask_option = repository.find_by_id(String::from("1")).await.unwrap();
        assert!(mask_option.is_some());
        let mask = mask_option.unwrap();
        assert_eq!(mask.mask(), "[TEST]");
        assert_eq!(mask.text(), "Hello");
    }
}
