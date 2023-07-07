use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct NotificationStatus {
    pub lastModified: i64,
    pub numUnread: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct UserStatus {
    pub userId: i64,
    pub isSignedIn: bool,
    pub isMockUser: bool,
    pub isPremium: bool,
    pub isVerified: bool,
    pub username: String,
    pub avatar: String,
    pub isAdmin: bool,
    pub isSuperuser: bool,
    pub permissions: Vec<String>,
    pub isTranslator: bool,
    pub activeSessionId: i64,
    pub checkedInToday: bool,
    pub notificationStatus: NotificationStatus,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileData {
    pub data: Data,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub userStatus: UserStatus,
}
