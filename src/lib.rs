#![warn(
    clippy::dbg_macro,
    clippy::use_self,
    clippy::semicolon_if_nothing_returned,
    clippy::needless_pass_by_value,
    clippy::inconsistent_struct_constructor,
    clippy::trivially_copy_pass_by_ref,
    clippy::explicit_iter_loop
)]

mod aldous_broder;
mod binary_tree;
mod cell;
mod distances;
mod grid;
mod hunt_and_kill;
mod mask;
mod recursive_backtracker;
mod sidewinder;
mod wilsons;

pub use aldous_broder::aldous_broder;
pub use binary_tree::binary_tree;
pub use cell::Cell;
pub use grid::{Grid, GridKind, Masked, Polar, Regular};
pub use hunt_and_kill::hunt_and_kill;
pub use mask::Mask;
pub use recursive_backtracker::recursive_backtracker;
pub use sidewinder::sidewinder;
pub use wilsons::wilsons;
