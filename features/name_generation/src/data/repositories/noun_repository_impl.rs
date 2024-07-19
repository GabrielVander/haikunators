use crate::domain::repositories::noun_repository::NounRepository;

#[derive(Debug)]
struct NounRepositoryImpl {
    nouns: Vec<String>,
}

impl NounRepositoryImpl {
    pub fn new() -> Self {
        NounRepositoryImpl {
            nouns: vec![
                "waterfall",
                "river",
                "breeze",
                "moon",
                "rain",
                "wind",
                "sea",
                "morning",
                "snow",
                "lake",
                "sunset",
                "pine",
                "shadow",
                "leaf",
                "dawn",
                "glitter",
                "forest",
                "hill",
                "cloud",
                "meadow",
                "sun",
                "glade",
                "bird",
                "brook",
                "butterfly",
                "bush",
                "dew",
                "dust",
                "field",
                "fire",
                "flower",
                "firefly",
                "feather",
                "grass",
                "haze",
                "mountain",
                "night",
                "pond",
                "darkness",
                "snowflake",
                "silence",
                "sound",
                "sky",
                "shape",
                "surf",
                "thunder",
                "violet",
                "water",
                "wildflower",
                "wave",
                "water",
                "resonance",
                "sun",
                "log",
                "dream",
                "cherry",
                "tree",
                "fog",
                "frost",
                "voice",
                "paper",
                "frog",
                "smoke",
                "star",
            ]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        }
    }
}

impl NounRepository for NounRepositoryImpl {
    async fn fetch_all(&self) -> Result<Vec<String>, String> {
        Ok(self.nouns.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_all_should_return_expected() {
        let expected: Vec<String> = vec![
            "waterfall",
            "river",
            "breeze",
            "moon",
            "rain",
            "wind",
            "sea",
            "morning",
            "snow",
            "lake",
            "sunset",
            "pine",
            "shadow",
            "leaf",
            "dawn",
            "glitter",
            "forest",
            "hill",
            "cloud",
            "meadow",
            "sun",
            "glade",
            "bird",
            "brook",
            "butterfly",
            "bush",
            "dew",
            "dust",
            "field",
            "fire",
            "flower",
            "firefly",
            "feather",
            "grass",
            "haze",
            "mountain",
            "night",
            "pond",
            "darkness",
            "snowflake",
            "silence",
            "sound",
            "sky",
            "shape",
            "surf",
            "thunder",
            "violet",
            "water",
            "wildflower",
            "wave",
            "water",
            "resonance",
            "sun",
            "log",
            "dream",
            "cherry",
            "tree",
            "fog",
            "frost",
            "voice",
            "paper",
            "frog",
            "smoke",
            "star",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

        let result: Result<Vec<String>, String> = NounRepositoryImpl::new().fetch_all().await;
        assert_eq!(result, Ok(expected));
    }
}
