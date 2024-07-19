use crate::domain::repositories::adjective_repository::AdjectiveRepository;

pub struct AdjectiveRepositoryImpl {
    pub adjectives: Vec<String>,
}

impl AdjectiveRepositoryImpl {
    pub fn new() -> AdjectiveRepositoryImpl {
        let adjectives: Vec<String> = vec![
            "autumn",
            "hidden",
            "bitter",
            "misty",
            "silent",
            "empty",
            "dry",
            "dark",
            "summer",
            "icy",
            "delicate",
            "quiet",
            "white",
            "cool",
            "spring",
            "winter",
            "patient",
            "twilight",
            "dawn",
            "crimson",
            "wispy",
            "weathered",
            "blue",
            "Billowing",
            "broken",
            "cold",
            "damp",
            "falling",
            "frosty",
            "green",
            "long",
            "late",
            "lingering",
            "bold",
            "little",
            "morning",
            "muddy",
            "old",
            "red",
            "rough",
            "still",
            "small",
            "sparkling",
            "thrumming",
            "shy",
            "wandering",
            "withered",
            "wild",
            "black",
            "young",
            "holy",
            "solitary",
            "fragrant",
            "aged",
            "snowy",
            "proud",
            "floral",
            "restless",
            "divine",
            "polished",
            "ancient",
            "purple",
            "lively",
            "nameless",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

        AdjectiveRepositoryImpl { adjectives }
    }
}

impl AdjectiveRepository for AdjectiveRepositoryImpl {
    fn fetch_all(&self) -> Result<Vec<String>, String> {
        Ok(self.adjectives.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_all_should_return_expected() {
        let expected: Vec<String> = vec![
            "autumn",
            "hidden",
            "bitter",
            "misty",
            "silent",
            "empty",
            "dry",
            "dark",
            "summer",
            "icy",
            "delicate",
            "quiet",
            "white",
            "cool",
            "spring",
            "winter",
            "patient",
            "twilight",
            "dawn",
            "crimson",
            "wispy",
            "weathered",
            "blue",
            "Billowing",
            "broken",
            "cold",
            "damp",
            "falling",
            "frosty",
            "green",
            "long",
            "late",
            "lingering",
            "bold",
            "little",
            "morning",
            "muddy",
            "old",
            "red",
            "rough",
            "still",
            "small",
            "sparkling",
            "thrumming",
            "shy",
            "wandering",
            "withered",
            "wild",
            "black",
            "young",
            "holy",
            "solitary",
            "fragrant",
            "aged",
            "snowy",
            "proud",
            "floral",
            "restless",
            "divine",
            "polished",
            "ancient",
            "purple",
            "lively",
            "nameless",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

        let result: Result<Vec<String>, String> = AdjectiveRepositoryImpl::new().fetch_all();

        assert_eq!(result, Ok(expected));
    }
}
