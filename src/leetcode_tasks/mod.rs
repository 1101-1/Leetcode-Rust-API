pub mod descr;
pub mod subm_send;
pub mod subm_show;
pub mod task_actions;
pub mod taskfulldata;
pub mod test_send;

use serde::Deserialize;

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub(crate) struct Rate {
    pub likes: u32,
    pub dislikes: u32,
}
