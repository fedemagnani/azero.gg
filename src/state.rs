use crate::discord::Config;
use sp_runtime::AccountId32;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct State {
    /// Maps discord guild id to its config object
    pub bot_config: HashMap<u64, Config>,
    /// Maps Discord user-id to an Aleph-zero account
    pub verified_accounts: HashMap<u64, AccountId32>,
}

lazy_static! {
    pub static ref STATE: Arc<Mutex<State>> = Arc::new(Mutex::new(State::default()));
}
