use std::sync::Mutex;
use crate::discord::Config;
use std::collections::HashMap;
use sp_runtime::AccountId32;

#[derive(Default)]
pub struct State {
    /// Maps discord guild id to its config object
    pub bot_config: HashMap<u64, Config>,
    /// Maps Discord user-id to an Aleph-zero account
    pub verified_accounts: HashMap<u64, AccountId32>
}

lazy_static! {
    pub static ref STATE: Mutex<State> = Mutex::new(State::default());
}