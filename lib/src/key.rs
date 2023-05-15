use aes::Aes256;
use aes::cipher::Key;
use aes_gcm_siv::{Aes256GcmSiv, KeyInit};
use aes_gcm_siv::aead::consts::U32;
use aes_gcm_siv::aead::generic_array::GenericArray;
use aes_gcm_siv::aead::OsRng;
use anyhow::ensure;
use base64::Engine;
use base64::engine::general_purpose;

pub fn generate() -> anyhow::Result<String> {
    let key= Aes256GcmSiv::generate_key(&mut OsRng);
    let encoded = general_purpose::URL_SAFE_NO_PAD.encode(key);
    Ok(encoded)
}

pub fn from_base64_string(string: String) -> anyhow::Result<GenericArray<u8, U32>> {
    let decoded = general_purpose::URL_SAFE_NO_PAD.decode(&string)?;
    ensure!(decoded.len() == 32, "Key size should be 32 chars / 32 bytes / 256 bits");
    Ok(Key::<Aes256>::from_slice(&decoded.as_slice()).to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_1() {
        assert_eq!(43, generate().unwrap().len());
    }

    #[test]
    fn from_base64_string_1() {
        let actual = from_base64_string(String::from("ioawhegpouaw"));
        assert!(actual.is_err());
        assert_eq!(actual.err().unwrap().to_string(), String::from("Key size should be 32 chars / 32 bytes / 256 bits"));
    }

    #[test]
    fn from_base64_string_2() {
        let decoded = general_purpose::URL_SAFE_NO_PAD.decode("aW9hd2hlZ3BvdWF3aHBndWlod2FwZ2hwaXV3YWhnaGg").unwrap();
        let expected = Key::<Aes256>::from_slice(decoded.as_slice());
        let actual = from_base64_string(String::from("aW9hd2hlZ3BvdWF3aHBndWlod2FwZ2hwaXV3YWhnaGg"));
        assert!(actual.is_ok());
        assert_eq!(expected, &actual.unwrap());
    }
}