// #[cfg(test)]
// mod tests {
//     use sp_runtime::AccountId32;
//     use std::str::FromStr;

//     // use sp_core::crypto::Ss58Codec;

//     #[test]
//     fn testerino() {
//         let test_account_id = "5C5WL6qGzkkwAvdnYW5LN55Vb3z5foVLWbQNE7by2vSPy4Xc".to_string();
//         let test_signature = "0xcc3b02bbe5ed07d07ffc5a6698f40dd8536bf6f5775a3de001119ba09523163b184fc4bd7712de9374f11c524b15b5029435998dea6d51bc07cf488137821880".to_string();
//         let message = format!(
//             "I want to login with Azero.GG to verify my identity: {}",
//             test_account_id
//         );
//         // let account_id = AccountId32::from_str("5CkwWMbgqGJVNe6Vacaeckd8bi8zNnWDQYyh82xsZuhornWx").unwrap();
//         // let p_key = AccountKeyring::from_account_id(&account_id).unwrap();
//         // Replace with the actual address (account ID) of the signer
//         let account_id: AccountId32 = test_account_id.parse().unwrap();
//         let account: ecdsa::PublicKey = account_id.into();
//         // Use your Substrate API client to query the public key
//         // let public_key = account(&address).get_public_key();
//         let account_slice: [u8; 33] = test_account_id.as_bytes().try_into().unwrap();
//         let account = ecdsa::PublicKey(account_slice);
//         let sig_slice: [u8; 65] = test_signature.as_bytes().try_into().unwrap();
//         let signature = ecdsa::Signature(sig_slice);
//         // let is_valid = ecdsa::verify(&signature, message, &account);
//         assert!(ecdsa::verify(&signature, message, &account));
//     }
// }
// // Secret phrase `caution juice atom organ advance problem want pledge someone senior holiday very` is account:
// //   Secret seed:       0xc8fa03532fb22ee1f7f6908b9c02b4e72483f0dbd66e4cd456b8f34c6230b849
// //   Public key (hex):  0xd6a3105d6768e956e9e5d41050ac29843f98561410d3a47f9dd5b3b227ab8746
// //   Public key (SS58): 5Gv8YYFu8H1btvmrJy9FjjAWfb99wrhV3uhPFoNEr918utyR
// //   Account ID:        0xd6a3105d6768e956e9e5d41050ac29843f98561410d3a47f9dd5b3b227ab8746
// //   SS58 Address:      5Gv8YYFu8H1btvmrJy9FjjAWfb99wrhV3uhPFoNEr918utyR
