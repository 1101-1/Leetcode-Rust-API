use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FavoriteList {
    pub data: FavoritesLists,
}

impl Default for FavoriteList {
    fn default() -> Self {
        Self {
            data: FavoritesLists {
                favoritesLists: FavoritesNode {
                    allFavorites: Vec::new(),
                    watchedFavorites: Vec::new(),
                },
                userStatus: MeNode {
                    username: String::new(),
                    __typename: String::new(),
                },
            },
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FavoritesLists {
    pub favoritesLists: FavoritesNode,
    pub userStatus: MeNode,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FavoritesNode {
    pub allFavorites: Vec<FavoriteNode>,
    pub watchedFavorites: Vec<FavoriteNode>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FavoriteNode {
    pub idHash: String,
    pub name: String,
    pub description: String,
    pub viewCount: i32,
    pub creator: String,
    pub isWatched: bool,
    pub isPublicFavorite: bool,
    pub questions: Vec<QuestionNode>,
    pub __typename: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuestionNode {
    pub questionId: String,
    pub status: Option<String>,
    pub title: String,
    pub titleSlug: String,
    pub __typename: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MeNode {
    pub username: String,
    pub __typename: String,
}
