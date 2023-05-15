mod setup;

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use lib::key;

#[wasm_bindgen]
pub fn decrypt(input_text: String, key: String) -> Result<String, JsValue> {
    return match key::from_base64_string(key) {
        Ok(ok) => {
            return match lib::decrypt::run(&input_text, ok) {
                Ok(ok) => { Ok(ok) }
                Err(err) => { Err(JsValue::from(format!("{:#?}", err))) }
            }
        }
        Err(err) => { Err(JsValue::from(format!("{:#?}", err))) }
    }
}

#[wasm_bindgen]
pub fn encrypt(input_text: String, key: String) -> Result<String, JsValue> {
    return match key::from_base64_string(key) {
        Ok(ok) => {
            return match lib::encrypt::run(&input_text, ok) {
                Ok(ok) => { Ok(ok) }
                Err(err) => { Err(JsValue::from(err.to_string())) }
            }
        }
        Err(err) => { Err(JsValue::from(format!("{:#?}", err))) }
    }
}

#[wasm_bindgen]
pub fn generate_key() -> Result<String, JsValue> {
    return match key::generate() {
        Ok(ok) => { Ok(ok) },
        Err(err) => { Err(JsValue::from(err.root_cause().to_string())) }
    }
}