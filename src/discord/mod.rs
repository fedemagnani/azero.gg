mod commands;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::{Member, RoleId, Guild};
use serenity::prelude::*;

pub use commands::config::*;
struct Handler;


#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, _ctx: Context, _new_member: Member) {
        let username = _new_member.user.name.clone();
        // We define a greeting message here
        let greeting = format!("Welcome to the server, @{}!", username);
        println!("Sending welcome message to {}", username);
        // We reply to the user sending the message with the greeting
        if let Err(why) = _new_member.user.direct_message(&_ctx.http, |m| {
            // We send a embed with a button to the user
            m.embed(|e| {
                e.title("Welcome to the server!");
                // e.description("Click the button below to get started!");
                e
            })
            // We add a button to the message hiding a link to the website
            .components(|c| {
                c.create_action_row(|a| {
                    a.create_button(|b| {
                        b.label(greeting);
                        b.style(serenity::model::application::component::ButtonStyle::Link);
                        b.url("http://localhost:3000/");
                        b
                    })
                })
            })
            // m.content(greeting)
        }).await {
            println!("Cannot send welcome message to {}: {:?}", username, why);
        }

    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);
            let guild_id = command.guild_id;
            let content = match command.data.name.as_str() {
                "config" => {
                    // We check the role, if the user doesn't have the admin role we return
                    Some(commands::config::run(&command.data.options).await)
                },
                _ => None,
            };

            if let Some(config) = content {
                // we get the role Ids of the discord server
                if let Some(guild_id) = &guild_id {
                    let ids = GuildId(guild_id.0).roles(&ctx.http).await.unwrap();
                    // We check if the role "Authenticated" is present
                    let auth_role = ids.iter().find(|(_,x)| x.name == "Authenticated");
                    if auth_role.is_none() {
                        // If not, we create it
                        let role = GuildId(guild_id.0).create_role(&ctx.http, |r| {
                            r.name("Authenticated")
                        }).await.unwrap();
                    } 
                    // let auth_role_id = ids.iter().find(|(a,x)| x.name == "Authenticated").unwrap();
                }
                

                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content(config.clone()))
                    })
                    .await
                {
                    println!("Cannot respond to slash command: {}", why);
                }

                // Update global state
                let mut state = crate::state::STATE.lock().unwrap();
                // using guild_id as string, we insert the config into the state
                state.bot_config.insert(guild_id.unwrap().to_string(), config);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        // println!("{} is connected!", ready.user.name);

        // If we want commands specific to a certain channel, use the following

        // let guild_id = GuildId(
        //     env::var("GUILD_ID")
        //         .expect("Expected GUILD_ID in environment")
        //         .parse()
        //         .expect("GUILD_ID must be an integer"),
        // );

        // let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
        //     commands
        //         .create_application_command(|command| commands::config::register(command))
        // })
        // .await;

        // println!("I now have the following guild slash commands: {:#?}", commands);


        let guild_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::config::register(command)
        })
        .await;

        // As soon as the discord bot gets into the server, it will create a role called "Authenticated" if not present



        println!("I created the following global slash command: {:#?}", guild_command);
    }
}


#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    use super::*;

    #[tokio::test]
    async fn test() {
        // Configure the client with your Discord bot token in the environment.
        let token = dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment");

        // Build our client.
        let mut client = Client::builder(token, GatewayIntents::GUILD_MEMBERS)
            .event_handler(Handler)
            .await
            .expect("Error creating client");

        // Finally, start a single shard, and start listening to events.
        //
        // Shards will automatically attempt to reconnect, and will perform
        // exponential backoff until it reconnects.
        
        // if let Err(why) = client.start().await {
        //     println!("Client error: {:?}", why);
        // }
    }

    use serenity::model::id::{GuildId, RoleId, UserId};
    use serenity::http::Http;

    #[tokio::test]
    pub async fn add_role_to0xiguana() {
    // Replace with your actual user, guild, and role IDs
    let user_id = UserId(123456789012345678);
    let guild_id = GuildId(987654321098765432);
    let role_id = RoleId(123456789012345678);
    let discord_token = dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new(&discord_token);

    // Check if the user is a member of the guild (server)
    if let Some(mut member) = guild_id.member(&http, user_id).await.ok() {
        // Add the role to the user
        if let Err(why) = member.add_role(&http, role_id).await {
            eprintln!("Failed to add role to user: {:?}", why);
        }
    }
    }
}
