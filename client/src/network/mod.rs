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

pub const SERVER_ADDR: &str = "127.0.0.1:8080";

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((http_client::plugin, renet2::plugin, init::plugin));
    // #[cfg(target_arch = "wasm32")]
    // app.add_plugins(wasm::plugin);
    //
    // #[cfg(not(target_arch = "wasm32"))]
    // app.add_plugins(native::plugin);
}
