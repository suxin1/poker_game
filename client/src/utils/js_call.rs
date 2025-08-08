use shared::Player;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn get_userinfo() -> JsValue;
}

#[wasm_bindgen]
pub fn get_user_info() -> Player {
    let js_value = get_userinfo();

    let data: Player = js_value.into_serde().unwrap();
    data
}