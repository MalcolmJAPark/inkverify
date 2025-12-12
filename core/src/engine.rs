use crate::grid::Grid;
use sha2::{Digest, Sha256};

// --- Part 1: Deterministic Randomness (The Seeder) ---

/// A minimal Pseudo-Random Number Generator (Xorshift32).
/// It allows us to turn a 32-byte hash into infinite random bytes
/// without needing the heavy 'rand' crate.
struct Xorshift32 {
    state: u32,
}

impl Xorshift32 {
    fn new(seed: u32) -> Self {
        // State cannot be 0, so we handle that edge case.
        let state = if seed == 0 { 0xDEADBEEF } else { seed };
        Xorshift32 { state }
    }

    /// Generates the next random u8 (byte) and advances state.
    fn next_u8(&mut self) -> u8 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        // Return the lowest 8 bits as a byte (0-255)
        (x & 0xFF) as u8
    }

    /// Generates a random boolean (0 or 1) based on a threshold.
    /// Returns 1 (Ink) or 0 (Empty).
    fn next_bool(&mut self) -> u8 {
        // 50% chance of being alive
        if self.next_u8() > 128 { 1 } else { 0 }
    }
}

// --- Part 2: Grid Generation ---

/// Generates the initial grid state from the user's credentials.
/// 
/// Process:
/// 1. Hash (Username + Password) using SHA-256.
/// 2. Use the first 4 bytes of the hash to seed our Xorshift PRNG.
/// 3. Fill the grid with deterministic noise.
pub fn generate_grid_from_seed(username: &str, password: &str, width: usize, height: usize) -> Grid<u8> {
    // 1. Create the Master Hash
    let mut hasher = Sha256::new();
    hasher.update(username.as_bytes());
    hasher.update(password.as_bytes());
    let result = hasher.finalize();

    // 2. Extract a Seed (Take the first 4 bytes to make a u32)
    // We use standard array slicing and conversion here.
    let seed_bytes: [u8; 4] = result[0..4].try_into().expect("Hash failed");
    let seed_u32 = u32::from_be_bytes(seed_bytes);

    // 3. Initialize RNG
    let mut rng = Xorshift32::new(seed_u32);

    // 4. Fill Data Vector
    let mut cells = Vec::with_capacity(width * height);
    for _ in 0..(width * height) {
        cells.push(rng.next_bool());
    }

    Grid::from_raw(width, height, cells)
}

// --- Part 3: The Simulation Logic ---

/// Runs the simulation for a fixed number of steps.
/// Returns the Final Grid state.
pub fn run_simulation(mut grid: Grid<u8>, steps: usize) -> Grid<u8> {
    // We use double-buffering implicitly by creating a new grid every tick.
    // In production Rust, we might swap two buffers to save allocation,
    // but for clarity/simplicity, we just generate `next` from `current`.
    for _ in 0..steps {
        grid = tick(&grid);
    }
    grid
}

/// Advances the grid by one time step (t -> t+1).
fn tick(current: &Grid<u8>) -> Grid<u8> {
    let width = current.width();
    let height = current.height();
    let mut next = Grid::new(width, height);

    // Iterate over every cell
    // Note: This loops is O(N^2). In Rust --release, this gets auto-vectorized by LLVM.
    for y in 0..height {
        for x in 0..width {
            let neighbors = count_neighbors(current, x as isize, y as isize);
            let state = current.get(x as isize, y as isize);
            
            // Apply the "Ink" Rule
            // Current Rule: Similar to Conway's Game of Life but adjusted for chaos.
            // 1 = Ink, 0 = Water
            let new_state = match (state, neighbors) {
                (1, 2) | (1, 3) => 1, // Survival: Ink stays if it has 2 or 3 neighbors
                (0, 3) => 1,          // Birth: Ink spreads to empty spots with 3 neighbors
                _ => 0,               // Death: Overcrowding (>3) or Loneliness (<2)
            };
            
            next.set(x, y, new_state);
        }
    }
    next
}

/// Counts the number of active ("Ink") neighbors around a coordinate.
/// Uses the Grid's internal wrapping (torus) logic automatically.
/// 
/// Neighborhood: Moore (8 surrounding cells)
/// [ ][ ][ ]
/// [ ][X][ ]
/// [ ][ ][ ]
fn count_neighbors(grid: &Grid<u8>, x: isize, y: isize) -> u8 {
    let mut count = 0;
    
    // Check all 8 directions
    // We use an array of offsets to avoid writing 8 if-statements.
    let offsets = [
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1)
    ];

    for (dx, dy) in offsets.iter() {
        // If the neighbor is 1 (Ink), add to count
        if grid.get(x + dx, y + dy) == 1 {
            count += 1;
        }
    }
    
    count
}
