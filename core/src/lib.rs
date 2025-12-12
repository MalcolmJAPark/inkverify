pub mod grid;
pub mod engine;

// Re-export common items for easier access
pub use grid::Grid;
pub use engine::{generate_grid_from_seed, run_simulation};
