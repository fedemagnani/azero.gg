use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::{CommandDataOption, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;
use std::collections::HashMap;
use std::fmt::Display;
use crate::discord::commands::constants;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand{
    command
    .name("ao")
    .description("AO!")
}

