use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
pub struct TopicTagNode {
    pub name: String,
    pub slug: String,
    pub translatedName: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
pub struct CodeSnippetNode {
    pub lang: String,
    pub langSlug: String,
    pub code: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
pub struct Solution {
    pub id: String,
    pub canSeeDetail: bool,
    pub paidOnly: bool,
    pub hasVideoSolution: bool,
    pub paidOnlyVideo: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
pub struct Question {
    pub questionId: String,
    pub questionFrontendId: String,
    pub boundTopicId: Option<String>,
    pub title: String,
    pub titleSlug: String,
    pub content: Option<String>,
    pub translatedTitle: Option<String>,
    pub translatedContent: Option<String>,
    pub isPaidOnly: bool,
    pub canSeeQuestion: bool,
    pub difficulty: String,
    pub likes: u32,
    pub dislikes: u32,
    pub isLiked: Option<bool>,
    pub similarQuestions: String,
    pub exampleTestcases: String,
    pub categoryTitle: String,
    pub contributors: Vec<String>,
    pub topicTags: Vec<TopicTagNode>,
    pub companyTagStats: Option<String>,
    pub codeSnippets: Option<Vec<CodeSnippetNode>>,
    pub stats: String,
    pub hints: Vec<String>,
    pub solution: Option<Solution>,
    pub status: Option<String>,
    pub sampleTestCase: String,
    pub metaData: String,
    pub judgerAvailable: Option<bool>,
    pub judgeType: Option<String>,
    pub mysqlSchemas: Option<Vec<String>>,
    pub enableRunCode: Option<bool>,
    pub enableTestMode: Option<bool>,
    pub enableDebugger: Option<bool>,
    pub envInfo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub question: Question,
}

#[derive(Debug, Deserialize)]
pub struct ProblemFullData {
    pub data: Data,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Statistics {
    pub totalAccepted: String,
    pub totalSubmission: String,
    pub totalAcceptedRaw: u32,
    pub totalSubmissionRaw: u32,
    pub acRate: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct SimilarQuestions {
    pub title: String,
    pub titleSlug: String,
    pub difficulty: String,
    pub translatedTitle: Option<String>,
}
