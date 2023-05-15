use aes_gcm_siv::{aead::{Aead, KeyInit}, Aes256GcmSiv, Nonce};
use aes_gcm_siv::aead::consts::U32;
use aes_gcm_siv::aead::generic_array::GenericArray;
use anyhow::{bail};
use base64::{Engine};
use base64::engine::general_purpose;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

/// AES itself does not directly use a salt. A situation when you might use a salt in combination with AES is when you are using
/// Password Based Encryption (PBE). In this scheme, a human-memorizable password is used, in combination with a salt, to generate an AES key.
/// A salt is used so that the same password does not always generate the same key; however, because the recipient must
/// be able to generate the correct key, the salt must be transmitted along with the encrypted data.
/// Source: https://stackoverflow.com/a/1950674
///
/// An IV is required if you are using AES in certain block cipher modes, like AES-CBC.
/// In this case, it used to ensure that the same plaintext data under the same key does not always encrypt to the same ciphertext.
/// Again, the IV is required by the recipient to correctly decrypt the data, so it must be transmitted along with the encrypted data.
/// Source: https://stackoverflow.com/a/1950674
///
/// GCM is based on CTR mode and inherits the many-time pad (or two-time pad) problem if a nonce is reused with the same key (
/// very nice example https://twitter.com/angealbertini/status/425561082841690112). A nonce for AES-GCM mode is expected to be 96 bit long.
/// If you're generating nonces randomly, then you are expected to generate a duplicate nonce after 2n/2=248 messages (see Birthday problem).
/// That is, the probability of generating a duplicate nonce is 50% if you generated 248 encrypted messages with the same key.
/// That is quite a lot of messages, but it can happen earlier. Hence CTR mode Nonces often include either a counter or a timer element:
/// something that is guaranteed not to repeat over the lifetime of the key.
/// Source: https://stackoverflow.com/a/36777353 & https://stackoverflow.com/a/36769603
///
/// AES-GCM-SIV is designed to preserve both privacy and integrity even if nonces are repeated. To accomplish this, encryption is a function
/// of a nonce, the plaintext message, and optional additional associated data (AAD). In the event a nonce is misused (i.e. used more than once),
/// nothing is revealed except in the case that same message is encrypted multiple times with the same nonce. When that happens, an attacker is
/// able to observe repeat encryptions, since encryption is a deterministic function of the nonce and message. However, beyond that, no additional
/// information is revealed to the attacker. For this reason, AES-GCM-SIV is an ideal choice in cases that unique nonces cannot be guaranteed, such
/// as multiple servers or network devices encrypting messages under the same key without coordination.
/// Source: https://en.wikipedia.org/wiki/AES-GCM-SIV
///
/// Conclusion:
/// - no salt
/// - random IV / Nonce
/// - use AES-GCM-SIV, so I can create a fully random nonce and dont have to worry about repeated nonces
pub fn run(input_text: &String, key: GenericArray<u8, U32>) -> anyhow::Result<String> {
    let mut rng = thread_rng();
    // unique_nonce 12 chars and 96-bits, unique per message
    let unique_nonce: String = (&mut rng).sample_iter(Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();
    let cipher = Aes256GcmSiv::new(&key);
    let nonce = Nonce::from_slice(unique_nonce.as_bytes());
    let encrypted = match cipher.encrypt(nonce, input_text.as_bytes()) {
        Ok(ok) => ok,
        Err(err) => bail!("Encrypt.rs: {}", err.to_string())
    };
    Ok(
        general_purpose::URL_SAFE_NO_PAD.encode(
            nonce
                .to_vec()
                .into_iter()
                .chain(encrypted)
                .collect::<Vec<u8>>()
        )
    )
}

#[cfg(test)]
mod tests {
    use crate::key;
    use super::*;

    #[test]
    fn run_1() {
        let key = key::from_base64_string(String::from("peMcWSvLpvKIho1TKLpRO4upxQxTTgT-s4aZIYU1qwc")).unwrap();
        let actual_1 = run(&String::from("Encrypt this"), key);
        let actual_2 = run(&String::from("Encrypt this"), key);
        assert!(actual_1.is_ok());
        assert!(actual_2.is_ok());
        let actual_1 = actual_1.unwrap();
        let actual_2 = actual_2.unwrap();
        assert_ne!(&actual_1, &actual_2);
        assert_ne!(actual_1.chars().skip(12).collect::<String>(), actual_2.chars().skip(12).collect::<String>());
    }
}
