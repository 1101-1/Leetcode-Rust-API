use std::{error::Error, time::Duration};

use serde_json::json;

use super::{
    subm_send::{SubmExecutionResult, SubmissionCase, SubmissionCaseResp},
    subm_show::SubmList,
    taskfulldata::TaskFullData,
    test_send::{TestCase, TestCaseResp, TestExecutionResult},
    Rate,
};

#[derive(Debug)]
pub(crate) struct Task {
    pub client: reqwest::Client,
    pub task_search_name: String,
    pub full_data: TaskFullData,
}

#[allow(unused)]
impl Task {
    pub async fn send_test(
        &self,
        lang: &str,
        typed_code: &str,
    ) -> Result<TestExecutionResult, Box<dyn Error>> {
        let json_data = serde_json::to_string(&TestCase {
            question_id: self.full_data.data.question.questionId.clone(),
            data_input: self.full_data.data.question.sampleTestCase.clone(),
            lang: lang.to_lowercase(),
            judge_type: String::from("large"),
            typed_code: String::from(typed_code),
        })
        .unwrap();

        let resp = match self
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
        {
            Ok(data) => data,
            Err(_err) => return Err("Your token is invalid or access to the Task is deny".into()),
        };

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
                .await?;
            if status.state == "SUCCESS" {
                return Ok(status);
            }
            println!("{:?}", status.state);
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    pub async fn send_subm(
        &self,
        lang: &str,
        code: &str,
    ) -> Result<SubmExecutionResult, Box<dyn Error>> {
        let json_data = serde_json::to_string(&SubmissionCase {
            question_id: self.full_data.data.question.questionId.clone(),
            lang: lang.to_lowercase(),
            typed_code: String::from(code),
        })
        .unwrap();

        let resp = match self
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
        {
            Ok(data) => data,
            Err(_err) => return Err("Your token is invalid or access to the task is deny".into()),
        };

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
                .await?;
            if status.state == "SUCCESS" {
                return Ok(status);
            }
            println!("{:?}", status.state);
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    pub async fn descryption(&self) -> Result<String, Box<dyn Error>> {
        let query = json!({
            "query": "\n    query questionContent($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    content\n    mysqlSchemas\n  }\n}\n    ",
            "variables": {
                "titleSlug": self.task_search_name
            },
            "operationName": "questionContent"
        });

        let query = serde_json::to_string(&query).unwrap();

        match self
            .client
            .post("https://leetcode.com/graphql/")
            .body(query)
            .send()
            .await
            .unwrap()
            .text()
            .await
        {
            Ok(data) => Ok(data),
            Err(_err) => Err("Can't take descryption from task".into()),
        }
    }

    pub async fn likes(&self) -> Rate {
        let rate = json!({
            "likes": self.full_data.data.question.likes,
            "dislikes": self.full_data.data.question.dislikes
        });

        serde_json::from_value::<Rate>(rate).unwrap()
    }

    pub async fn submissions(&self) -> Result<SubmList, Box<dyn Error>> {
        let query = json!({
            "operationName": "Submissions",
            "variables": {
                "offset": 0,
                "limit": 20,
                "lastKey": null,
                "questionSlug": self.task_search_name
            },
            "query": "query Submissions($offset: Int!, $limit: Int!, $lastKey: String, $questionSlug: String!) {\n  submissionList(offset: $offset, limit: $limit, lastKey: $lastKey, questionSlug: $questionSlug) {\n    lastKey\n    hasNext\n    submissions {\n      id\n      statusDisplay\n      lang\n      runtime\n      timestamp\n      url\n      isPending\n      memory\n      __typename\n    }\n    __typename\n  }\n}\n"
        });

        let query = serde_json::to_string(&query).unwrap();

        match self
            .client
            .post("https://leetcode.com/graphql/")
            .body(query)
            .send()
            .await
            .unwrap()
            .json::<SubmList>()
            .await
        {
            Ok(data) => Ok(data),
            Err(_err) => Err("Can't take descryption from task".into()),
        }
    }
}
