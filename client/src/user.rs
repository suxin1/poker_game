use std::env;
use crate::prelude::*;

use shared::Player;

/// 从浏览器获取用户信息
pub(super) fn plugin(app: &mut App) {

    #[cfg(target_arch = "wasm32")]
    let player = crate::utils::js_call::get_user_info();

    app.insert_resource(player);
}
