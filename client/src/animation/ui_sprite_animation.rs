//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, animate_sprite);
    app.insert_resource(AnimationTimer(Timer::from_seconds(
        0.1,
        TimerMode::Repeating,
    )));
}

#[derive(Component, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Resource, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

fn animate_sprite(
    time: Res<Time>,
    mut animation_timer: ResMut<AnimationTimer>,
    mut query: Query<(&AnimationIndices, &mut ImageNode)>,
) {
    let timer = &mut animation_timer.0;
    timer.tick(time.delta());

    if timer.just_finished() {
        for (indices, mut sprite) in &mut query {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}
