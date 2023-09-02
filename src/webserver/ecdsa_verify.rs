// use sp_runtime::AccountId32;
// use std::str::FromStr;
// use subxt_signer::ecdsa;
use aleph_client;

pub fn verify_sig(_signature: &str, _account_id: &str) -> bool {
    // let message = format!("I want to login with Azero.GG to verify my identity: {}", account_id);

    // let account_id: AccountId32 = account_id.parse().unwrap();
    // let account = ecdsa::PublicKey(account_id.into());

    // ecdsa::verify(&signature, message, &account)

    true
}

#[cfg(test)]
mod tests {
    //use sp_core::crypto::{Pair, Public};
    //use sp_keyring::{AccountKeyring, Ed25519Keyring, Sr25519Keyring};
    //use sp_runtime::AccountId32;
    use std::str::FromStr;
    // use sp_core::crypto::Ss58Codec;
    //#[test]
    //fn check_signature() {
    //    /*let account = "5C5WL6qGzkkwAvdnYW5LN55Vb3z5foVLWbQNE7by2vSPy4Xc".to_string();
    //    let signature_hex = "cc3b02bbe5ed07d07ffc5a6698f40dd8536bf6f5775a3de001119ba09523163b184fc4bd7712de9374f11c524b15b5029435998dea6d51bc07cf488137821880".to_string();
    //    let message = format!(
    //        "I want to login with Azero.GG to verify my identity: {}",
    //        account
    //    );
    //    let account_id: AccountId32 = account.parse().unwrap();
    //    let keypair = AccountKeyring::from_account_id(&account_id).unwrap();
    //    let signature = keypair.sign(message.as_bytes());
    //    assert!(ecdsa::verify(&signature, message, &public_key));*/
    //}

    // #[test]
    // fn testerino() {
    //     let keypair = ecdsa::dev::alice();
    //     let message = b"Hello!";
    //     let signature = keypair.sign(message);
    //     let public_key = keypair.public_key();
    //     assert!(ecdsa::verify(&signature, message, &public_key));
    // }
}
