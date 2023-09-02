#[test]
fn testerino() {
    use subxt_signer::ecdsa;
    use sp_runtime::AccountId32;

    let test_account_id = "5C5WL6qGzkkwAvdnYW5LN55Vb3z5foVLWbQNE7by2vSPy4Xc".to_string();
    let test_signature = "0xcc3b02bbe5ed07d07ffc5a6698f40dd8536bf6f5775a3de001119ba09523163b184fc4bd7712de9374f11c524b15b5029435998dea6d51bc07cf488137821880".to_string();
    
    let message = format!("I want to login with Azero.GG to verify my identity: {}", test_account_id);    

    let sig_slice: [u8; 65] = test_signature.as_bytes().try_into().unwrap();
    let signature = ecdsa::Signature(sig_slice);
    let account_id = AccountId32::from_str(&test_account_id).unwrap();
    let s = ecdsa::verify(&payload.signature, message, account_id);

    let signature = keypair.sign(message);
    let public_key = keypair.public_key();
    assert!(subxt_signer::ecdsa::verify(&signature, message, &public_key));
}
