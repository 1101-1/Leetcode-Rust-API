use std::time::Duration;

use serde_json::json;


use super::{
    subm::{SubmExecutionResult, SubmissionCase, SubmissionCaseResp},
    test::{TestCase, TestCaseResp, TestExecutionResult}, taskfulldata::TaskFullData,
};

#[derive(Debug)]
pub(crate) struct Task {
    pub client: reqwest::Client,
    pub task_search_name: String,
    pub full_data: TaskFullData,
}

#[allow(unused)]
impl Task {
    pub async fn send_test(&self, lang: &str, typed_code: &str) -> TestExecutionResult {
        let json_data = serde_json::to_string(&TestCase {
            question_id: self.full_data.data.question.questionId.clone(),
            data_input: self.full_data.data.question.sampleTestCase.clone(),
            lang: lang.to_lowercase(),
            judge_type: String::from("large"),
            typed_code: String::from(typed_code),
        })
        .unwrap();

        let resp = self
            .client
            .post(format!(
                "https://leetcode.com/problems/{}/interpret_solution/",
                self.task_search_name
            ))
            .body(json_data)
            .send()
            .await
            .unwrap()
            .json::<TestCaseResp>()
            .await
            .unwrap();

        loop {
            let status = self
                .client
                .get(format!(
                    "https://leetcode.com/submissions/detail/{}/check/",
                    resp.interpret_id
                ))
                .send()
                .await
                .unwrap()
                .json::<TestExecutionResult>()
                .await
                .unwrap();
            if status.state == "SUCCESS" {
                return status;
            }
            println!("{:?}", status.state);
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    pub async fn send_subm(&self, lang: &str, typed_code: &str) -> SubmExecutionResult {
        let json_data = serde_json::to_string(&SubmissionCase {
            question_id: self.full_data.data.question.questionId.clone(),
            lang: lang.to_lowercase(),
            typed_code: String::from(typed_code),
        })
        .unwrap();

        let resp = self
            .client
            .post(format!(
                "https://leetcode.com/problems/{}/submit/",
                self.task_search_name
            ))
            .body(json_data)
            .send()
            .await
            .unwrap()
            .json::<SubmissionCaseResp>()
            .await
            .unwrap();

        loop {
            let status = self
                .client
                .get(format!(
                    "https://leetcode.com/submissions/detail/{}/check/",
                    resp.submission_id
                ))
                .send()
                .await
                .unwrap()
                .json::<SubmExecutionResult>()
                .await
                .unwrap();
            if status.state == "SUCCESS" {
                return status;
            }
            println!("{:?}", status.state);
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    pub async fn find_info(&self) -> String {
        let query = json!({
            "query": "\n    query questionContent($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    content\n    mysqlSchemas\n  }\n}\n    ",
            "variables": {
                "titleSlug": self.task_search_name
            },
            "operationName": "questionContent"
        });

        let query = serde_json::to_string(&query).unwrap();

        self.client
            .post("https://leetcode.com/graphql/")
            .body(query)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }
}
