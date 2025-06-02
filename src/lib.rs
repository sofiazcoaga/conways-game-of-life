pub const DEFAULT_CELL_AREA: f32 = 400.; // arbitrary value for now
pub const DEFAULT_CELL_LEN: f32 = 20.; // arbitrary value for now

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    start_x: f32,
    start_y: f32,
    alive: bool,
}
impl Cell {
    pub fn new(start_x: f32, start_y: f32, alive: bool) -> Self {
        Cell {
            start_x,
            start_y,
            alive,
        }
    }
    // Initial function to calculate whether two cells are adjacent -- TODO make smaller
    pub fn is_adjacent(&self, cell: &Cell) -> bool {
        let x = cell.start_x;
        let y = cell.start_y;
        let x_plus_len = x == self.start_x + DEFAULT_CELL_LEN;
        let x_minus_len = x == self.start_x - DEFAULT_CELL_LEN;
        let y_equal = y.eq(&self.start_y);
        let y_minus_len = y == self.start_y - DEFAULT_CELL_LEN;
        let y_plus_len = y == self.start_y + DEFAULT_CELL_LEN;
        let x_equal = x.eq(&self.start_x);
        let cond_1 = (x_minus_len || x_plus_len) && (y_equal || y_minus_len || y_plus_len);
        let cond_2 = x_equal && (y_minus_len || y_plus_len);
        let result = cond_1 || cond_2;
        result
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
}

// Function to generate the initial grid (none of the cells alive before the game starts)
pub fn generate_cells(screen_width: f32, screen_height: f32) -> Vec<Cell> {
    // Calculate how many cells there will be based on the screen area and the default cell area
    let cells_amount = ((screen_height * screen_width) / DEFAULT_CELL_AREA) as i32;
    let mut offset_x = 0.;
    let mut offset_y = 0.;
    let mut cells: Vec<Cell> = Vec::new();

    for _ in 0..cells_amount {
        let new_cell = Cell {
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
pub fn update_cells(cells: &mut Vec<Cell>) {
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
