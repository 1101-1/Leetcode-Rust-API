pub mod cookie;
pub mod descr;
pub mod subm_send;
pub mod subm_show;
pub mod taskfulldata;
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
