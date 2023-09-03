use async_trait::async_trait;
use log::{error, info};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use thiserror::Error;
use warp::{Filter, Rejection, Reply};

pub mod ecdsa_verify;
pub mod routes;

#[derive(Clone)]
pub struct WarpImpl;

impl WarpImpl {
    pub async fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
        routes::auth_route()
        // .with(cors)
    }
}

#[async_trait]
pub trait WebServer {
    type Port;
    async fn serve_http(&self, port: Self::Port);
}

#[async_trait]
impl WebServer for WarpImpl {
    type Port = u16;

    async fn serve_http(&self, port: Self::Port) {
        let api = WarpImpl::routes().await;

        // let cors = warp::cors()
        //     .allow_any_origin()
        //     .allow_methods(vec!["POST", "OPTIONS", "GET"])
        //     .build();

        let with_log = api.with(warp::log("api"));
        // .with(cors);

        info!("Webserver listening on {}", port);

        warp::serve(with_log)
            .run((Ipv4Addr::UNSPECIFIED, port))
            .await;
    }
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    error!("Route rejection: {:?}", err);
    Ok(warp::reply::with_status(
        warp::reply::json(&Errors::InternalServerError.to_string()),
        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum Errors {
    #[error("Internal server error.")]
    InternalServerError,
}
