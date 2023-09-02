use std::sync::Mutex;
use crate::discord::Config;

#[derive(Default)]
pub struct State {
    pub bot_config: Config,
}

lazy_static! {
    pub static ref STATE: Mutex<State> = Mutex::new(State::default());
}