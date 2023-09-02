#[test]
fn testerino() {
    use subxt_signer::{ bip39::Mnemonic, ecdsa };

    let keypair = ecdsa::dev::alice();
    let message = b"Hello!";

    let signature = keypair.sign(message);
    let public_key = keypair.public_key();
    assert!(ecdsa::verify(&signature, message, &public_key));
}
