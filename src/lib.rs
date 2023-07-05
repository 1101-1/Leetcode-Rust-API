use std::error::Error;

use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;
use source::{taskfulldata::TaskFullData, descr::TaskData};
use task_actions::Task;
use task_build::{TaskBuilder, Filters};

mod source;
mod task_actions;
mod task_build;
mod query_enums;


pub(crate) struct UserApi {
    client: reqwest::Client,
}

#[allow(unused)]
impl UserApi {
    pub fn new(cookie: &str) -> Self {
        let mut headers = HeaderMap::new();
        let token = String::from(
            cookie
                .strip_prefix("csrftoken=")
                .and_then(|val| Some(&val[..64]))
                .unwrap_or(""),
        );

        headers.insert("Host", HeaderValue::from_static("leetcode.com"));
        headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36"));
        headers.insert("Cookie", HeaderValue::from_str(&cookie).unwrap());
        headers.insert("x-csrftoken", HeaderValue::from_str(&token).unwrap());
        headers.insert("Origin", HeaderValue::from_static("https://leetcode.com"));
        headers.insert("Referer", HeaderValue::from_static("https://leetcode.com/"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        headers.insert("Connection", HeaderValue::from_static("keep-alive"));
        headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
        headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
        headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-origin"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { client }
    }

    pub async fn set_task(&self, task: &str) -> Result<Task, Box<dyn Error>> {
        let info = Self::get_full_data(
            &self,
            Self::get_question_name(&self, String::from(task)).await?,
        )
        .await?;

        Ok(Task {
            client: self.client.clone(),
            task_search_name: info.0,
            full_data: info.1,
        })
    }

    async fn get_full_data(
        &self,
        task_name: String,
    ) -> Result<(String, TaskFullData), Box<dyn Error>> {
        let json_obj = json!({
            "operationName": "questionData",
            "variables": {
                "titleSlug": task_name
            },
            "query": "query questionData($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    questionId\n    questionFrontendId\n    boundTopicId\n    title\n    titleSlug\n    content\n    translatedTitle\n    translatedContent\n    isPaidOnly\n    canSeeQuestion\n    difficulty\n    likes\n    dislikes\n    isLiked\n    similarQuestions\n    exampleTestcases\n    categoryTitle\n    contributors {\n      username\n      profileUrl\n      avatarUrl\n      __typename\n    }\n    topicTags {\n      name\n      slug\n      translatedName\n      __typename\n    }\n    companyTagStats\n    codeSnippets {\n      lang\n      langSlug\n      code\n      __typename\n    }\n    stats\n    hints\n    solution {\n      id\n      canSeeDetail\n      paidOnly\n      hasVideoSolution\n      paidOnlyVideo\n      __typename\n    }\n    status\n    sampleTestCase\n    metaData\n    judgerAvailable\n    judgeType\n    mysqlSchemas\n    enableRunCode\n    enableTestMode\n    enableDebugger\n    envInfo\n    libraryUrl\n    adminUrl\n    challengeQuestion {\n      id\n      date\n      incompleteChallengeCount\n      streakCount\n      type\n      __typename\n    }\n    __typename\n  }\n}"
        });

        let query = serde_json::to_string(&json_obj).unwrap();

        let full_data = match self
            .client
            .post("https://leetcode.com/graphql/")
            .body(query)
            .send()
            .await
            .unwrap()
            .json::<TaskFullData>()
            .await
        {
            Ok(data) => data,
            Err(_err) => return Err("Can't take task data".into()),
        };

        Ok((full_data.data.question.titleSlug.clone(), full_data))
    }

    pub async fn show_task_list(
        &self,
        key_word: &str,
        limit: u32,
    ) -> Result<TaskData, Box<dyn Error>> {
        let query = json!({
            "query": "query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) { problemsetQuestionList: questionList( categorySlug: $categorySlug limit: $limit skip: $skip filters: $filters ) { total: totalNum questions: data { acRate difficulty freqBar frontendQuestionId: questionFrontendId isFavor paidOnly: isPaidOnly status title titleSlug topicTags { name id slug } hasSolution hasVideoSolution } } }",
            "variables": {
                "categorySlug": "",
                "skip": 0,
                "limit": limit,
                "filters": {
                    "searchKeywords": String::from(key_word)
                }
            },
            "operationName": "problemsetQuestionList"
        });

        let query = serde_json::to_string(&query).unwrap();

        let task_info = self
            .client
            .get("https://leetcode.com/graphql/")
            .body(query)
            .send()
            .await
            .unwrap()
            .text()
            .await;

        if let Err(_err) = task_info {
            return Err("Task does not found".into());
        }

        Ok(serde_json::from_str::<TaskData>(&task_info.unwrap())?)
    }

    pub fn show_task_builder(&self) -> TaskBuilder {
        TaskBuilder {
            client: self.client.clone(),
            key_word: String::new(),
            limit: 5,
            category: String::new(),
            filters: Filters::default(),
        }
    }

    async fn get_question_name(&self, name: String) -> Result<String, Box<dyn Error>> {
        let query = json!({
            "query": "query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) { problemsetQuestionList: questionList( categorySlug: $categorySlug limit: $limit skip: $skip filters: $filters ) { total: totalNum questions: data { acRate difficulty freqBar frontendQuestionId: questionFrontendId isFavor paidOnly: isPaidOnly status title titleSlug topicTags { name id slug } hasSolution hasVideoSolution } } }",
            "variables": {
                "categorySlug": "",
                "skip": 0,
                "limit": 1,
                "filters": {
                    "searchKeywords": name
                }
            },
            "operationName": "problemsetQuestionList"
        });

        let query = serde_json::to_string(&query).unwrap();

        let task_info = self
            .client
            .get("https://leetcode.com/graphql/")
            .body(query)
            .send()
            .await
            .unwrap()
            .text()
            .await;

        if let Err(_err) = task_info {
            return Err("Task does not found".into());
        }

        let parsed_data: TaskData = serde_json::from_str(&task_info.unwrap())?;

        Ok(parsed_data.data.problemsetQuestionList.questions[0]
            .titleSlug
            .clone())
    }
}
