use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct SubmissionDumpNode {
    pub id: String,
    pub statusDisplay: String,
    pub lang: String,
    pub runtime: String,
    pub timestamp: String,
    pub url: String,
    pub isPending: String,
    pub memory: String,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct SubmissionListNode {
    pub lastKey: Option<String>,
    pub hasNext: bool,
    pub submissions: Vec<SubmissionDumpNode>,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Data {
    pub submissionList: SubmissionListNode,
}

#[derive(Debug, Deserialize)]
pub struct SubmList {
    pub data: Data,
}
