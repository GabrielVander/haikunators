use async_trait::async_trait;

#[async_trait]
pub trait AdjectiveRepository {
    async fn fetch_all(&self) -> Result<Vec<String>, String>;
}
