use std::sync::Mutex;
use crate::discord::Config;
use std::collections::HashMap;
#[derive(Default)]
pub struct State {
    pub bot_config: HashMap<String, Config>
}

lazy_static! {
    pub static ref STATE: Mutex<State> = Mutex::new(State::default());
}