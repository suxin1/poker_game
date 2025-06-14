use crate::screens::Screen;
use bevy::prelude::*;
use bevy_renet2::netcode::{NetcodeClientPlugin, NetcodeTransportError};
use bevy_renet2::prelude::{RenetClientPlugin, client_disconnected};

pub fn plugin(app: &mut App) {
    app.add_plugins((RenetClientPlugin, NetcodeClientPlugin));
    app.add_systems(Update, panic_on_error_system);

}

fn panic_on_error_system(mut renet_error: EventReader<NetcodeTransportError>) {
    for err in renet_error.read() {
        info!("{}", err);
    }
}

