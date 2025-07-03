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
pub const SERVER_PORT: &str = "8080";

#[cfg(feature = "dev")]
pub const SERVER_ADDR: &str = "http://127.0.0.1";

#[cfg(not(feature = "dev"))]
pub const SERVER_ADDR: &str = "http://www.wasdqe.top";
// pub const SERVER_ADDR: &str = "http://[240e:331:e00:139:a236:bcff:fe23:ead]";

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((http_client::plugin, renet2::plugin, init::plugin));
    // #[cfg(target_arch = "wasm32")]
    // app.add_plugins(wasm::plugin);
    //
    // #[cfg(not(target_arch = "wasm32"))]
    // app.add_plugins(native::plugin);
}
