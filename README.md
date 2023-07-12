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
leetcoderustapi = "1.0.4"
```
## Usage
### Authentication
To use the LeetCode API, you need to obtain an authentication token. Follow the instructions provided by LeetCode to obtain your token.

### Example: Action with problems
```rust
use leetcoderustapi::{problem_build::{Tags, Category, Difficulty, Status}, UserApi, ProgrammingLanguage,};

#[tokio::main]
async fn main() {
    // Set cookie from leetcode
    let token = std::env::var("COOKIE").expect("cookie doesn't set");

    // Create a new LeetCode client
    let api = UserApi::new(&token).await.unwrap();

    // Show found problems by keyword and show 5 notes
    let show_problems = api.show_problm_list("sum", 5).await.unwrap();

    // Find problems by properties with creating problem builder
    let problems_builder = api
        .problem_builder()
        .set_category(Category::Algorithms)
        .set_difficulty(Difficulty::Easy)
        .set_keyword("sum")
        //max show notes limit is 2763; default is 5
        .set_note_limit(3)
        .set_status(Status::Solved)
        //max tags over 50+
        .set_tags(vec![
            Tags::Array,
            Tags::BinarySearch,
        ])
        .build()
        .await
        .unwrap();

    // Fetch the full data for a specific problem
    let problem_info = api.set_problem("two sum").await.unwrap();

    // Retrieve previous submissions to this problem
    let my_submissions = problem_info.my_submissions().await.unwrap();

    // Retrieve code snippets
    let code_snippets = problem_info.code_snippets().unwrap();

    // Retrieve solution info
    let solution_info = problem_info.solution_info().unwrap();

    // Retrieve related topics
    let related_topics = problem_info.related_topics();

    // Retrieve similar questions
    let similar_questions = problem_info.similar_questions().unwrap();

    // Retrieve stats
    let stats = problem_info.stats().unwrap();

    // Retrieve hints
    let hints = problem_info.hints();

    // Retrieve description
    let description = problem_info.description().unwrap();

    // Retrieve difficulty
    let difficulty = problem_info.difficulty();

    // Retrieve likes and dislikes
    let likes = problem_info.rating().unwrap();

    // Retrieve category
    let category = problem_info.category();

    // We also can send submissions and tests
    // Need to specify a lang and provided code
    let subm_response = problem_info
        .send_subm(ProgrammingLanguage::Rust, "impl Solution { fn two_sum() {}}")
        .await
        .unwrap();
    let test_response = problem_info
        .send_test(ProgrammingLanguage::Rust, "impl Solution { fn two_sum() {}}")
        .await
        .unwrap();
}
```

### Example: Actions with Self profile
```rust
#[tokio::main]
async fn main() {
    // Set cookie from leetcode
    let token = std::env::var("COOKIE").expect("cookie doesn't set");

    // Create a new LeetCode client
    let api = UserApi::new(&token).await.unwrap();

    // Create interaction with profile
    let user_profile = api.my_profile().await.unwrap();

    // Create empty list of the problems with provided name
    user_profile
        .create_fav_list("my_new_favorite_list")
        .await
        .unwrap();

    // Rename list
    user_profile
        .rename_fav_list("my_new_favorite_list", "hard_problems")
        .await
        .unwrap();

    // Set list puplic
    user_profile.set_public("hard_problems").await.unwrap();

    // Set list private
    user_profile.set_private("hard_problems").await.unwrap();

    // Get link to the list if it is a public
    let share_list_url = user_profile.get_share_url("hard_problems").await.unwrap();

    // Show existing lists
    let lists = user_profile.show_lists();

    // Delete list with provided name
    user_profile
        .delete_list("hard_problems")
        .await
        .unwrap();

    // Show users last 10 notification
    let notifications = user_profile.get_notifications().await.unwrap();
}
```
### Example: Actions with Public user profile
```rust
#[tokio::main]
async fn main() {
    // Set cookie from leetcode
    let token = std::env::var("COOKIE").expect("cookie doesn't set");

    // Create a new LeetCode client
    let api = UserApi::new(&token).await.unwrap();

    // Find public user
    let user = api
        .find_profile("1101-1")
        .await;

    // Check public user common stats
    let user_stats = user.
        user_stats()
        .await
        .unwrap();

    // Check what langs used user
    let lang_stats = user
        .language_stats()
        .await
        .unwrap();
    
    // Check what problems (by tags) solve user
    let skill_stats = user
        .skill_stats()
        .await
        .unwrap();

    // Show rating by beating problems
    let beat_stats = ser
        .problem_beat_stats()
        .await
        .unwrap();

     // Show recent submissons of user
    let beat_stats = ser
        .recent_subm_list()
        .await
        .unwrap();
}
```


#### Important
Replace `"COOKIE"` with your actual LeetCode authentication cookie.

For example format in `.env` file:

```env
COOKIE="csrftoken=gN3mmFEKoBFHLZuiHEvZYupqirq7brDmi845GhUK8xBa9u3SUVkgTPFTPsLFuAzR; _ga_CDRWKZTDEX=GS1.1.1688568040.1.1.1688568081.19.0.0; _ga=GA1.1.2048740381.1688568040; _dd_s=rum=0&expire=1688568980299; NEW_PROBLEMLIST_PAGE=1"
```

### License
This library is licensed under the `MIT License`.

