use serenity::model::prelude::{GuildId, UserId};
use serenity::http::Http;
pub struct Utils;
impl Utils {
    pub async fn assign_role(guild_id:u64, user_id:u64){
        let discord_token = dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        let role_name = dotenv::var("AUTH_ROLE_NAME").expect("Expected AUTH_ROLE_NAME in environment");
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
}

#[tokio::test]
pub async fn add_role_to_user(){
    let user_id = UserId(569965272052793344);
    let guild_id = GuildId(1147497062557024289);

}