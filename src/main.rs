

#[macro_use]
extern crate lazy_static;

mod webserver;
mod discord;
mod state;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let web_server = webserver::WebServer::serve_http(&webserver::WarpImpl, 8080);
    let bot = discord::DiscordBot::spawn();

    tokio::select! {
        _ = web_server => {
            log::error!("Web server has terminated!!")
        }
        _ = bot.start() => {
            log::error!("Discord bot has terminated!!")
        }
    }

    log::error!("Shutting down..")

}
