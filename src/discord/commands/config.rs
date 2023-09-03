use crate::discord::commands::constants;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::{CommandDataOption, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Default, Debug, Clone)]
pub struct Config {
    pub account_id: String,
    pub decimals: u8,
    pub required_amount: u64,
    pub standard: String,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = HashMap::new();
        map.insert(
            constants::ACCOUNT_ID_KEY,
            CommandDataOptionValue::String(self.account_id.clone()),
        );
        map.insert(
            constants::DECIMALS_KEY,
            CommandDataOptionValue::Integer(self.decimals as i64),
        );
        map.insert(
            constants::REQUIRED_AMOUNT_KEY,
            CommandDataOptionValue::Integer(self.required_amount as i64),
        );
        map.insert(
            constants::STANDARD_KEY,
            CommandDataOptionValue::String(self.standard.clone()),
        );
        write!(f, "{:?}", map)
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let admin_permission = serenity::model::Permissions::ADMINISTRATOR;
    command
        .name("config")
        .description("Define your configuration for azero.gg")
        .default_member_permissions(admin_permission)
        .create_option(|option| {
            option
                .name(constants::STANDARD_KEY)
                .description("Smart contract standard")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice("NATIVE", "NATIVE")
                .add_string_choice("PSP22", "PSP22")
                .add_string_choice("PSP34", "PSP34")
        })
        .create_option(|option| {
            option
                .name(constants::REQUIRED_AMOUNT_KEY)
                .description("Minimum required token amount")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
        .create_option(|option| {
            option
                .name(constants::ACCOUNT_ID_KEY)
                .description("Account ID of your token")
                .kind(CommandOptionType::String)
                .required(false)
        })
        .create_option(|option| {
            option
                .name(constants::DECIMALS_KEY)
                .description("Decimals")
                .kind(CommandOptionType::Integer)
                .required(false)
        })
}

pub async fn run(options: &[CommandDataOption]) -> Config {
    let mut config = Config::default();
    for option in options {
        match option.name.as_str() {
            constants::ACCOUNT_ID_KEY => {
                if let Some(val) = &option.value {
                    config.account_id = val.to_string();
                }
            }
            constants::DECIMALS_KEY => {
                if let Some(val) = &option.value {
                    config.decimals = val.as_i64().unwrap() as u8; //command option type is integer so unwrap is safe
                }
            }
            constants::REQUIRED_AMOUNT_KEY => {
                if let Some(val) = &option.value {
                    config.required_amount = val.as_i64().unwrap() as u64; //command option type is integer so unwrap is safe
                }
            }
            constants::STANDARD_KEY => {
                if let Some(val) = &option.value {
                    config.standard = val.to_string();
                }
            }
            _ => {}
        }
    }
    config
}
