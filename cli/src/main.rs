use std::env;
use std::fs::File;
use std::io::Write;
use std::process;
use std::time::Instant;
use sha2::{Sha256, Digest};
use hex;

use inkverify_core::{generate_grid_from_seed, run_simulation, Grid};

// --- Configuration Struct ---
struct Config {
    username: String,
    password: String,
    width: usize,
    height: usize,
    steps: usize,
    output_file: String,
}

fn main() {
    // 1. Parse Arguments
    let config = parse_args().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        print_usage();
        process::exit(1);
    });

    println!("--- InkVerify Protocol ---");
    println!("[*] User: {}", config.username);
    println!("[*] Grid: {}x{}", config.width, config.height);
    println!("[*] Steps: {}", config.steps);

    // 2. Initialize (The "Seed")
    let start_time = Instant::now();
    println!("[1] Generating Initial Seed...");
    let initial_grid = generate_grid_from_seed(
        &config.username, 
        &config.password, 
        config.width, 
        config.height
    );

    // 3. Simulation (The "Work")
    println!("[2] Running Simulation...");
    let final_grid = run_simulation(initial_grid, config.steps);
    
    let duration = start_time.elapsed();
    println!("[*] Completed in {:.2?}", duration);

    // 4. Hashing (The "Verification")
    let hash = calculate_grid_hash(&final_grid);
    println!("[3] Final Grid Hash: {}", hash);

    // 5. Visualization (The "Proof")
    println!("[4] Saving visual proof to '{}'...", config.output_file);
    save_ppm_image(&final_grid, &config.output_file).expect("Failed to write image");

    println!("--- Done ---");
}

/// Hashes the raw bytes of the grid to create a verification string.
fn calculate_grid_hash(grid: &Grid<u8>) -> String {
    let mut hasher = Sha256::new();
    // We hash the entire flat vector of cells
    hasher.update(grid.as_raw());
    let result = hasher.finalize();
    hex::encode(result)
}

/// Saves the grid as a standard Netpbm (.ppm) image.
/// This format is supported by most image viewers (like Photoshop, GIMP, Preview)
/// and requires NO external libraries to write.
fn save_ppm_image(grid: &Grid<u8>, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    
    // P3 = Text-based PPM header
    // Width Height
    // Max Color Value (255)
    writeln!(file, "P3\n{} {}\n255", grid.width(), grid.height())?;

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let cell = grid.get(x as isize, y as isize);
            
            // Color Logic:
            // Alive (1) = Black (0 0 0)
            // Dead (0)  = White (255 255 255)
            // You can customize this to make "Ink" look blue/purple!
            let (r, g, b) = if cell == 1 {
                (0, 0, 0) // Black
            } else {
                (255, 255, 255) // White
            };

            write!(file, "{} {} {} ", r, g, b)?;
        }
        writeln!(file, "")?; // Newline for file structure
    }
    Ok(())
}

/// Minimal argument parser.
/// Expects: ./cli <username> <password> [width] [height] [steps]
fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err("Not enough arguments.".to_string());
    }

    let username = args[1].clone();
    let password = args[2].clone();
    
    // Defaults
    let width = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(200);
    let height = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(200);
    let steps = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(500);
    let output_file = "proof.ppm".to_string();

    Ok(Config {
        username,
        password,
        width,
        height,
        steps,
        output_file,
    })
}

fn print_usage() {
    println!("Usage:");
    println!("  cargo run -- <username> <password> [width] [height] [steps]");
    println!("Example:");
    println!("  cargo run -- Alice MySecretPass 500 500 1000");
}
