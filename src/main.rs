

#[macro_use]
extern crate lazy_static;

mod webserver;
mod discord;
mod state;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    webserver::WebServer::serve_http(&webserver::WarpImpl, 8080).await;
}
