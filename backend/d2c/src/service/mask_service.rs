use helpers::text_fragment::TextFragment;
use model::mask::Mask;
use repository::mask_repository_trait::MaskRepositoryTrait;
use std::sync::Arc;

pub struct MaskService {
    mask_repository: Arc<dyn MaskRepositoryTrait + Sync + Send>,
}

impl MaskService {
    pub fn new(mask_repository: Arc<dyn MaskRepositoryTrait + Sync + Send>) -> Self {
        Self { mask_repository }
    }

    pub async fn get_or_save(
        &self,
        user_id: String,
        fragment: &TextFragment,
    ) -> anyhow::Result<Mask> {
        let mask_option = self.get(user_id.clone(), fragment).await?;
        if let Some(mask) = mask_option {
            Ok(mask)
        } else {
            let mask = self.save(user_id, fragment).await?;
            Ok(mask)
        }
    }

    pub async fn save(&self, user_id: String, fragment: &TextFragment) -> anyhow::Result<Mask> {
        let new_mask = Mask::from_text(&fragment)?;
        self.mask_repository
            .create(user_id, new_mask.clone())
            .await?;
        Ok(new_mask)
    }

    pub async fn get(
        &self,
        user_id: String,
        fragment: &TextFragment,
    ) -> anyhow::Result<Option<Mask>> {
        match self.mask_repository.find_by_id(user_id).await? {
            Some(array) => {
                let mask = array
                    .iter()
                    .find(|mask| Some(fragment.text()) == mask.text().as_ref());
                Ok(mask.cloned())
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]

mod tests {
    use std::sync::Arc;

    use helpers::text_fragment::TextFragmentBuilder;
    use repository::sled_mask_repository::SledMaskRepository;

    use crate::service::mask_service::MaskService;
    #[tokio::test]
    async fn save() {
        let temp_dir = tempfile::tempdir().unwrap();
        let sled_repo =
            SledMaskRepository::new(sled::open(temp_dir.path().to_str().unwrap()).unwrap());

        let service = MaskService::new(Arc::new(sled_repo));

        let fragment = TextFragmentBuilder::default()
            .text("Hello")
            .category("TEST")
            .position((0, 0))
            .build()
            .unwrap();

        let mask = service.save("123".into(), &fragment).await.unwrap();

        assert_ne!(*mask.text(), None);
    }
    #[tokio::test]
    async fn get() {
        let user_id = "123".to_string();
        let temp_dir = tempfile::tempdir().unwrap();
        let sled_repo =
            SledMaskRepository::new(sled::open(temp_dir.path().to_str().unwrap()).unwrap());
        let service = MaskService::new(Arc::new(sled_repo));

        let fragment = TextFragmentBuilder::default()
            .text("Hello")
            .category("TEST")
            .position((0, 0))
            .build()
            .unwrap();

        let saved = service.save(user_id.clone(), &fragment).await.unwrap();

        let mask = service
            .get(user_id.clone(), &fragment)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(mask.text().as_ref().unwrap(), fragment.text());
        assert_eq!(mask.mask(), saved.mask());
    }
}
