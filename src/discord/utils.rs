use crate::webserver::routes::validate_native_balance;
use serenity::http::Http;
use serenity::model::prelude::{GuildId, UserId};

pub struct Utils;
// use tokio::time

impl Utils {
    pub async fn assign_role(guild_id: u64, user_id: u64) {
        let discord_token =
            dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        let role_name =
            dotenv::var("AUTH_ROLE_NAME").expect("Expected AUTH_ROLE_NAME in environment");
        let guild_id = GuildId(guild_id);
        let user_id = UserId(user_id);
        let http = Http::new(&discord_token);
        let ids = GuildId(guild_id.0).roles(&http).await.unwrap();
        // we take just the values
        let mut roles = ids.values().collect::<Vec<_>>();
        // we filter the roles by name
        roles.retain(|role| role.name == role_name);
        // we take the first role
        let role = roles.first().unwrap();
        // we add the role to the user
        if let Some(mut member) = guild_id.member(&http, user_id).await.ok() {
            // Add the role to the user
            if let Err(why) = member.add_role(&http, role.id).await {
                eprintln!("Failed to add role to user: {:?}", why);
            }
        }
    }

    #[allow(unused)]
    pub async fn purge(guild_id: u64) {
        let discord_token =
            dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        let http = Http::new(&discord_token);
        let ids = GuildId(guild_id).members(&http, None, None).await;
        if let Ok(ids) = ids {
            for id in ids {
                // We recover the accountID from the shared state
                let state = crate::state::STATE.lock().await;
                let account = state.verified_accounts.get(&id.user.id.0);
                if let Some(account) = account {
                    let condition = validate_native_balance(account.clone(), 0).await;
                    // let condition = true;
                    if !condition {
                        // We need to kick the user
                        if let Err(why) = id.kick(&http).await {
                            eprintln!("Failed to kick user: {:?}", why);
                        }
                    }
                }
            }
        }
    }
}

#[tokio::test]
pub async fn add_role_to_user() {
    let user_id = 1017777904236638270;
    let guild_id = 1147497062557024289;
    Utils::assign_role(guild_id, user_id).await;
}

#[tokio::test]
pub async fn get_all_users() {
    let discord_token = dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let guild_id = GuildId(1147497062557024289);
    let http = Http::new(&discord_token);
    let ids = GuildId(guild_id.0).members(&http, None, None).await;
    println!("{:#?}", ids);
}

#[tokio::test]
pub async fn kick_everyone() {
    let _discord_token = dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let guild_id = 1147497062557024289;
    Utils::purge(guild_id).await;
}
