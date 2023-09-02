use warp::{Filter, reply, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::webserver::ecdsa_verify;
use crate::discord::utils::Utils;
use crate::state::STATE;
use sp_runtime::AccountId32;
use substrate_api_client::{
	ac_primitives::{AssetRuntimeConfig},
	rpc::JsonrpseeClient,
	Api, GetAccountInformation,
};

pub type WarpResult<T> = std::result::Result<T, Rejection>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequestPayload {
    discord_id: u64,
    guild_id: u64,
    account_id: String,
    signature: String,
}

/// POST /auth
pub fn auth_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("auth").and(warp::post()).and(warp::body::json::<AuthRequestPayload>()).and_then(auth_handler)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    message: String
}

/// This handler will handle the POST /auth request:
/// 1. verify the message signature 
/// 2. verify the account holds the required tokens
pub async fn auth_handler(payload: AuthRequestPayload) -> WarpResult<impl Reply> {
    // make sure that the user account id is valid
    let user_account_id = if let Ok(id) = AccountId32::from_str(&payload.account_id) {
        id
    } else {
        return Err(warp::reject())
    };

    // check that the wallet signature is valid via ECDSA verification
    if !ecdsa_verify::verify_sig(
        &payload.signature,
        &payload.account_id
    ) {
        return Err(warp::reject());
    }
    
    {
        let mut state = STATE.lock().unwrap();
        if let Some(_config) = state.bot_config.get(&payload.guild_id) {
            // NOTE: we are checking for native balance here,
            // in the future we will add support for any PSP token
            // via the `config` struct

            if validate_balance(user_account_id.clone()).await {
                // register the verified user in the state
                state.verified_accounts.entry(payload.discord_id).or_insert_with(|| user_account_id);

                // assign the user the defined role on Discord
                Utils::assign_role(payload.guild_id, payload.discord_id).await;
            }
        }
    } // state lock dropped here
    

    Ok(reply::json(&Response {message: "User authorized.".to_string() }))
}

pub async fn validate_balance(user_account_id: AccountId32) -> bool {
    let client = JsonrpseeClient::async_new("wss://ws.test.azero.dev:443").await.unwrap();
	let api = Api::<AssetRuntimeConfig, _>::new(client).unwrap();

	let account_id =
		AccountId32::from_str("5CkwWMbgqGJVNe6Vacaeckd8bi8zNnWDQYyh82xsZuhornWx").unwrap();
	let balance = api.get_account_data(&account_id).unwrap().unwrap();
    balance.free > 0

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
            .body(r#"{"discordId": 123, "accountId": "456", "signature": "0x789", "guildId": 152}"#)
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
