// This file is a direct copy from GitoxideLabs/gitoxide
// Source: https://github.com/GitoxideLabs/gitoxide/blob/main/src/plumbing/mod.rs
// No modifications - 100% identical to original

mod main;
pub use main::main;

#[path = "progress.rs"]
mod progress_impl;
pub use progress_impl::show_progress;

mod options;
