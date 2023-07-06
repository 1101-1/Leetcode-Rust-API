use serde::Deserialize;
use serde_json::json;

use crate::{error::Errors, resources::descr::ProblemData, Category, Difficulty, Status, Tags};

#[derive(Debug)]
pub struct TaskBuilder {
    pub client: reqwest::Client,
    pub key_word: String,
    pub limit: u32,
    pub category: String,
    pub filters: Filters,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Filters {
    difficulty: String,
    orderBy: String,
    sortOrder: String,
    status: String,
    tags: Vec<String>,
}

impl Default for Filters {
    fn default() -> Self {
        Self {
            difficulty: Default::default(),
            status: Default::default(),
            tags: Vec::<String>::new(),
            orderBy: String::from("FRONTEND_ID"),
            sortOrder: String::from("ASCENDING"),
        }
    }
}

#[allow(unused)]
impl TaskBuilder {
    pub fn set_category(mut self, categoty: Category) -> TaskBuilder {
        match categoty {
            Category::AllTopics => self.category = String::from(""),
            Category::Algorithms => self.category = String::from("algorithms"),
            Category::DataBase => self.category = String::from("database"),
            Category::JavaScript => self.category = String::from("javascript"),
            Category::Shell => self.category = String::from("shell"),
            Category::Concurrency => self.category = String::from("concurrency"),
        }
        self
    }

    pub fn set_difficulty(mut self, difficulty: Difficulty) -> TaskBuilder {
        match difficulty {
            Difficulty::Easy => self.filters.difficulty = String::from("EASY"),
            Difficulty::Medium => self.filters.difficulty = String::from("MEDIUM"),
            Difficulty::Hard => self.filters.difficulty = String::from("HARD"),
        }
        self
    }

    pub fn set_status(mut self, status: Status) -> TaskBuilder {
        match status {
            Status::Todo => self.filters.status = String::from("NOT_STARTED"),
            Status::Solved => self.filters.status = String::from("AC"),
            Status::Attempted => self.filters.status = String::from("TRIED"),
        }
        self
    }

    pub fn set_note_limit(mut self, limit: u32) -> TaskBuilder {
        self.limit = limit;
        self
    }

    pub fn set_keyword(mut self, keyword: &str) -> TaskBuilder {
        self.key_word = String::from(keyword);
        self
    }

    pub fn set_tags(mut self, tags: Vec<Tags>) -> TaskBuilder {
        let mut res_tags = Vec::new();

        for tag in tags {
            let tag = match tag {
                Tags::Array => "array",
                Tags::String => "string",
                Tags::HashTable => "hash-table",
                Tags::Math => "math",
                Tags::DynamicProgramming => "dynamic-programming",
                Tags::Sorting => "sorting",
                Tags::Greedy => "greedy",
                Tags::DepthFirstSearch => "depth-first-search",
                Tags::Database => "database",
                Tags::BinarySearch => "binary-search",
                Tags::BreadthFirstSearch => "breadth-first-search",
                Tags::Tree => "tree",
                Tags::Matrix => "matrix",
                Tags::TwoPointers => "two-pointers",
                Tags::BinaryTree => "binary-tree",
                Tags::BitManipulation => "bit-manipulation",
                Tags::HeapPriorityQueue => "heap-priority-queue",
                Tags::Stack => "stack",
                Tags::Graph => "graph",
                Tags::PrefixSum => "prefix-sum",
                Tags::Simulation => "simulation",
                Tags::Design => "design",
                Tags::Counting => "counting",
                Tags::Backtracking => "backtracking",
                Tags::SlidingWindow => "sliding-window",
                Tags::UnionFind => "union-find",
                Tags::LinkedList => "linked-list",
                Tags::OrderedSet => "ordered-set",
                Tags::MonotonicStack => "monotonic-stack",
                Tags::Enumeration => "enumeration",
                Tags::Recursion => "recursion",
                Tags::Trie => "trie",
                Tags::DivideAndConquer => "divide-and-conquer",
                Tags::Bitmask => "bitmask",
                Tags::BinarySearchTree => "binary-search-tree",
                Tags::NumberTheory => "number-theory",
                Tags::Queue => "queue",
                Tags::SegmentTree => "segment-tree",
                Tags::Memoization => "memoization",
                Tags::Geometry => "geometry",
                Tags::TopologicalSort => "topological-sort",
                Tags::BinaryIndexedTree => "binary-indexed-tree",
                Tags::HashFunction => "hash-function",
                Tags::GameTheory => "game-theory",
                Tags::ShortestPath => "shortest-path",
                Tags::Combinatorics => "combinatorics",
                Tags::DataStream => "data-stream",
                Tags::Interactive => "interactive",
                Tags::StringMatching => "string-matching",
                Tags::RollingHash => "rolling-hash",
                Tags::Brainteaser => "brainteaser",
                Tags::Randomized => "randomized",
                Tags::MonotonicQueue => "monotonic-queue",
                Tags::MergeSort => "merge-sort",
                Tags::Iterator => "iterator",
                Tags::Concurrency => "concurrency",
                Tags::DoublyLinkedList => "doubly-linked-list",
                Tags::ProbabilityStatistics => "probability-statistics",
                Tags::Quickselect => "quickselect",
                Tags::BucketSort => "bucket-sort",
                Tags::SuffixArray => "suffix-array",
                Tags::MinimumSpanningTree => "minimum-spanning-tree",
                Tags::CountingSort => "counting-sort",
                Tags::Shell => "shell",
                Tags::LineSweep => "line-sweep",
                Tags::ReservoirSampling => "reservoir-sampling",
                Tags::EulerianCircuit => "eulerian-circuit",
                Tags::RadixSort => "radix-sort",
                Tags::StronglyConnectedComponent => "strongly-connected-component",
                Tags::RejectionSampling => "rejection-sampling",
                Tags::BiconnectedComponent => "biconnected-component",
            };

            res_tags.push(String::from(tag));
        }

        self.filters.tags = res_tags;

        self
    }

    pub async fn build(self) -> Result<ProblemData, Errors> {
        let mut filters = json!({
            "orderBy": self.filters.orderBy,
            "sortOrder": self.filters.sortOrder,
        });

        if !self.filters.difficulty.is_empty() {
            filters["difficulty"] = json!(self.filters.difficulty);
        }

        if !self.filters.status.is_empty() {
            filters["status"] = json!(self.filters.status);
        }

        if !self.filters.tags.is_empty() {
            filters["tags"] = json!(self.filters.tags);
        }

        if !self.key_word.is_empty() {
            filters["searchKeywords"] = json!(self.key_word);
        }

        let query = json!({
            "query": r#"
                query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) {
                    problemsetQuestionList: questionList(
                        categorySlug: $categorySlug
                        limit: $limit
                        skip: $skip
                        filters: $filters
                    ) {
                        total: totalNum
                        questions: data {
                            acRate
                            difficulty
                            freqBar
                            frontendQuestionId: questionFrontendId
                            isFavor
                            paidOnly: isPaidOnly
                            status
                            title
                            titleSlug
                            topicTags {
                                name
                                id
                                slug
                            }
                            hasSolution
                            hasVideoSolution
                        }
                    }
                }
            "#,
            "variables": {
                "categorySlug": self.category,
                "skip": 0,
                "limit": self.limit,
                "filters": filters
            },
            "operationName": "problemsetQuestionList"
        });

        let query = serde_json::to_string(&query)?;

        let task_info = self
            .client
            .get("https://leetcode.com/graphql/")
            .body(query)
            .send()
            .await?
            .text()
            .await?;

        Ok(serde_json::from_str::<ProblemData>(&task_info)?)
    }
}
