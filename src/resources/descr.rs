use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProblemData {
    pub data: ProblemsetQuestionListData,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct ProblemsetQuestionListData {
    pub problemsetQuestionList: ProblemsetQuestionList,
}

#[derive(Debug, Deserialize)]
pub struct ProblemsetQuestionList {
    pub questions: Vec<Question>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Question {
    pub titleSlug: String,
}