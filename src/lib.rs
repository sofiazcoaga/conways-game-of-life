pub const DEFAULT_CELL_AREA: f32 = 400.; // arbitrary value for now
pub const DEFAULT_CELL_LEN: f32 = 20.; // arbitrary value for now

#[derive(Clone, Copy, Debug)]
pub struct GameCell {
    start_x: f32,
    start_y: f32,
    alive: bool,
}
impl GameCell {
    pub fn new(start_x: f32, start_y: f32, alive: bool) -> Self {
        GameCell {
            start_x,
            start_y,
            alive,
        }
    }
    // Initial function to calculate whether two cells are adjacent -- TODO make smaller
    pub fn is_adjacent(&self, cell: &GameCell) -> bool {
        // This is Pythagoras which finds the distance from a cell's start
        // point to another cell's start point that is diagonally placed
        let max_allowed_distance = (DEFAULT_CELL_LEN.powi(2) * 2.0).sqrt();
        // Calculates the distance between the two cells starting points
        let distance = ((cell.x() - self.x()).powi(2) + (cell.y() - self.y()).powi(2)).sqrt();
        // Distance can either be the max distance or the default edge len if the adjacent cell is not diagonally placed
        distance.eq(&DEFAULT_CELL_LEN) || distance.eq(&max_allowed_distance)
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }
    pub fn x(&self) -> f32 {
        self.start_x
    }
    pub fn y(&self) -> f32 {
        self.start_y
    }

    pub fn set_alive(&mut self, alive: bool) {
        self.alive = alive;
    }
}

// Function to generate the initial grid (none of the cells alive before the game starts)
pub fn generate_cells(screen_width: f32, screen_height: f32) -> Vec<GameCell> {
    // Calculate how many cells there will be based on the screen area and the default cell area
    let cells_amount = ((screen_height * screen_width) / DEFAULT_CELL_AREA) as i32;
    let mut offset_x = 0.;
    let mut offset_y = 0.;
    let mut cells: Vec<GameCell> = Vec::new();

    for _ in 0..cells_amount {
        let new_cell = GameCell {
            start_x: offset_x,
            start_y: offset_y,
            alive: false,
        };
        cells.push(new_cell);

        if offset_x == screen_width - DEFAULT_CELL_LEN {
            offset_x = 0.;
            offset_y += DEFAULT_CELL_LEN;
        } else {
            offset_x += DEFAULT_CELL_LEN;
        }
    }
    cells
}

// Function to update current cells state (basically if they remain alive or not)
pub fn update_cells(cells: &mut Vec<GameCell>) {
    let cells_reference = cells.clone();
    for s in cells {
        // Obtain previously alive neighbours
        let neighbors_amount = cells_reference
            .iter()
            .filter(|x| s.is_adjacent(x) && x.is_alive())
            .count();
        if s.is_alive() {
            match neighbors_amount {
                0..2 => s.alive = false, // Less than two alive neighbours -> cell dies
                2..=3 => s.alive = true, // Two or three alive neighbours -> cell lives
                4.. => s.alive = false,  // More than three alive neighbours -> cell dies
            }
        } else if !s.is_alive() && neighbors_amount == 3 {
            s.alive = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SCREEN_WIDTH: f32 = 200.;
    const TEST_SCREEN_HEIGHT: f32 = 200.;

    #[test]
    fn start_cells_vector_test() {
        let mut cells_grid: Vec<GameCell> = Vec::new();
        assert_eq!(cells_grid.len(), 0);
        cells_grid = generate_cells(TEST_SCREEN_WIDTH, TEST_SCREEN_HEIGHT);
        let cells_amount = (TEST_SCREEN_WIDTH * TEST_SCREEN_HEIGHT) / DEFAULT_CELL_AREA;
        assert_eq!(cells_grid.len(), cells_amount as usize);
    }

    #[test]
    fn two_cells_are_adjacent_test() {
        let first_cell = GameCell::new(DEFAULT_CELL_LEN * 2., DEFAULT_CELL_LEN, true);
        let adjacent_cell = GameCell::new(first_cell.x() + DEFAULT_CELL_LEN, first_cell.y(), false);
        let not_adjacent_cell = GameCell::new(
            first_cell.x() + 3. * DEFAULT_CELL_AREA,
            first_cell.y() + 3. * DEFAULT_CELL_AREA,
            true,
        );

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
        let cell_0 = GameCell::new(0., 0., true);
        let cell_1 = GameCell::new(cell_0.x() + DEFAULT_CELL_LEN, cell_0.y(), false);
        let cell_2 = GameCell::new(cell_0.x() + DEFAULT_CELL_LEN * 2., cell_0.y(), true);

        // second line
        let cell_3 = GameCell::new(cell_0.x(), cell_0.y() + DEFAULT_CELL_LEN, false);
        let cell_4 = GameCell::new(
            cell_0.x() + DEFAULT_CELL_LEN,
            cell_0.y() + DEFAULT_CELL_LEN,
            true,
        );
        let cell_5 = GameCell::new(
            cell_0.x() + 2. * DEFAULT_CELL_LEN,
            cell_0.y() + DEFAULT_CELL_LEN,
            false,
        );

        //third line
        let cell_6 = GameCell::new(cell_0.x(), cell_0.y() + 2. * DEFAULT_CELL_LEN, false);
        let cell_7 = GameCell::new(
            cell_0.x() + DEFAULT_CELL_LEN,
            cell_0.y() + 2. * DEFAULT_CELL_LEN,
            false,
        );
        let cell_8 = GameCell::new(
            cell_0.x() + 2. * DEFAULT_CELL_LEN,
            cell_0.y() + 2. * DEFAULT_CELL_LEN,
            true,
        );

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
