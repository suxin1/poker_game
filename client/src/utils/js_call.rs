use log::info;
use shared::Player;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn get_userinfo() -> JsValue;
}

pub fn get_user_info() -> Result<Player, JsValue>  {
    let js_value = get_userinfo();
    info!("get_user_info: {:?}", js_value);
    let data: Player = serde_wasm_bindgen::from_value(js_value)?;
    info!("get_user_info: {:?}", data);
    Ok(data)
}