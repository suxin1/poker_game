//! The game's main screen states and transitions between them.

mod gameplay;
mod loading;
mod splash;
mod title;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<ScreenState>();

    app.add_plugins((
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum ScreenState {
    #[default]
    Splash,
    Title,
    Loading,
    Gameplay,
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ScreenRoot {
    pub ui: Entity,
}

impl Configure for ScreenRoot {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl FromWorld for ScreenRoot {
    fn from_world(world: &mut World) -> Self {
        Self {
            ui: world
                .spawn((
                    Name::new("ScreenUi"),
                    Node::DEFAULT.full_size(),
                    Pickable::IGNORE,
                ))
                .id(),
        }
    }
}
