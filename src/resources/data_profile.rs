use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NotificationStatus {
    lastModified: i64,
    numUnread: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserStatus {
    userId: i64,
    isSignedIn: bool,
    isMockUser: bool,
    isPremium: bool,
    isVerified: bool,
    username: String,
    avatar: String,
    isAdmin: bool,
    isSuperuser: bool,
    permissions: Vec<String>,
    isTranslator: bool,
    activeSessionId: i64,
    checkedInToday: bool,
    notificationStatus: NotificationStatus,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileData {
    data: Data,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    userStatus: UserStatus,
}