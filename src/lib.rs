#![warn(
    clippy::dbg_macro,
    clippy::use_self,
    clippy::semicolon_if_nothing_returned,
    clippy::needless_pass_by_value,
    clippy::inconsistent_struct_constructor,
    clippy::trivially_copy_pass_by_ref,
    clippy::explicit_iter_loop
)]

pub mod cell;
mod distances;
mod generators;
mod grid;
pub mod kind;
mod mask;

pub use grid::Grid;
pub use mask::Mask;
