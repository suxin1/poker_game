use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<InteractionPalette<BoxShadow>>();
}