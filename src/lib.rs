#![warn(
    clippy::dbg_macro,
    clippy::use_self,
    clippy::semicolon_if_nothing_returned,
    clippy::needless_pass_by_value,
    clippy::inconsistent_struct_constructor,
    clippy::trivially_copy_pass_by_ref,
    clippy::explicit_iter_loop
)]

mod binary_tree;
pub use binary_tree::binary_tree;
mod sidewinder;
pub use sidewinder::sidewinder;
mod grid;
pub use grid::Grid;
