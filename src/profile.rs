use serde_json::json;

use crate::{error::Errors, resources::fav_list::FavoriteList};

pub struct MyProfile {
    pub client: reqwest::Client,
    pub fav_lists: FavoriteList,
}

impl MyProfile {
    pub async fn create_fav_list(self, list_name: &str) -> Result<MyProfile, Errors> {
        let query = json!({ "name": list_name });

        let query = serde_json::to_string(&query)?;

        self.client
            .get("https://leetcode.com/list/api/")
            .body(query)
            .send()
            .await?; // let info = Self::s
        Ok(self)
    }
    pub fn show_fav_lists(self) -> FavoriteList {
        self.fav_lists
    }

    pub async fn rename_fav_list(
        self,
        prev_list_name: &str,
        new_list_name: &str,
    ) -> Result<MyProfile, Errors> {
        let id_hash = if let Some(id) = Self::get_id_hash(&self, prev_list_name).await {
            id
        } else {
            return Err(Errors::ApiError("Provided name doesn't found".into()));
        };
        let query = json!({
            "favorite_id_hash": id_hash,
            "is_public_favorite": false,
            "name": new_list_name
        });

        let query = serde_json::to_string(&query)?;

        self.client
            .put("https://leetcode.com/list/api/")
            .body(query)
            .send()
            .await?;
        Ok(self)
    }
    pub async fn set_accessibility(self, list_name: &str) -> Result<MyProfile, Errors> {
        let id_hash = if let Some(id) = Self::get_id_hash(&self, list_name).await {
            id
        } else {
            return Err(Errors::ApiError(
                "Provided name doesn't found in lists".into(),
            ));
        };
        let query = json!({
            "favorite_id_hash": id_hash,
            "is_public_favorite": true,
            "name": list_name
        });

        let query = serde_json::to_string(&query)?;

        self.client
            .put("https://leetcode.com/list/api/")
            .body(query)
            .send()
            .await?;
        Ok(self)
    }

    pub async fn get_share_url(&self, list_name: &str) -> Result<String, Errors> {
        let id_hash = if let Some(id) = Self::get_id_hash(&self, list_name).await {
            id
        } else {
            return Err(Errors::ApiError(
                "Provided name doesn't found in lists".into(),
            ));
        };

        Ok(format!("https://leetcode.com/list/{}", id_hash))
    }

    pub async fn delete_list(self, list_name: &str) -> Result<MyProfile, Errors> {
        let id_hash = if let Some(id) = Self::get_id_hash(&self, list_name).await {
            id
        } else {
            return Err(Errors::ApiError(
                "Provided name doesn't found in lists".into(),
            ));
        };

        self.client
            .delete(format!("https://leetcode.com/list/api/{}", id_hash))
            .send()
            .await?;

        Ok(self)
    }

    async fn get_id_hash(&self, list_name: &str) -> Option<String> {
        for favourite in &self.fav_lists.data.favoritesLists.allFavorites {
            if favourite.name == list_name.to_string() {
                return Some(favourite.idHash.clone());
            }
        }
        None
    }
}

pub struct UserProfile {
    pub client: reqwest::Client,
}

pub struct UserFullData {}
