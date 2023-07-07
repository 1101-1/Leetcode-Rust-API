use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct LanguageProblemCount {
    pub languageName: String,
    pub problemsSolved: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct MatchedUser {
    pub languageProblemCount: Vec<LanguageProblemCount>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Data {
    pub matchedUser: MatchedUser,
}

#[derive(Debug, Deserialize)]
pub struct LanguageStats {
    pub data: Data,
}
