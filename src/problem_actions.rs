use std::time::Duration;

use serde_json::json;

use crate::{
    error::Errors,
    resources::{
        problemfulldata::{
            CodeSnippetNode, ProblemFullData, SimilarQuestions, Solution, Statistics, TopicTagNode,
        },
        subm_send::{SubmExecutionResult, SubmissionCase, SubmissionCaseResp},
        subm_show::SubmList,
        test_send::{TestCase, TestCaseResp, TestExecutionResult},
        Descryption, Rate,
    },
    ProgrammingLanguage,
};

#[derive(Debug)]
pub struct Problem {
    pub(crate) client: reqwest::Client,
    pub(crate) task_search_name: String,
    pub full_data: ProblemFullData,
}

#[allow(unused)]
impl Problem {
    pub async fn send_test(
        &self,
        lang: ProgrammingLanguage,
        typed_code: &str,
    ) -> Result<TestExecutionResult, Errors> {
        let lang = Self::lang_converter(lang);
        let json_data = serde_json::to_string(&TestCase {
            question_id: self.full_data.data.question.questionId.clone(),
            data_input: self.full_data.data.question.sampleTestCase.clone(),
            lang: lang.to_owned(),
            judge_type: String::from("large"),
            typed_code: String::from(typed_code),
        })?;

        let resp = self
            .client
            .post(format!(
                "https://leetcode.com/problems/{}/interpret_solution/",
                self.task_search_name
            ))
            .body(json_data)
            .send()
            .await?
            .json::<TestCaseResp>()
            .await?;

        loop {
            let status = self
                .client
                .get(format!(
                    "https://leetcode.com/submissions/detail/{}/check/",
                    resp.interpret_id
                ))
                .send()
                .await?
                .json::<TestExecutionResult>()
                .await?;
            if status.state == "SUCCESS" {
                return Ok(status);
            } else if status.state == "FAILURE" {
                return Err(Errors::SendError(
                    "Sent code failure. Err may be occured by Unsupported lang for this problem"
                        .into(),
                ));
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    pub async fn send_subm(
        &self,
        lang: ProgrammingLanguage,
        code: &str,
    ) -> Result<SubmExecutionResult, Errors> {
        let lang = Self::lang_converter(lang);
        let json_data = serde_json::to_string(&SubmissionCase {
            question_id: self.full_data.data.question.questionId.clone(),
            lang: lang.to_owned(),
            typed_code: String::from(code),
        })?;

        let resp = self
            .client
            .post(format!(
                "https://leetcode.com/problems/{}/submit/",
                self.task_search_name
            ))
            .body(json_data)
            .send()
            .await?
            .json::<SubmissionCaseResp>()
            .await?;

        loop {
            let status = self
                .client
                .get(format!(
                    "https://leetcode.com/submissions/detail/{}/check/",
                    resp.submission_id
                ))
                .send()
                .await?
                .json::<SubmExecutionResult>()
                .await?;
            if status.state == "SUCCESS" {
                return Ok(status);
            } else if status.state == "FAILURE" {
                return Err(Errors::SendError(
                    "Sent code failure. Err may be occured by Unsupported lang for this problem"
                        .into(),
                ));
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    fn lang_converter(lang: ProgrammingLanguage) -> &'static str {
        match lang {
            ProgrammingLanguage::CPP => "cpp",
            ProgrammingLanguage::Java => "java",
            ProgrammingLanguage::Python => "python",
            ProgrammingLanguage::Python3 => "python3",
            ProgrammingLanguage::C => "c",
            ProgrammingLanguage::CSharp => "csharp",
            ProgrammingLanguage::JavaScript => "javascript",
            ProgrammingLanguage::TypeScript => "typescript",
            ProgrammingLanguage::Ruby => "ruby",
            ProgrammingLanguage::Swift => "swift",
            ProgrammingLanguage::Go => "go",
            ProgrammingLanguage::Bash => "bash",
            ProgrammingLanguage::Scala => "scala",
            ProgrammingLanguage::Kotlin => "kotlin",
            ProgrammingLanguage::Rust => "rust",
            ProgrammingLanguage::PHP => "php",
            ProgrammingLanguage::Racket => "racket",
            ProgrammingLanguage::Erlang => "erlang",
            ProgrammingLanguage::Elixir => "elixir",
            ProgrammingLanguage::Dart => "dart",
            ProgrammingLanguage::Pandas => "pandas",
            ProgrammingLanguage::React => "react",
        }
    }
    pub fn code_snippets(&self) -> Option<Vec<CodeSnippetNode>> {
        self.full_data.data.question.codeSnippets.clone()
    }

    pub fn solution_info(&self) -> Option<Solution> {
        self.full_data.data.question.solution.clone()
    }

    pub fn related_topics(&self) -> Vec<TopicTagNode> {
        self.full_data.data.question.topicTags.clone()
    }

    pub fn similar_questions(&self) -> Result<Vec<SimilarQuestions>, Errors> {
        Ok(serde_json::from_str::<Vec<SimilarQuestions>>(
            self.full_data.data.question.similarQuestions.as_str(),
        )?)
    }
    pub fn stats(&self) -> Result<Statistics, Errors> {
        Ok(serde_json::from_str::<Statistics>(
            self.full_data.data.question.stats.as_str(),
        )?)
    }

    pub fn hints(&self) -> Vec<String> {
        self.full_data.data.question.hints.clone()
    }

    pub fn description(&self) -> Result<Descryption, Errors> {
        let descryption = json!({
            "name": self.full_data.data.question.title,
            "content": self.full_data.data.question.content
        });
        Ok(serde_json::from_value::<Descryption>(descryption)?)
    }

    pub fn difficulty(&self) -> String {
        self.full_data.data.question.difficulty.clone()
    }

    pub fn rating(&self) -> Result<Rate, Errors> {
        let rate = json!({
            "likes": self.full_data.data.question.likes,
            "dislikes": self.full_data.data.question.dislikes
        });

        Ok(serde_json::from_value::<Rate>(rate)?)
    }

    pub fn category(&self) -> String {
        self.full_data.data.question.categoryTitle.clone()
    }

    pub async fn my_submissions(&self) -> Result<SubmList, Errors> {
        let query = json!({
            "operationName": "Submissions",
            "variables": {
                "offset": 0,
                "limit": 10,
                "lastKey": null,
                "questionSlug": self.task_search_name
            },
            "query": "query Submissions($offset: Int!, $limit: Int!, $lastKey: String, $questionSlug: String!) {\n  submissionList(offset: $offset, limit: $limit, lastKey: $lastKey, questionSlug: $questionSlug) {\n    lastKey\n    hasNext\n    submissions {\n      id\n      statusDisplay\n      lang\n      runtime\n      timestamp\n      url\n      isPending\n      memory\n      __typename\n    }\n    __typename\n  }\n}\n"
        });

        let query = serde_json::to_string(&query)?;

        Ok(self
            .client
            .post("https://leetcode.com/graphql/")
            .body(query)
            .send()
            .await?
            .json::<SubmList>()
            .await?)
    }
}
