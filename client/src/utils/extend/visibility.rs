use crate::prelude::*;

pub trait VisibilityExt {
    fn from_bool(visible: bool) -> Self;
}

impl VisibilityExt for Visibility {
    fn from_bool(visible: bool) -> Self {
        if visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        }
    }
}