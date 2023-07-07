use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct RecentAcSubmission {
    pub id: String,
    pub title: String,
    pub titleSlug: String,
    pub timestamp: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Data {
    pub recentAcSubmissionList: Vec<RecentAcSubmission>,
}

#[derive(Debug, Deserialize)]
pub struct RecentSubmList {
    pub data: Data,
}
