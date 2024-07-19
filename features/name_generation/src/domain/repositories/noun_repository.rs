pub trait NounRepository {
    async fn fetch_all(&self) -> Result<Vec<String>, String>;
}
