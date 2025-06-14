#[cfg(not(target_arch = "wasm32"))]
mod init_native;

mod http_client;
#[cfg(target_arch = "wasm32")]
mod init_wasm;
mod renet2;

use bevy::prelude::*;

pub const PROTOCOL_ID: u64 = 7;

pub const SERVER_ADDR: &str = "127.0.0.1:8080";

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((http_client::plugin, renet2::plugin));
    #[cfg(target_arch = "wasm32")]
    app.add_plugins(init_wasm::plugin);

    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins(init_native::plugin);
}
