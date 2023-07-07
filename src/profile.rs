use serde_json::json;

use crate::{
    error::Errors,
    resources::{
        data_profile::ProfileData, fav_list::FavoriteList, notification::NotificationsData,
    },
};

#[derive(Debug)]
pub struct MyProfile {
    pub(crate) client: reqwest::Client,
    pub(crate) fav_lists: FavoriteList,
}

impl MyProfile {
    pub async fn create_list(&self, list_name: &str) -> Result<&MyProfile, Errors> {
        let query = json!({ "name": list_name });

        let query = serde_json::to_string(&query)?;

        self.client
            .get("https://leetcode.com/list/api/")
            .body(query)
            .send()
            .await?;
        Ok(self)
    }
    pub fn show_lists(&self) -> FavoriteList {
        self.fav_lists.clone()
    }

    pub async fn rename_list(
        &self,
        prev_list_name: &str,
        new_list_name: &str,
    ) -> Result<&MyProfile, Errors> {
        let id_hash = if let Some(id) = Self::get_id_hash(&self, prev_list_name).await {
            id
        } else {
            return Err(Errors::ApiError("Provided name doesn't found".into()));
        };
        let query = json!({
            "favorite_id_hash": id_hash.0,
            "is_public_favorite": id_hash.1,
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

    pub async fn set_public(&self, list_name: &str) -> Result<&MyProfile, Errors> {
        let id_hash = if let Some(id) = Self::get_id_hash(&self, list_name).await {
            id
        } else {
            return Err(Errors::ApiError(
                "Provided name doesn't found in lists".into(),
            ));
        };
        let query = json!({
            "favorite_id_hash": id_hash.0,
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

    pub async fn set_private(&self, list_name: &str) -> Result<&MyProfile, Errors> {
        let id_hash = if let Some(id) = Self::get_id_hash(&self, list_name).await {
            id
        } else {
            return Err(Errors::ApiError(
                "Provided name doesn't found in lists".into(),
            ));
        };
        let query = json!({
            "favorite_id_hash": id_hash.0,
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

        Ok(format!("https://leetcode.com/list/{}", id_hash.0))
    }

    pub async fn delete_list(&self, list_name: &str) -> Result<&MyProfile, Errors> {
        let id_hash = if let Some(id) = Self::get_id_hash(&self, list_name).await {
            id
        } else {
            return Err(Errors::ApiError(
                "Provided name doesn't found in lists".into(),
            ));
        };

        self.client
            .delete(format!("https://leetcode.com/list/api/{}", id_hash.0))
            .send()
            .await?;

        Ok(self)
    }

    async fn get_id_hash(&self, list_name: &str) -> Option<(String, bool)> {
        for favourite in &self.fav_lists.data.favoritesLists.allFavorites {
            if favourite.name == list_name.to_string() {
                return Some((favourite.idHash.clone(), favourite.isPublicFavorite.clone()));
            }
        }
        None
    }

    pub async fn get_notifications(&self) -> Result<NotificationsData, Errors> {
        let operation_name = "fetchNotifications";
        let variables = json!({ "first": 10 });
        let query = r#"query fetchNotifications($first: Int!, $after: String) {
        notifications(first: $first, after: $after) {
            edges {
                node {
                    id
                    notificationId
                    modifiedDate
                    actioned
                    notificationData {
                        id
                        content
                        type
                        metadata
                        __typename
                    }
                    __typename
                }
                __typename
            }
            pageInfo {
                endCursor
                hasNextPage
                __typename
            }
            __typename
        }
        }"#;

        let json_data = json!({
            "operationName": operation_name,
            "variables": variables,
            "query": query,
        });

        let query = serde_json::to_string(&json_data)?;

        let problem_info = self
            .client
            .post("https://leetcode.com/graphql/")
            .body(query)
            .send()
            .await?
            .text()
            .await?;

        Ok(serde_json::from_str::<NotificationsData>(&problem_info)?)
    }

    pub async fn profile_info(&self) -> Result<ProfileData, Errors> {
        let query = r#"
        query globalData {
          userStatus {
            userId
            isSignedIn
            isMockUser
            isPremium
            isVerified
            username
            avatar
            isAdmin
            isSuperuser
            permissions
            isTranslator
            activeSessionId
            checkedInToday
            notificationStatus {
              lastModified
              numUnread
            }
          }
        }
        "#;

        let variables = json!({});
        let operation_name = "globalData";

        let json_data = json!({
            "query": query,
            "variables": variables,
            "operationName": operation_name
        });

        let query = serde_json::to_string(&json_data).unwrap();

        let data_info = self
            .client
            .post("https://leetcode.com/graphql/")
            .body(query)
            .send()
            .await?
            .text()
            .await?;

        Ok(serde_json::from_str::<ProfileData>(&data_info)?)
    }
}
