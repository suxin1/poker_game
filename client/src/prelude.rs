#![allow(unused_imports)]

pub use core::fmt::Debug;
pub use core::hash::Hash;
pub use core::marker::PhantomData;
pub use core::time::Duration;

pub use bevy::audio::Volume;
pub use bevy::color::palettes::tailwind::*;
pub use bevy::diagnostic::FrameCount;
pub use bevy::ecs::entity_disabling::Disabled;
pub use bevy::ecs::spawn::SpawnIter;
pub use bevy::ecs::spawn::SpawnWith;
pub use bevy::image::{ImageLoaderSettings, ImageSampler};
pub use bevy::input::common_conditions::*;
pub use bevy::math::vec2;
pub use bevy::math::vec3;
pub use bevy::platform::collections::HashMap;
pub use bevy::platform::collections::HashSet;
pub use bevy::prelude::*;
pub use bevy::sprite::Anchor;
pub use bevy::ui::FocusPolicy;
pub use bevy::ui::Val::*;

pub use bevy_asset_loader::prelude::*;
pub use iyes_progress::prelude::*;
// pub use leafwing_input_manager::common_conditions::*;
pub use rand::prelude::*;
pub use serde::Deserialize;
pub use serde::Serialize;
pub use tiny_bail::prelude::*;

pub use crate::utils::prelude::*;
pub use crate::core::AppSystems;
pub use crate::core::pause::Pause;
pub use crate::core::pause::PausableSystems;
pub use crate::theme::prelude::*;