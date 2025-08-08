pub mod extend;

pub mod config;
mod patch;

#[cfg(target_arch = "wasm32")]
pub mod js_call;


#[allow(unused_imports)]
pub mod prelude {
    pub use super::config::Config;
    pub use super::config::ConfigHandle;
    pub use super::config::ConfigMut;
    pub use super::config::ConfigRef;

    pub use super::extend::prelude::*;
    
    pub use super::patch::Patch;
}