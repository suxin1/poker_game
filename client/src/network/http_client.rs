use bevy::prelude::*;
use bevy_http_client::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(HttpClientPlugin);
}