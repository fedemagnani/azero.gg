use warp::{Filter, reply, Rejection, Reply};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use sp_runtime::AccountId32;

use super::ecdsa_verify;

pub type WarpResult<T> = std::result::Result<T, Rejection>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequestPayload {
    discord_id: String,
    account_id: String,
    signature: String,
}

/// POST /auth
pub fn auth_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("auth").and(warp::post()).and(
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    ).and_then(auth_handler)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    message: String
}

/// This handler will handle the POST /auth request:
/// 1. verify the message signature 
/// 2. verify the account holds the required tokens
/// 3. assign the user the specified role(s)
pub async fn auth_handler(payload: AuthRequestPayload) -> WarpResult<impl Reply> {
    if !ecdsa_verify::verify_sig(
        &payload.signature,
        &payload.account_id
    ) {
        return Err(warp::reject::reject());
    }

    Ok(reply::json(&Response {message: "User authorized.".into() }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;
    use warp::{test::request, Filter};


    #[tokio::test]
    async fn test_auth() {
        let api = warp::path!("auth")
            .and(warp::post())
            .and(warp::body::content_length_limit(1024 * 16).and(warp::body::json()))
            .and_then(auth_handler);
        let resp = request()
            .method("POST")
            .path(&format!("/auth"))
            .body(r#"{"discordId": "123", "accountId": "456", "signature": "789"}"#)
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
