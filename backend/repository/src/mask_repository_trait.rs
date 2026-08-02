use async_trait::async_trait;
use model::mask::Mask;

#[async_trait]
pub trait MaskRepositoryTrait: Send + Sync {
    async fn create(&self, mask: Mask) -> anyhow::Result<()>;
    async fn find_by_id(&self, id: String) -> anyhow::Result<Option<Mask>>;
}
