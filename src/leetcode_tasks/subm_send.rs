use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub(crate) struct SubmissionCaseResp {
    pub submission_id: u32,
}

#[derive(Serialize, Debug)]
pub(crate) struct SubmissionCase {
    pub question_id: String,
    pub lang: String,
    pub typed_code: String,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub(crate) struct SubmExecutionResult {
    pub status_code: Option<u32>,
    pub lang: Option<String>,
    pub run_success: Option<bool>,
    pub status_runtime: Option<String>,
    pub memory: Option<u32>,
    pub question_id: Option<String>,
    pub elapsed_time: Option<u32>,
    pub compare_result: Option<String>,
    pub code_output: Option<String>,
    pub std_output: Option<String>,
    pub last_testcase: Option<String>,
    pub expected_output: Option<String>,
    pub task_finish_time: Option<u64>,
    pub task_name: Option<String>,
    pub finished: Option<bool>,
    pub total_correct: Option<u32>,
    pub total_testcases: Option<u32>,
    pub runtime_percentile: Option<f32>,
    pub status_memory: Option<String>,
    pub memory_percentile: Option<f32>,
    pub pretty_lang: Option<String>,
    pub submission_id: Option<String>,
    pub input_formatted: Option<String>,
    pub input: Option<String>,
    pub status_msg: Option<String>,
    pub state: String,
}
