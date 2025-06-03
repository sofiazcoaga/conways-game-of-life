use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
pub struct GameCell {
    x: u16,
    y: u16,
    alive: bool,
}
impl GameCell {
    /// GameCell constructor, allows the creation of a new instance of this type.
    pub fn new(x: u16, y: u16, alive: bool) -> Self {
        GameCell { x, y, alive }
    }
    /// Calculates whether two cells are adjacent (vertically, horizontally or diagonally)
    pub fn is_adjacent(&self, cell: &GameCell) -> bool {
        let distance_x = match self.x().cmp(&cell.x()) {
            Ordering::Greater => self.x() - cell.x(),
            _ => cell.x() - self.x(),
        };
        let distance_y = match self.y().cmp(&cell.y()) {
            Ordering::Greater => self.y() - cell.y(),
            _ => cell.y() - self.y(),
        };
        (distance_x.pow(2) + distance_y.pow(2)).isqrt() == 1
    }

    /// Returns whether a cell is alive or not
    pub fn is_alive(&self) -> bool {
        self.alive
    }

    /// Returns the x component of the cell's position
    pub fn x(&self) -> u16 {
        self.x
    }

    /// Returns the y component of the cell's position
    pub fn y(&self) -> u16 {
        self.y
    }

    /// Sets the alive field of the cell to a specified value
    pub fn set_alive(&mut self, alive: bool) {
        self.alive = alive;
    }
}

/// Generates the initial grid (none of the cells alive before the game starts) based on the specified
/// length and width for the board
pub fn generate_cells(screen_width_in_cells: u16, screen_height_in_cells: u16) -> Vec<GameCell> {
    let mut offset_x = 0;
    let mut offset_y = 0;
    let mut cells: Vec<GameCell> = Vec::new();
    let cells_amount = screen_height_in_cells * screen_width_in_cells;

    for _ in 0..cells_amount {
        let new_cell = GameCell {
            x: offset_x,
            y: offset_y,
            alive: false,
        };
        cells.push(new_cell);

        if offset_x == screen_width_in_cells - 1 {
            offset_x = 0;
            offset_y += 1;
        } else {
            offset_x += 1;
        }
    }
    cells
}

/// Updates current cells state (basically if they remain alive or not)
pub fn update_cells(cells: &mut Vec<GameCell>) {
    let cells_reference = cells.clone();
    for s in cells {
        // Obtain previously alive neighbours
        let neighbors_amount = cells_reference
            .iter()
            .filter(|x| s.is_adjacent(x) && x.is_alive())
            .count();
        match (s.is_alive(), neighbors_amount) {
            (true, 2..=3) => {} // A live cell with two or three neighbours is kept alive
            (false, 3) => s.alive = true, // A dead cell with three neighbours is revived
            _ => s.alive = false, // Any other case (dead or alive) cell will be dead
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SCREEN_WIDTH: u16 = 20;
    const TEST_SCREEN_HEIGHT: u16 = 20;

    #[test]
    fn start_cells_vector_test() {
        let mut cells_grid: Vec<GameCell> = Vec::new();
        assert_eq!(cells_grid.len(), 0);
        cells_grid = generate_cells(TEST_SCREEN_WIDTH, TEST_SCREEN_HEIGHT);
        let cells_amount = TEST_SCREEN_WIDTH * TEST_SCREEN_HEIGHT;
        assert_eq!(cells_grid.len(), cells_amount as usize);
    }

    #[test]
    fn two_cells_are_adjacent_test() {
        let first_cell = GameCell::new(0, 0, true);
        let adjacent_cell = GameCell::new(1, 0, false);
        let not_adjacent_cell = GameCell::new(3, 2, true);

        assert!(first_cell.is_adjacent(&adjacent_cell));
        assert!(!first_cell.is_adjacent(&not_adjacent_cell));
    }

    #[test]
    fn update_cells_grid_test() {
        /* Let's pretend we have a grid of nine cells starting from origin (0;0).
        |x| |x|                     | |x| |
        | |x| |  --> should become  | |x|x|
        | | |x|                     | | | |

        Also for reference:
        |0|1|2|
        |3|4|5|   (cell index)
        |6|7|8|
         */

        // first line
        let cell_0 = GameCell::new(0, 0, true);
        let cell_1 = GameCell::new(1, 0, false);
        let cell_2 = GameCell::new(2, 0, true);

        // second line
        let cell_3 = GameCell::new(0, 1, false);
        let cell_4 = GameCell::new(1, 1, true);
        let cell_5 = GameCell::new(2, 1, false);

        // third line
        let cell_6 = GameCell::new(0, 2, false);
        let cell_7 = GameCell::new(1, 2, false);
        let cell_8 = GameCell::new(2, 2, true);

        let mut cells_grid = vec![
            cell_0, cell_1, cell_2, cell_3, cell_4, cell_5, cell_6, cell_7, cell_8,
        ];

        update_cells(&mut cells_grid);

        // Cells that should not be alive
        for i in [0, 2, 3, 6, 7, 8] {
            assert!(!cells_grid[i].is_alive());
        }

        // Cells that should be alive
        for i in [1, 4, 5] {
            assert!(cells_grid[i].is_alive());
        }
    }
}
