pub fn tick(current: &Grid) -> Grid {
    let mut next = Grid::new(current.width, current.height);

    for y in 0..current.height {
        for x in 0..current.width {
            let neighbors = count_neighbors(current, x, y);
            let state = current.get(x, y);
            
            // Apply The Rule (Example: Conway's Game of Life)
            let new_state = match (state, neighbors) {
                (1, 2) | (1, 3) => 1, // Stay alive
                (0, 3) => 1,          // Reproduction
                _ => 0,               // Die (Overcrowding/Loneliness)
            };
            
            next.set(x, y, new_state);
        }
    }
    next
}
