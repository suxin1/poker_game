use std::env;
use fake::Fake;
use crate::prelude::*;

use fake::faker::name::raw::*;
use fake::locales::ZH_CN;
use shared::Player;

/// 创建一个假玩家, 用于测试
/// 在正式环境中，游戏应有一个唯一的 Player 资源
/// 测试id：
/// 1750835732212
/// 1750835732210
/// 1750835732213
/// 1750835305999
pub(super) fn plugin(app: &mut App) {
    #[cfg(not(target_arch = "wasm32"))]
    use std::time::SystemTime;
    #[cfg(target_arch = "wasm32")]
    use web_time::SystemTime;

    let args = std::env::args().collect::<Vec<String>>();
    let s = if args.len() > 1 {args[1].clone()} else {
        env::var("TEST_ID").unwrap_or_else(|_| {
            let current_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            let client_id = current_time.as_millis() as u64;
            client_id.to_string()
        })
    };
    let client_id = s.parse::<u64>().unwrap();

    // let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    // let client_id = current_time.as_millis() as u64;
    let name:String = Name(ZH_CN).fake();
    let player = Player {
        id: client_id,
        name,
        avatar: None,
    };
    info!("generate fake player: {:?}", player);

    app.insert_resource(player);
}
