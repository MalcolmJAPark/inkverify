pub mod grid;
pub mod engine;

use wasm_bindgen::prelude::*;
use sha2::{Sha256, Digest};
pub use grid::Grid;

// Re-exports
pub use engine::{generate_grid_from_seed, run_simulation};

// --- WASM INTERFACE ---
// Everything below this line is for the Browser

#[wasm_bindgen]
pub fn prove_work(username: &str, password: &str, steps: usize) -> String {
    // 1. Generate the Grid (Memory Hard step)
    // We use a fixed size (e.g., 500x500) for web challenges to ensure
    // it runs in <1 second on most laptops.
    let width = 500;
    let height = 500;
    
    let initial_grid = generate_grid_from_seed(username, password, width, height);

    // 2. Run the Simulation (CPU/Memory Bandwidth step)
    let final_grid = run_simulation(initial_grid, steps);

    // 3. Hash the result
    // We return the Hex String so JavaScript can send it to the server.
    let mut hasher = Sha256::new();
    hasher.update(final_grid.as_raw());
    let result = hasher.finalize();
    
    hex::encode(result)
}
