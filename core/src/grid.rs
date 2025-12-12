use std::fmt;

/// The core Memory-Hard container.
///
/// We use a generic 'T' to allow flexibility (e.g., u8 for 256 states, or bool for binary).
/// For InkVerify, we will primarily use Grid<u8>.
///
/// Performance Note:
/// Using a flat vector `Vec<T>` instead of `Vec<Vec<T>>` improves CPU cache locality
/// and prevents memory fragmentation. It allows the CPU to pre-fetch data efficiently.
#[derive(Clone, Debug, PartialEq)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T> Grid<T>
where
    T: Clone + Copy + Default,
{
    /// Creates a new grid of the specified size, initialized with default values (0).
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            cells: vec![T::default(); width * height],
        }
    }

    /// Creates a new grid from a raw vector of data.
    /// Panics if the vector size does not match width * height.
    /// Useful when initializing the grid from a hash seed.
    pub fn from_raw(width: usize, height: usize, cells: Vec<T>) -> Self {
        assert_eq!(
            cells.len(),
            width * height,
            "Cell count does not match grid dimensions"
        );
        Grid {
            width,
            height,
            cells,
        }
    }

    /// Returns the width of the grid.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Converts 2D (x, y) coordinates into a 1D index for the flat vector.
    ///
    /// Implements Toroidal Topology (Wrap-around):
    /// - If x is -1 (left of edge), it wraps to width-1 (right edge).
    /// - If y is height (below bottom), it wraps to 0 (top).
    /// This ensures there are no "walls" to stop the chaos expansion.
    #[inline]
    fn get_index(&self, x: isize, y: isize) -> usize {
        // rem_euclid calculates the true mathematical modulo, handling negative numbers correctly.
        // In Rust, -1 % 10 = -1, but -1.rem_euclid(10) = 9.
        let y_wrapped = y.rem_euclid(self.height as isize) as usize;
        let x_wrapped = x.rem_euclid(self.width as isize) as usize;

        y_wrapped * self.width + x_wrapped
    }

    /// READS a cell's value at (x, y).
    /// Safe to call with negative coordinates due to wrapping.
    pub fn get(&self, x: isize, y: isize) -> T {
        let idx = self.get_index(x, y);
        // We use unsafe for maximum speed in production, but safe indexing here for stability.
        self.cells[idx] 
    }

    /// WRITES a value to a cell at (x, y).
    /// Note: inputs are `usize` because we only write to valid coordinates
    /// during the update loop.
    pub fn set(&mut self, x: usize, y: usize, value: T) {
        let idx = y * self.width + x;
        self.cells[idx] = value;
    }

    /// Returns a reference to the underlying raw data.
    /// Critical for the final step where we hash the entire grid state.
    pub fn as_raw(&self) -> &[T] {
        &self.cells
    }
}

// --- Display Implementation for Debugging ---
// Allows you to print the grid to the console with `println!("{}", grid);`
// Renders the grid as ASCII art.
impl fmt::Display for Grid<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Only print a small section if the grid is huge to avoid flooding the console
        let display_limit = 64; 
        let h = self.height.min(display_limit);
        let w = self.width.min(display_limit);

        writeln!(f, "Grid Preview ({}x{}):", w, h)?;
        for y in 0..h {
            for x in 0..w {
                let cell = self.cells[y * self.width + x];
                // 0 is Empty (Space), 1 is Ink (Block)
                let symbol = if cell > 0 { "█" } else { "." };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        if self.height > display_limit {
            writeln!(f, "... (truncated)")?;
        }
        Ok(())
    }
}
