mod puzzle;

pub mod config;
pub mod runner;
pub mod solver;

pub use puzzle::Puzzle;

// re-export paste (so it can be used in exported macros)
pub use paste;
