use crate::domain::repositories::{
    adjective_repository::AdjectiveRepository, noun_repository::NounRepository,
};
use rand::prelude::SliceRandom;

pub struct GenerateNameUseCase {
    adjective_repository: Box<dyn AdjectiveRepository>,
    noun_repository: Box<dyn NounRepository>,
}

impl GenerateNameUseCase {
    pub fn new(
        adjective_repository: Box<dyn AdjectiveRepository>,
        noun_repository: Box<dyn NounRepository>,
    ) -> Self {
        GenerateNameUseCase {
            adjective_repository,
            noun_repository,
        }
    }

    pub async fn execute(&self) -> Result<String, String> {
        let adjective: Result<String, String> = self.choose_adjective().await;
        let noun: Result<String, String> = self.choose_noun().await;

        adjective
            .and_then(|a| noun.map(|n| format!("{a}-{n}")))
            .map_err(|_| "Unable to generate name".to_string())
    }

    async fn choose_adjective(&self) -> Result<String, String> {
        self.retrieve_adjectives()
            .await
            .map_err(|_| "Unable to choose adjective".to_owned())
            .and_then(|a| {
                a.choose(&mut rand::thread_rng())
                    .cloned()
                    .ok_or("".to_string())
            })
    }

    async fn choose_noun(&self) -> Result<String, String> {
        self.retrieve_nouns()
            .await
            .map_err(|_| "Unable to choose noun".to_owned())
            .and_then(|n| {
                n.choose(&mut rand::thread_rng())
                    .cloned()
                    .ok_or("".to_string())
            })
    }

    async fn retrieve_adjectives(&self) -> Result<Vec<String>, String> {
        self.adjective_repository
            .fetch_all()
            .await
            .map_err(|_| "Unable to retrieve adjectives".to_owned())
    }

    async fn retrieve_nouns(&self) -> Result<Vec<String>, String> {
        self.noun_repository
            .fetch_all()
            .await
            .map_err(|_| "Unable to retrieve nouns".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use crate::domain::repositories::{
        adjective_repository::AdjectiveRepository, noun_repository::NounRepository,
    };

    use super::*;

    struct DummyAdjectiveRepository {
        fetch_all_result: Result<Vec<String>, String>,
    }

    impl DummyAdjectiveRepository {
        pub fn fetch_all_returns(result: Result<Vec<String>, String>) -> Self {
            DummyAdjectiveRepository {
                fetch_all_result: result,
            }
        }
    }

    #[async_trait]
    impl AdjectiveRepository for DummyAdjectiveRepository {
        async fn fetch_all(&self) -> Result<Vec<String>, String> {
            self.fetch_all_result.to_owned()
        }
    }

    struct DummyNounRepository {
        fetch_all_result: Result<Vec<String>, String>,
    }

    impl DummyNounRepository {
        pub fn fetch_all_returns(result: Result<Vec<String>, String>) -> Self {
            DummyNounRepository {
                fetch_all_result: result,
            }
        }
    }

    #[async_trait]
    impl NounRepository for DummyNounRepository {
        async fn fetch_all(&self) -> Result<Vec<String>, String> {
            self.fetch_all_result.to_owned()
        }
    }

    macro_rules! should_return_expected {
        ($($name:ident: ($a:expr,$n:expr,$e:expr))*) => {
        $(
            #[tokio::test]
            async fn $name() {
                let adjectives: Result<Vec<String>, String> = $a;
                let nouns:  Result<Vec<String>, String>= $n;
                let expected: Result<String, String> = $e;

                let adjective_repository: DummyAdjectiveRepository = DummyAdjectiveRepository::fetch_all_returns(adjectives);
                let noun_repository: DummyNounRepository = DummyNounRepository::fetch_all_returns(nouns);

                let use_case: GenerateNameUseCase =
                    GenerateNameUseCase::new(Box::new(adjective_repository), Box::new(noun_repository));

                let result: Result<String, String> = use_case.execute().await;

                assert_eq!(expected, result);
            }
        )*
        }

    }
    should_return_expected! {
        adjective_repository_fails: (Err("Failed!".to_string()), Ok(vec![
                "blizzard".to_string(),
                "sun".to_string(),
                "snow".to_string(),
            ]), Err("Unable to generate name".to_string()))
        noun_repository_fails: (Ok(vec![
                "great".to_string(),
                "terrific".to_string(),
                "awesome".to_string(),
            ]), Err("Failed!".to_string()), Err("Unable to generate name".to_string()))
        empty_adjectives_and_empty_nouns: (Ok(vec![]), Ok(vec![]), Err("Unable to generate name".to_string()))
        only_one_possible_combination_empty: (Ok(vec!["".to_string()]), Ok(vec!["".to_string()]), Ok("-".to_string()))
        only_one_possible_combination: (Ok(vec!["breezy".to_string()]), Ok(vec!["horse".to_string()]), Ok("breezy-horse".to_string()))
    }

    macro_rules! should_return_ok_with_one_of {
        ($($name:ident: ($a:expr,$n:expr,$e:expr))*) => {
        $(
            #[tokio::test]
            async fn $name() {
                let adjectives: Vec<String> = $a;
                let nouns:  Vec<String>= $n;
                let expected_possible_values: Vec<String> = $e;

                let adjective_repository: DummyAdjectiveRepository = DummyAdjectiveRepository::fetch_all_returns(Ok(adjectives));
                let noun_repository: DummyNounRepository = DummyNounRepository::fetch_all_returns(Ok(nouns));

                let use_case: GenerateNameUseCase =
                    GenerateNameUseCase::new(Box::new(adjective_repository), Box::new(noun_repository));

                for _ in 0..51 {
                    let result: Result<String, String> = use_case.execute().await;

                    assert_eq!(expected_possible_values.contains(&result.unwrap()), true);
                }
            }
        )*
        }

    }

    should_return_ok_with_one_of! {
        _1: (
                vec![
                    "great".to_string(),
                    "terrific".to_string(),
                    "awesome".to_string(),
                ],
                vec![
                    "blizzard".to_string(),
                    "sun".to_string(),
                    "snow".to_string(),
                ],
                vec![
                    "great-blizzard".to_string(),
                    "terrific-blizzard".to_string(),
                    "awesome-blizzard".to_string(),
                    "great-sun".to_string(),
                    "terrific-sun".to_string(),
                    "awesome-sun".to_string(),
                    "great-snow".to_string(),
                    "terrific-snow".to_string(),
                    "awesome-snow".to_string(),
                ]
            )
    }
}
