use std::error::Error;

use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{json, Value};
use source::{cookie::CookieData, descr::TaskData, taskfulldata::TaskFullData};
use task_actions::Task;
use task_build::{Filters, TaskBuilder};

pub mod source;
pub mod task_actions;
pub mod task_build;

pub struct UserApi {
    client: reqwest::Client,
}

#[allow(unused)]
impl UserApi {
    pub async fn new(cookie: &str) -> Result<Self, Box<dyn Error>> {
        let mut headers = HeaderMap::new();

        headers.insert("Host", HeaderValue::from_static("leetcode.com"));
        headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36"));
        headers.insert("Origin", HeaderValue::from_static("https://leetcode.com"));
        headers.insert("Referer", HeaderValue::from_static("https://leetcode.com/"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        headers.insert("Connection", HeaderValue::from_static("keep-alive"));
        headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
        headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
        headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-origin"));

        let valid_data = Self::valid_check(headers.clone(), &cookie).await?;

        let cookie = if valid_data.0 {
            cookie
        } else {
            return Err("Cookie is invalid or User not signed".into());
        };

        let token = valid_data.1;

        headers.insert("Cookie", HeaderValue::from_str(&cookie).unwrap());
        headers.insert("x-csrftoken", HeaderValue::from_str(&token).unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Ok(Self { client })
    }

    async fn valid_check(
        mut headers: HeaderMap,
        cookie: &str,
    ) -> Result<(bool, String), Box<dyn Error>> {
        let token = if let Some(cookie) = cookie
            .strip_prefix("csrftoken=")
            .and_then(|val| Some(&val[..64]))
        {
            cookie
        } else {
            return Err("cannot take token from cookie".into());
        };
        headers.insert("Cookie", HeaderValue::from_str(&cookie).unwrap());
        headers.insert("x-csrftoken", HeaderValue::from_str(&token).unwrap());

        let operation_name = "globalData";
        let variables: Value = json!({});

        let query = r#"
        query globalData {
            feature {
                questionTranslation
                subscription
                signUp
                discuss
                mockInterview
                contest
                store
                chinaProblemDiscuss
                socialProviders
                studentFooter
                enableChannels
                dangerZone
                enableSharedWorker
                enableRecaptchaV3
                enableDebugger
                enableDebuggerPremium
                enableAutocomplete
                enableAutocompletePremium
                enableAllQuestionsRaw
                autocompleteLanguages
                enableIndiaPricing
                enableReferralDiscount
                maxTimeTravelTicketCount
                enableStoreShippingForm
                enableCodingChallengeV2
                __typename
            }
            streakCounter {
                streakCount
                daysSkipped
                currentDayCompleted
                __typename
            }
            currentTimestamp
            userStatus {
                isSignedIn
                isAdmin
                isStaff
                isSuperuser
                isMockUser
                isTranslator
                isPremium
                isVerified
                checkedInToday
                username
                realName
                avatar
                optedIn
                requestRegion
                region
                activeSessionId
                permissions
                notificationStatus {
                    lastModified
                    numUnread
                    __typename
                }
                completedFeatureGuides
                __typename
            }
            siteRegion
            chinaHost
            websocketUrl
            recaptchaKey
            recaptchaKeyV2
            sitewideAnnouncement
            userCountryCode
        }
    "#;

        let json_data = json!({
            "operationName": operation_name,
            "variables": variables,
            "query": query,
        });

        let query = serde_json::to_string(&json_data).unwrap();

        let client = reqwest::Client::new();

        let cookie_info = match client
            .post("https://leetcode.com/graphql/")
            .body(query)
            .headers(headers)
            .send()
            .await
            .unwrap()
            .text()
            .await
        {
            Ok(data) => {
                // println!("{}", data);
                data
            },
            Err(_err) => return Err("Can't take cookie info".into()),
        };

        if serde_json::from_str::<CookieData>(&cookie_info)?
            .data
            .userStatus
            .isSignedIn
        {
            println!("work");
            return Ok((true, String::from(token)));
        }

        Ok((false, String::from(token)))
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

    pub async fn show_tasks_list(
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
            return Err("Problem does not found".into());
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

#[allow(unused)]
pub enum Category {
    AllTopics,
    Algorithms,
    DataBase,
    JavaScript,
    Shell,
    Concurrency,
}

#[allow(unused)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[allow(unused)]
pub enum Status {
    Todo,
    Solved,
    Attempted,
}
#[allow(unused)]
#[derive(Debug)]
pub enum Tags {
    Array,
    String,
    HashTable,
    Math,
    DynamicProgramming,
    Sorting,
    Greedy,
    DepthFirstSearch,
    Database,
    BinarySearch,
    BreadthFirstSearch,
    Tree,
    Matrix,
    TwoPointers,
    BinaryTree,
    BitManipulation,
    HeapPriorityQueue,
    Stack,
    Graph,
    PrefixSum,
    Simulation,
    Design,
    Counting,
    Backtracking,
    SlidingWindow,
    UnionFind,
    LinkedList,
    OrderedSet,
    MonotonicStack,
    Enumeration,
    Recursion,
    Trie,
    DivideAndConquer,
    Bitmask,
    BinarySearchTree,
    NumberTheory,
    Queue,
    SegmentTree,
    Memoization,
    Geometry,
    TopologicalSort,
    BinaryIndexedTree,
    HashFunction,
    GameTheory,
    ShortestPath,
    Combinatorics,
    DataStream,
    Interactive,
    StringMatching,
    RollingHash,
    Brainteaser,
    Randomized,
    MonotonicQueue,
    MergeSort,
    Iterator,
    Concurrency,
    DoublyLinkedList,
    ProbabilityStatistics,
    Quickselect,
    BucketSort,
    SuffixArray,
    MinimumSpanningTree,
    CountingSort,
    Shell,
    LineSweep,
    ReservoirSampling,
    EulerianCircuit,
    RadixSort,
    StronglyConnectedComponent,
    RejectionSampling,
    BiconnectedComponent,
}
