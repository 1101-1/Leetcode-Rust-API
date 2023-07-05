# LeetCode API Rust Library
This Rust library provides a convenient way to interact with the LeetCode API, allowing you to programmatically access LeetCode problems, submit solutions, and retrieve submission results.
## Features
* Retrieve a list of LeetCode problems.
* Fetch problem details, including the problem description, constraints, and examples.
* Submit solutions to LeetCode problems.
* Check submission results, including status, runtime, and memory usage.

## Installation

Add the following line to your `Cargo.toml` file:
```toml
[dependencies]
leetcode-api = "0.1.0"
```
## Usage
### Authentication
To use the LeetCode API, you need to obtain an authentication token. Follow the instructions provided by LeetCode to obtain your token.

### Example
```rust
use leetcoderustapi::UserApi;

#[tokio::main]
async fn main() {
    // Set cookie from leetcode
    let token = std::env::var("COOKIE").expect("cookie doesn't set");

    // Create a new LeetCode client
    let api = UserApi::new(&token);

    // Show found problems by keyword and show 5 notes
    let show_problems = api.show_tasks_list("sum", 5).await.unwrap();

    // Find problems by properties with creating builder
    let show_problems_builder = api
        .show_task_builder()
        .set_category(leetcoderustapi::Category::Algorithms)
        .set_difficulty(leetcoderustapi::Difficulty::Easy)
        .set_keyword("sum")
        //max show notes limit is 2763; default is 5
        .set_note_limit(3)
        .set_status(leetcoderustapi::Status::Solved)
        .set_tags(vec![
            leetcoderustapi::Tags::Array,
            leetcoderustapi::Tags::BinarySearch,
        ])
        .build()
        .await
        .unwrap();

    // Fetch the full data for a specific problem
    let problem_info = api.set_task("two sum").await.unwrap();

    // Retrieve code snippets
    let code_snippets = problem_info.code_snippets();

    // Retrieve solution info
    let solution_info = problem_info.solution_info();

    // Retrieve related topics
    let related_topics = problem_info.related_topics();

    // Retrieve similar questions
    let similar_questions = problem_info.similar_questions();

    // Retrieve stats
    let stats = problem_info.stats();

    // Retrieve hints
    let hints = problem_info.hints();

    // Retrieve description
    let description = problem_info.description();

    // Retrieve difficulty
    let difficulty = problem_info.difficulty();

    // Retrieve likes
    let likes = problem_info.likes();

    // Retrieve category
    let category = problem_info.category();

    // We also can send submissions and tests
    // Need to specify a lang and provided code
    let subm_response = problem_info
        .send_subm("rust", "impl Solution { fn two_sum() {}}")
        .await
        .unwrap();
    let test_response = problem_info
        .send_test("rust", "impl Solution { fn two_sum() {}}")
        .await
        .unwrap();
}
```
#### Important
Replace `"COOKIE"` with your actual LeetCode authentication cookie.

For example format in `.env` file:

COOKIE="csrftoken=gN3mmFEKoBFHLZuiHEvZYupqirq7brDmi845GhUK8xBa9u3SUVkgTPFTPsLFuAzR; _ga_CDRWKZTDEX=GS1.1.1688568040.1.1.1688568081.19.0.0; _ga=GA1.1.2048740381.1688568040; _dd_s=rum=0&expire=1688568980299; NEW_PROBLEMLIST_PAGE=1"

### License
This library is licensed under the `MIT License`.