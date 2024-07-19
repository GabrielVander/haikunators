pub trait AdjectiveRepository {
    fn fetch_all(&self) -> Result<Vec<String>, String>;
}
