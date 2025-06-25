pub mod state;
pub mod reducer;
mod combination;
mod error;

pub mod prelude {
    pub use crate::the_hidden_card::combination::{Combination, HandAnalyzer};
    pub use crate::the_hidden_card::state::{GameState, Stage};
    pub use crate::the_hidden_card::reducer;
}