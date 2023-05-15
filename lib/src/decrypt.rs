use aes_gcm_siv::{aead::{Aead, KeyInit}, Aes256GcmSiv, Nonce};
use aes_gcm_siv::aead::consts::U32;
use aes_gcm_siv::aead::generic_array::GenericArray;
use anyhow::{bail};
use base64::{Engine};
use base64::engine::general_purpose;

pub fn run(input_text: &String, key: GenericArray<u8, U32>) -> anyhow::Result<String> {
    let decoded = general_purpose::URL_SAFE_NO_PAD.decode(input_text)?;
    let (nonce, encrypted_text) = decoded.split_at(12);
    let cipher = Aes256GcmSiv::new(&key);
    let plaintext = match cipher.decrypt(Nonce::from_slice(nonce), encrypted_text.as_ref()) {
        Ok(ok) => ok,
        Err(err) => bail!("Decrypt.rs {}", err.to_string())
    };
    Ok(String::from_utf8(plaintext)?)
}

#[cfg(test)]
mod tests {
    use crate::key;
    use super::*;

    #[test]
    fn run_1() {
        let key = key::from_base64_string(String::from("peMcWSvLpvKIho1TKLpRO4upxQxTTgT-s4aZIYU1qwc")).unwrap();
        let actual_1 = run(&String::from("QjNKdW5pNzgxMWs0D5zerW4ZKbKDh7DcB1x1AFrE2HnGNoRKhGxb4A"), key);
        let actual_2 = run(&String::from("NWh3Y1VWWGI5QVJWPtfooDycZU62x_shPXsn-JQy_qw4jylZn8s-sg"), key);
        assert!(actual_1.is_ok());
        assert!(actual_2.is_ok());
        assert_eq!(actual_1.unwrap(), String::from("Encrypt this"));
        assert_eq!(actual_2.unwrap(), String::from("Encrypt this"));
    }
}
