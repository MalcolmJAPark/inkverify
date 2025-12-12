pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<u8>, // 0 = Dead, 1 = Live (u8 is faster than bool for CPU alignment)
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            cells: vec![0; width * height],
        }
    }

    // Helper to handle wrapping (Toroidal topology)
    pub fn get_index(&self, row: isize, col: isize) -> usize {
        let r = (row.rem_euclid(self.height as isize)) as usize;
        let c = (col.rem_euclid(self.width as isize)) as usize;
        r * self.width + c
    }
}
