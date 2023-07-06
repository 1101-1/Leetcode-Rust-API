use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct QueryResponse {
    pub userStatus: UserStatus,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct UserStatus {
    pub isSignedIn: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct GlobalData {
    pub userStatus: UserStatus,
}

#[derive(Debug, Deserialize)]
pub struct CookieData {
    pub data: GlobalData,
}
