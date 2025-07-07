#[cfg(not(target_arch = "wasm32"))]
mod native;

mod http_client;
#[cfg(target_arch = "wasm32")]
mod wasm;
mod renet2;
mod init;

use bevy::prelude::*;

pub use init::{MessageEvent};

pub const PROTOCOL_ID: u64 = 7;

#[cfg(feature = "dev")]
pub const SERVER_ADDR: &str = "http://127.0.0.1:8081";
#[cfg(not(feature = "dev"))]
pub const SERVER_ADDR: &str = "http://poker_server.wasdqe.top:1080";

#[cfg(all(target_arch = "wasm32", feature = "dev"))]
pub const WS_URL: &str = "ws://poker_server.wasdqe.top:1447";

#[cfg(all(target_arch = "wasm32"))]
pub const WS_URL: &str = "ws://[::]:8085";

#[cfg(all(not(target_arch = "wasm32"), feature = "dev"))]
pub const NATIVE_SOCKET_ADDR: &str = "[::]:8082";

#[cfg(all(not(target_arch = "wasm32"), not(feature = "dev")))]
pub const NATIVE_SOCKET_ADDR: &str = "poker_server.wasdqe.top:1445";

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((http_client::plugin, renet2::plugin, init::plugin));
    // #[cfg(target_arch = "wasm32")]
    // app.add_plugins(wasm::plugin);
    //
    // #[cfg(not(target_arch = "wasm32"))]
    // app.add_plugins(native::plugin);
}
