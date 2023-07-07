use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct ContestBadge {
    pub name: Option<String>,
    pub expired: Option<bool>,
    pub hoverText: Option<String>,
    pub icon: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Profile {
    pub ranking: i32,
    pub userAvatar: String,
    pub realName: String,
    pub aboutMe: String,
    pub school: Option<String>,
    pub websites: Vec<String>,
    pub countryName: Option<String>,
    pub company: Option<String>,
    pub jobTitle: Option<String>,
    pub skillTags: Vec<String>,
    pub postViewCount: i32,
    pub postViewCountDiff: i32,
    pub reputation: i32,
    pub reputationDiff: i32,
    pub solutionCount: i32,
    pub solutionCountDiff: i32,
    pub categoryDiscussCount: i32,
    pub categoryDiscussCountDiff: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct MatchedUser {
    pub contestBadge: Option<ContestBadge>,
    pub username: String,
    pub githubUrl: Option<String>,
    pub twitterUrl: Option<String>,
    pub linkedinUrl: Option<String>,
    pub profile: Profile,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Data {
    pub matchedUser: MatchedUser,
}

#[derive(Debug, Deserialize)]
pub struct UserFoundData {
    pub data: Data,
}
