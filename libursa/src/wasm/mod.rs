#[macro_use]
mod macros;
#[cfg(feature = "bls")]
pub mod bls;

pub mod ed25519;
pub mod encryption;
pub mod secp256k1;

//#[cfg(feature = "cl")]
//pub mod cl;

use keys::{PrivateKey, PublicKey};

use errors::{UrsaCryptoError, UrsaCryptoErrorKind};
use serde;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct WasmPrivateKey(String);
#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct WasmPublicKey(String);

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyPair {
    pk: WasmPublicKey,
    sk: WasmPrivateKey,
}

impl From<&PublicKey> for WasmPublicKey {
    fn from(pk: &PublicKey) -> WasmPublicKey {
        WasmPublicKey(hex::encode(&pk[..]))
    }
}

impl From<PublicKey> for WasmPublicKey {
    fn from(pk: PublicKey) -> WasmPublicKey {
        WasmPublicKey(hex::encode(&pk[..]))
    }
}

impl From<Vec<u8>> for WasmPublicKey {
    fn from(value: Vec<u8>) -> WasmPublicKey {
        WasmPublicKey(hex::encode(value.as_slice()))
    }
}

impl From<&WasmPublicKey> for PublicKey {
    fn from(pk: &WasmPublicKey) -> PublicKey {
        PublicKey(hex::decode(&pk.0).unwrap().to_vec())
    }
}
impl From<WasmPublicKey> for PublicKey {
    fn from(pk: WasmPublicKey) -> PublicKey {
        PublicKey(hex::decode(&pk.0).unwrap().to_vec())
    }
}

impl From<&PrivateKey> for WasmPrivateKey {
    fn from(sk: &PrivateKey) -> WasmPrivateKey {
        WasmPrivateKey(hex::encode(&sk[..]))
    }
}

impl From<PrivateKey> for WasmPrivateKey {
    fn from(sk: PrivateKey) -> WasmPrivateKey {
        WasmPrivateKey(hex::encode(&sk[..]))
    }
}

impl From<&WasmPrivateKey> for PrivateKey {
    fn from(pk: &WasmPrivateKey) -> PrivateKey {
        PrivateKey(hex::decode(&pk.0).unwrap().to_vec())
    }
}

impl From<WasmPrivateKey> for PrivateKey {
    fn from(pk: WasmPrivateKey) -> PrivateKey {
        PrivateKey(hex::decode(&pk.0).unwrap().to_vec())
    }
}

impl From<UrsaCryptoError> for JsValue {
    fn from(err: UrsaCryptoError) -> JsValue {
        let error = format!("{:?}", err);
        JsValue::from_serde(&error).unwrap()
    }
}

fn convert_from_js<T>(val: &JsValue) -> Result<T, UrsaCryptoError>
where
    for<'a> T: serde::Deserialize<'a>,
{
    match val.into_serde() {
        Ok(unwrapped) => Ok(unwrapped),
        Err(_) => Err(UrsaCryptoError::from_msg(
            UrsaCryptoErrorKind::InvalidStructure,
            "Invalid argument".to_string(),
        )),
    }
}
