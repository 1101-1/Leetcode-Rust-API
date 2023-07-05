pub mod descr;
pub mod subm_send;
pub mod subm_show;
pub mod test_send;
pub mod taskfulldata;

use serde::Deserialize;

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub(crate) struct Rate {
    pub likes: u32,
    pub dislikes: u32,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub(crate) struct Descryption {
    pub name: String,
    pub content: String,
}