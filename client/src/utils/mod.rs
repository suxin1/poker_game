pub mod extend;

pub mod config;
mod patch;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::config::Config;
    pub use super::config::ConfigHandle;
    pub use super::config::ConfigMut;
    pub use super::config::ConfigRef;

    pub use super::extend::prelude::*;
    
    pub use super::patch::Patch;
}