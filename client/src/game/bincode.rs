use bevy::prelude::*;
use bincode::config::Configuration;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(BincodeConfig(bincode::config::standard()));
}

#[derive(Resource)]
pub(crate) struct BincodeConfig(pub Configuration);