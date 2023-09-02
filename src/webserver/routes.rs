use warp::{Filter, reply, Rejection, Reply};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

pub type WarpResult<T> = std::result::Result<T, Rejection>;

/// GET /auth/{discord_id}/{account_id}
pub fn auth_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("auth" / String  / String).and(warp::get()).and_then(auth_handler)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    id: Uuid
}

pub async fn auth_handler(discord_id: String, account_id: String) -> WarpResult<impl Reply> {
    // TODO:
    // 1. this needs to receive the association: discordId -> accountId
    // 2. with this, we need to query the blockchain to see if the user has the 
    //    required tokens (PSP22, PSP34, ideally this will be configurable)
    // 3. if the user has the required tokens, we need to assign them a role
    //    in the server
    
    Ok(reply::json(&Response {id: discord_id.parse().unwrap() }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;
    use uuid::Uuid;
    use warp::{test::request, Filter};


    #[tokio::test]
    async fn test_auth() {
        let order_id = Uuid::new_v4();
        let api = warp::path!("auth" / String / String)
            .and(warp::get())
            .and_then(auth_handler);
        let resp = request()
            .method("GET")
            .path(&format!("/auth/{}", order_id))
            .reply(&api)
            .await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
