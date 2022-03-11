use wasm_bindgen::prelude::*;
// use libsignal_bridge_macros::*;
// use libsignal_protocol::error::Result;
// use libsignal_protocol::*;
// use static_assertions::const_assert_eq;
// use std::convert::TryFrom;
// use uuid::Uuid;


// bridge_handle!(PrivateKey, ffi = privatekey, jni = ECPrivateKey);

#[wasm_bindgen]
pub fn helloWorld() -> String {
    "Hell o World!!!".to_string()
}

// #[wasm_bindgen]
// pub fn ECPrivateKey_Generate() -> PrivateKey {
//     let mut rng = rand::rngs::OsRng;
//     let keypair = KeyPair::generate(&mut rng);
//     keypair.private_key
// }