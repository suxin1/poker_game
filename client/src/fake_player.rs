use fake::Fake;
use crate::prelude::*;

use fake::faker::name::raw::*;
use fake::locales::ZH_CN;
use shared::Player;

/// 创建一个假玩家, 用于测试
/// 在正式环境中，游戏应有一个唯一的 Player 资源
pub(super) fn plugin(app: &mut App) {
    #[cfg(not(target_arch = "wasm32"))]
    use std::time::SystemTime;
    #[cfg(target_arch = "wasm32")]
    use web_time::SystemTime;

    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let name:String = Name(ZH_CN).fake();
    let player = Player {
        id: client_id,
        name,
        avatar: None,
    };
    info!("generate fake player: {:?}", player);

    app.insert_resource(player);
}
