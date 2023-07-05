use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub(crate) struct TestCase {
    pub question_id: String,
    pub data_input: String,
    pub lang: String,
    pub typed_code: String,
    pub judge_type: String,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub(crate) struct TestCaseResp {
    pub interpret_id: String,
    pub test_case: String,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub(crate) struct TestExecutionResult {
    pub status_code: Option<u32>,
    pub lang: Option<String>,
    pub run_success: Option<bool>,
    pub compile_error: Option<String>,
    pub full_compile_error: Option<String>,
    pub status_runtime: Option<String>,
    pub memory: Option<u32>,
    pub code_answer: Option<Vec<String>>,
    pub code_output: Option<Vec<String>>,
    pub std_output_list: Option<Vec<String>>,
    pub elapsed_time: Option<u32>,
    pub task_finish_time: Option<u64>,
    pub task_name: Option<String>,
    pub expected_status_code: Option<u32>,
    pub expected_lang: Option<String>,
    pub expected_run_success: Option<bool>,
    pub expected_status_runtime: Option<String>,
    pub expected_memory: Option<u32>,
    pub expected_code_answer: Option<Vec<String>>,
    pub expected_code_output: Option<Vec<String>>,
    pub expected_std_output_list: Option<Vec<String>>,
    pub expected_elapsed_time: Option<u32>,
    pub expected_task_finish_time: Option<u64>,
    pub expected_task_name: Option<String>,
    pub correct_answer: Option<bool>,
    pub compare_result: Option<String>,
    pub total_correct: Option<u32>,
    pub total_testcases: Option<u32>,
    pub runtime_percentile: Option<Option<f64>>,
    pub status_memory: Option<String>,
    pub memory_percentile: Option<Option<f64>>,
    pub pretty_lang: Option<String>,
    pub submission_id: Option<String>,
    pub status_msg: Option<String>,
    pub state: String,
}
