use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Feature {
    pub questionTranslation: bool,
    pub subscription: bool,
    pub signUp: bool,
    pub discuss: bool,
    pub mockInterview: bool,
    pub contest: bool,
    pub store: bool,
    pub chinaProblemDiscuss: bool,
    pub socialProviders: String,
    pub studentFooter: bool,
    pub enableChannels: bool,
    pub dangerZone: bool,
    pub enableSharedWorker: bool,
    pub enableRecaptchaV3: bool,
    pub enableDebugger: bool,
    pub enableDebuggerPremium: bool,
    pub enableAutocomplete: bool,
    pub enableAutocompletePremium: bool,
    pub enableAllQuestionsRaw: bool,
    pub autocompleteLanguages: String,
    pub enableIndiaPricing: bool,
    pub enableReferralDiscount: bool,
    pub maxTimeTravelTicketCount: i32,
    pub enableStoreShippingForm: bool,
    pub enableCodingChallengeV2: bool,
    pub __typename: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct StreakCounter {
    pub streakCount: i32,
    pub daysSkipped: Option<i32>,
    pub currentDayCompleted: bool,
    pub __typename: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct NotificationStatus {
    pub lastModified: i64,
    pub numUnread: i32,
    pub __typename: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct UserStatus {
    pub isSignedIn: bool,
    pub isAdmin: bool,
    pub isStaff: bool,
    pub isSuperuser: bool,
    pub isMockUser: bool,
    pub isTranslator: bool,
    pub isPremium: Option<bool>,
    pub isVerified: bool,
    pub checkedInToday: bool,
    pub username: String,
    pub realName: Option<String>,
    pub avatar: Option<String>,
    pub optedIn: bool,
    pub requestRegion: String,
    pub region: Option<String>,
    pub activeSessionId: i32,
    pub permissions: Vec<String>,
    pub notificationStatus: Option<NotificationStatus>,
    pub completedFeatureGuides: Vec<String>,
    pub __typename: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct GlobalData {
    pub feature: Feature,
    pub streakCounter: Option<StreakCounter>,
    pub currentTimestamp: f64,
    pub userStatus: UserStatus,
    pub siteRegion: String,
    pub chinaHost: String,
    pub websocketUrl: String,
    pub recaptchaKey: String,
    pub recaptchaKeyV2: String,
    pub sitewideAnnouncement: Option<String>,
    pub userCountryCode: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CookieData {
    pub data: GlobalData,
}
