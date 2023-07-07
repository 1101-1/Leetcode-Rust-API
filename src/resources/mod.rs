pub mod beat_stats;
pub mod cookie;
pub mod data_profile;
pub mod descr;
pub mod fav_list;
pub mod lang_stats;
pub mod notification;
pub mod problemfulldata;
pub mod pub_data_profile;
pub mod skill_stats;
pub mod subm_list;
pub mod subm_send;
pub mod subm_show;
pub mod test_send;

use serde::Deserialize;

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct Rate {
    pub likes: u32,
    pub dislikes: u32,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct Descryption {
    pub name: String,
    pub content: String,
}
