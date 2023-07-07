use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NotificationsData {
    pub data: Notifications,
}

#[derive(Debug, Deserialize)]
pub struct Notifications {
    pub notifications: NotificationConnection,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct NotificationConnection {
    pub edges: Vec<NotificationEdge>,
    pub pageInfo: PageInfo,
    pub __typename: String,
}

#[derive(Debug, Deserialize)]
pub struct NotificationEdge {
    pub node: NotificationNode,
    pub __typename: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct NotificationNode {
    pub id: String,
    pub notificationId: i32,
    pub modifiedDate: i64,
    pub actioned: bool,
    pub notificationData: NotificationDataNode,
    pub __typename: String,
}

#[derive(Debug, Deserialize)]
pub struct NotificationDataNode {
    pub id: String,
    pub content: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub metadata: String,
    pub __typename: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct PageInfo {
    pub endCursor: String,
    pub hasNextPage: bool,
    pub __typename: String,
}
