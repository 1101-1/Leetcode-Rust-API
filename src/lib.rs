use error::Errors;
use problem_actions::Problem;
use problem_build::{Filters, ProblemBuilder};
use profile::{MyProfile, UserProfile};
use reqwest::header::{HeaderMap, HeaderValue};
use resources::{
    cookie::CookieData, descr::ProblemData, fav_list::FavoriteList,
    problemfulldata::ProblemFullData,
};
use serde_json::json;

pub mod error;
pub mod problem_actions;
pub mod problem_build;
pub mod profile;
pub mod resources;

#[derive(Debug)]
pub struct UserApi {
    client: reqwest::Client,
}

impl UserApi {
    pub async fn new(cookie: &str) -> Result<Self, Errors> {
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
            return Err(error::Errors::ApiError(
                "Cookie is invalid or User not signed".into(),
            ));
        };

        let token = valid_data.1;

        headers.insert("Cookie", HeaderValue::from_str(&cookie).unwrap());
        headers.insert("x-csrftoken", HeaderValue::from_str(&token).unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { client })
    }

    async fn valid_check(mut headers: HeaderMap, cookie: &str) -> Result<(bool, String), Errors> {
        let token = if let Some(token) = cookie
            .strip_prefix("csrftoken=")
            .and_then(|val| Some(&val[..64]))
        {
            token
        } else {
            return Err(Errors::ApiError("Cannot take token from cookie".into()));
        };

        headers.insert("Cookie", HeaderValue::from_str(&cookie).unwrap());
        headers.insert("x-csrftoken", HeaderValue::from_str(&token).unwrap());
        headers.insert("content-type", HeaderValue::from_static("application/json"));

        let json_data = json!({
            "operationName": "globalData",
            "variables": {},
            "query": "query globalData {\n  userStatus {\n    isSignedIn\n    isAdmin\n    isStaff\n    isSuperuser\n    isMockUser\n    isTranslator\n    isPremium\n    isVerified\n    checkedInToday\n    username\n    realName\n    avatar\n    optedIn\n    requestRegion\n    region\n    activeSessionId\n    permissions\n    notificationStatus {\n      lastModified\n      numUnread\n      __typename\n    }\n    completedFeatureGuides\n    __typename\n  }\n  recaptchaKey\n}"
        });

        let query = serde_json::to_string(&json_data)?;

        let client = reqwest::Client::new();

        let cookie_info = client
            .post("https://leetcode.com/graphql/")
            .body(query)
            .headers(headers.clone())
            .send()
            .await?
            .text()
            .await?;

        let resp_info = serde_json::from_str::<CookieData>(&cookie_info)?;

        if resp_info.data.userStatus.isSignedIn {
            return Ok((true, String::from(token)));
        }

        Ok((false, String::from(token)))
    }

    pub async fn set_problem(&self, problem_name: &str) -> Result<Problem, Errors> {
        let info = Self::fetch_problem_full_data(
            &self,
            Self::get_question_name(&self, String::from(problem_name)).await?,
        )
        .await?;

        Ok(Problem {
            client: self.client.clone(),
            task_search_name: info.0,
            full_data: info.1,
        })
    }

    pub async fn set_problem_by_id(&self, problem_id: u32) -> Result<Problem, Errors> {
        let info = Self::fetch_problem_full_data(
            &self,
            Self::get_question_name(&self, problem_id.to_string()).await?,
        )
        .await?;

        Ok(Problem {
            client: self.client.clone(),
            task_search_name: info.0,
            full_data: info.1,
        })
    }

    async fn fetch_problem_full_data(
        &self,
        problem: String,
    ) -> Result<(String, ProblemFullData), Errors> {
        let json_obj = json!({
            "operationName": "questionData",
            "variables": {
                "titleSlug": problem
            },
            "query": "query questionData($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    questionId\n    questionFrontendId\n    boundTopicId\n    title\n    titleSlug\n    content\n    translatedTitle\n    translatedContent\n    isPaidOnly\n    canSeeQuestion\n    difficulty\n    likes\n    dislikes\n    isLiked\n    similarQuestions\n    exampleTestcases\n    categoryTitle\n    contributors {\n      username\n      profileUrl\n      avatarUrl\n      __typename\n    }\n    topicTags {\n      name\n      slug\n      translatedName\n      __typename\n    }\n    companyTagStats\n    codeSnippets {\n      lang\n      langSlug\n      code\n      __typename\n    }\n    stats\n    hints\n    solution {\n      id\n      canSeeDetail\n      paidOnly\n      hasVideoSolution\n      paidOnlyVideo\n      __typename\n    }\n    status\n    sampleTestCase\n    metaData\n    judgerAvailable\n    judgeType\n    mysqlSchemas\n    enableRunCode\n    enableTestMode\n    enableDebugger\n    envInfo\n    libraryUrl\n    adminUrl\n    challengeQuestion {\n      id\n      date\n      incompleteChallengeCount\n      streakCount\n      type\n      __typename\n    }\n    __typename\n  }\n}"
        });

        let query = serde_json::to_string(&json_obj)?;

        let full_data = self
            .client
            .post("https://leetcode.com/graphql/")
            .body(query)
            .send()
            .await?
            .json::<ProblemFullData>()
            .await?;

        Ok((full_data.data.question.titleSlug.clone(), full_data))
    }

    pub async fn show_problm_list(
        &self,
        key_word: &str,
        limit: u32,
    ) -> Result<ProblemData, Errors> {
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

        let query = serde_json::to_string(&query)?;

        let problem_info = self
            .client
            .post("https://leetcode.com/graphql/")
            .body(query)
            .send()
            .await?
            .text()
            .await?;

        Ok(serde_json::from_str::<ProblemData>(&problem_info)?)
    }

    pub fn problem_builder(&self) -> ProblemBuilder {
        ProblemBuilder {
            client: self.client.clone(),
            key_word: String::new(),
            limit: 5,
            category: String::new(),
            filters: Filters::default(),
        }
    }

    async fn get_question_name(&self, name: String) -> Result<String, Errors> {
        let query = json!({
            "query": "query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) { problemsetQuestionList: questionList( categorySlug: $categorySlug limit: $limit skip: $skip filters: $filters ) { questions: data { titleSlug } } }",
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

        let query = serde_json::to_string(&query)?;

        let problem_info = self
            .client
            .post("https://leetcode.com/graphql/")
            .body(query)
            .send()
            .await?
            .text()
            .await?;

        let parsed_data: ProblemData = serde_json::from_str(&problem_info)?;

        Ok(parsed_data.data.problemsetQuestionList.questions[0]
            .titleSlug
            .clone())
    }

    pub async fn my_profile(&self) -> Result<MyProfile, Errors> {
        Ok(MyProfile {
            client: self.client.clone(),
            fav_lists: Self::fetch_fav_list_data(&self).await?,
        })
    }

    pub fn find_profile(&self, username: &str) -> UserProfile {
        UserProfile {
            client: self.client.clone(),
            username: String::from(username),
        }
    }

    async fn fetch_fav_list_data(&self) -> Result<FavoriteList, Errors> {
        let query = json!({
            "operationName": "favoritesList",
            "variables": {},
            "query": "query favoritesList {
                favoritesLists {
                    allFavorites {
                        idHash
                        name
                        description
                        viewCount
                        creator
                        isWatched
                        isPublicFavorite
                        questions {
                            questionId
                            status
                            title
                            titleSlug
                            __typename
                        }
                        __typename
                    }
                    watchedFavorites {
                        idHash
                        name
                        description
                        viewCount
                        creator
                        isWatched
                        isPublicFavorite
                        questions {
                            questionId
                            status
                            title
                            titleSlug
                            __typename
                        }
                        __typename
                    }
                    __typename
                }
                userStatus {
                    username
                    __typename
                }
            }"
        });

        let query = serde_json::to_string(&query)?;

        let list_data = self
            .client
            .post("https://leetcode.com/graphql/")
            .body(query)
            .send()
            .await?
            .text()
            .await?;

        Ok(serde_json::from_str::<FavoriteList>(&list_data)?)
    }
}

#[derive(Debug)]
pub enum ProgrammingLanguage {
    CPP,
    Java,
    Python,
    Python3,
    C,
    CSharp,
    JavaScript,
    TypeScript,
    Ruby,
    Swift,
    Go,
    Bash,
    Scala,
    Kotlin,
    Rust,
    PHP,
    Racket,
    Erlang,
    Elixir,
    Dart,
    Pandas,
    React,
}
