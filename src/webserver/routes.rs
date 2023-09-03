use serde::{Deserialize, Serialize};
use std::str::FromStr;
use warp::{reply, Filter, Rejection, Reply};

use crate::discord::utils::Utils;
use crate::state::STATE;
use crate::webserver::ecdsa_verify;
use aleph_client::pallets::system::SystemApi;
use aleph_client::AccountId;
use aleph_client::Connection;
use serde_with::{serde_as, DisplayFromStr};

pub type WarpResult<T> = std::result::Result<T, Rejection>;

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequestPayload {
    #[serde_as(as = "DisplayFromStr")]
    discord_id: u64,
    #[serde_as(as = "DisplayFromStr")]
    guild_id: u64,
    account_id: String,
    signature: String,
}

/// POST /auth
pub fn auth_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("auth")
        .and(warp::post())
        .and(warp::body::json::<AuthRequestPayload>())
        .and_then(auth_handler)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    message: String,
}

/// This handler will handle the POST /auth request:
/// 1. verify the message signature
/// 2. verify the account holds the required tokens
pub async fn auth_handler(payload: AuthRequestPayload) -> WarpResult<impl Reply> {
    println!("auth_handler: {:?}", payload);

    // make sure that the user account id is valid
    let user_account_id = if let Ok(id) = AccountId::from_str(&payload.account_id) {
        id
    } else {
        return Err(warp::reject());
    };

    println!("verifying signature");
    // check that the wallet signature is valid via ECDSA verification
    if !ecdsa_verify::verify_sig(&payload.signature, &payload.account_id) {
        return Err(warp::reject());
    }

    {
        let mut state = STATE.lock().await;

        println!(
            "checking guild id match: {} == {:?}",
            payload.guild_id,
            state.bot_config.keys().collect::<Vec<_>>()
        );

        if let Some(config) = state.bot_config.get(&payload.guild_id) {
            // NOTE: we are checking for native balance here,
            // in the future we will add support for any PSP token
            // via the `config` struct

            println!("validating balance");
            if validate_native_balance(user_account_id.clone(), config.required_amount).await {
                // register the verified user in the state
                state
                    .verified_accounts
                    .entry(payload.discord_id)
                    .or_insert_with(|| user_account_id);

                println!("assigning role");

                // assign the user the defined role on Discord
                Utils::assign_role(payload.guild_id, payload.discord_id).await;
                return Ok(reply::json(&Response {
                    message: "User authorized.".to_string(),
                }));
            }
        }
    } // state lock dropped here

    Ok(reply::json(&Response {
        message: "Requirements not satisfied.".to_string(),
    }))
}

pub async fn validate_native_balance(user_account_id: AccountId, required_amount: u64) -> bool {
    let conn = Connection::new("wss://ws.test.azero.dev:443").await;
    let free_bal = conn.get_free_balance(user_account_id, None).await;
    log::info!("verifying balance: {}", free_bal.clone());
    free_bal > required_amount.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;
    use warp::{test::request, Filter};

    #[tokio::test]
    async fn test_auth() {
        env_logger::init();

        let api = warp::path!("auth")
            .and(warp::post())
            .and(warp::body::content_length_limit(1024 * 16).and(warp::body::json()))
            .and_then(auth_handler);
        let resp = request()
            .method("POST")
            .path(&format!("/auth"))
            .body(r#"{"discordId": 123, "accountId": "456", "signature": "0x789", "guildId": 152}"#)
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
