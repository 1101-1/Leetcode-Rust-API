use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct TagProblemCount {
    pub tagName: String,
    pub tagSlug: String,
    pub problemsSolved: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct TagProblemCounts {
    pub advanced: Vec<TagProblemCount>,
    pub intermediate: Vec<TagProblemCount>,
    pub fundamental: Vec<TagProblemCount>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct MatchedUser {
    pub tagProblemCounts: TagProblemCounts,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Data {
    pub matchedUser: MatchedUser,
}

#[derive(Debug, Deserialize)]
pub struct SkillStats {
    pub data: Data,
}
