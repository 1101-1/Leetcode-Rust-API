use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct UserStatus {
    pub isSignedIn: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct GlobalData {
    pub userStatus: UserStatus,
    pub recaptchaKey: String,
}

#[derive(Debug, Deserialize)]
pub struct CookieData {
    pub data: GlobalData,
}
