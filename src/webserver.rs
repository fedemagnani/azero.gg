

#[derive(Clone)]
pub struct WarpImpl;

pub type WarpResult<T> = std::result::Result<T, Rejection>;

impl WarpImpl {
    pub async fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
        health()
            .recover(handle_rejection)
    }
}

#[async_trait]
impl WebServer for WarpImpl {
    type Port = u16;

    async fn serve_http(&self, port: Self::Port) {
        let api = WarpImpl::routes().await;
        let with_log = api.with(warp::log("api"));
        warp::serve(with_log)
            .run((Ipv4Addr::UNSPECIFIED, port))
            .await;
    }
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    error!("Route rejection: {:?}", err);
    Ok(warp::reply::with_status(
        warp::reply::json(&InternalServerError.to_string()),
        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    ))
    
}