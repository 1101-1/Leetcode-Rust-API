use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProblemsSolvedBeatsStats {
    pub difficulty: String,
    pub percentage: f32,
}

#[derive(Debug, Deserialize)]
pub struct AcSubmissionNum {
    pub difficulty: String,
    pub count: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct SubmitStatsGlobal {
    pub acSubmissionNum: Vec<AcSubmissionNum>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct MatchedUser {
    pub problemsSolvedBeatsStats: Vec<ProblemsSolvedBeatsStats>,
    pub submitStatsGlobal: SubmitStatsGlobal,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Data {
    pub matchedUser: MatchedUser,
}

#[derive(Debug, Deserialize)]
pub struct BeatStats {
    pub data: Data,
}
