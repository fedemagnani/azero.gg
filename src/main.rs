#[macro_use]
extern crate lazy_static;

mod discord;
mod state;
mod webserver;

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
        _ = bot => {
            log::error!("Discord bot has terminated!!")
        }
    }

    log::error!("Shutting down..")
}
