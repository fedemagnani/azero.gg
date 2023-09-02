use warp::{Filter, Rejection, Reply};
use std::net::Ipv4Addr;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use log::{error,info};

pub mod routes;
pub mod ecdsa_verify;

#[derive(Clone)]
pub struct WarpImpl;

impl WarpImpl {
    pub async fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
        routes::auth_route()
            .recover(handle_rejection)
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
        let with_log = api.with(warp::log("api"));
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