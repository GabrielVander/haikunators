pub trait AdjectiveRepository {
    async fn fetch_all(&self) -> Result<Vec<String>, String>;
}
