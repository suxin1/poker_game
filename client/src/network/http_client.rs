use crate::screens::ScreenState;
use bevy::prelude::*;
use bevy_http_client::prelude::*;
use crate::network::init::ClientConnectionInfo;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(HttpClientPlugin);
    app.add_systems(
        Update,
        handle_error.run_if(in_state(ScreenState::Title)),
    );
}

fn handle_http_response_error_system(mut http_error: EventReader<HttpResponseError>) {
    for err in http_error.read() {
        info!("{:?}", err);
    }
}


fn handle_error(mut ev_error: EventReader<TypedResponseError<ClientConnectionInfo>>) {
    for error in ev_error.read() {
        error!("Error retrieving IP: {}", error.err);
    }
}