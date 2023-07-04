use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskData {
    pub data: ProblemsetQuestionListData,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct ProblemsetQuestionListData {
    pub problemsetQuestionList: ProblemsetQuestionList,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProblemsetQuestionList {
    pub total: i32,
    pub questions: Vec<Question>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Question {
    pub acRate: f64,
    pub difficulty: String,
    pub freqBar: Option<String>,
    pub frontendQuestionId: String,
    pub isFavor: bool,
    pub paidOnly: bool,
    pub status: Option<String>,
    pub title: String,
    pub titleSlug: String,
    pub topicTags: Vec<TopicTag>,
    pub hasSolution: bool,
    pub hasVideoSolution: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TopicTag {
    pub name: String,
    pub id: String,
    pub slug: String,
}
