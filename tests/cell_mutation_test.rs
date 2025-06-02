pub use conways_game_of_life::*;
const TEST_SCREEN_WIDTH: f32 = 200.;
const TEST_SCREEN_HEIGHT: f32 = 200.;

#[test]
fn start_cells_vector_test() {
    let mut cells_grid: Vec<Cell> = Vec::new();
    assert_eq!(cells_grid.len(), 0);
    cells_grid = generate_cells(TEST_SCREEN_WIDTH, TEST_SCREEN_HEIGHT);
    let cells_amount = (TEST_SCREEN_WIDTH * TEST_SCREEN_HEIGHT) / DEFAULT_CELL_AREA;
    assert_eq!(cells_grid.len(), cells_amount as usize);
}

#[test]
fn two_cells_are_adjacent_test() {
    let first_cell = Cell::new(DEFAULT_CELL_LEN * 2., DEFAULT_CELL_LEN, true);
    let adjacent_cell = Cell::new(first_cell.x() + DEFAULT_CELL_LEN, first_cell.y(), false);
    let not_adjacent_cell = Cell::new(
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
    let cell_0 = Cell::new(0., 0., true);
    let cell_1 = Cell::new(cell_0.x() + DEFAULT_CELL_LEN, cell_0.y(), false);
    let cell_2 = Cell::new(cell_0.x() + DEFAULT_CELL_LEN * 2., cell_0.y(), true);

    // second line
    let cell_3 = Cell::new(cell_0.x(), cell_0.y() + DEFAULT_CELL_LEN, false);
    let cell_4 = Cell::new(
        cell_0.x() + DEFAULT_CELL_LEN,
        cell_0.y() + DEFAULT_CELL_LEN,
        true,
    );
    let cell_5 = Cell::new(
        cell_0.x() + 2. * DEFAULT_CELL_LEN,
        cell_0.y() + DEFAULT_CELL_LEN,
        false,
    );

    //third line
    let cell_6 = Cell::new(cell_0.x(), cell_0.y() + 2. * DEFAULT_CELL_LEN, false);
    let cell_7 = Cell::new(
        cell_0.x() + DEFAULT_CELL_LEN,
        cell_0.y() + 2. * DEFAULT_CELL_LEN,
        false,
    );
    let cell_8 = Cell::new(
        cell_0.x() + 2. * DEFAULT_CELL_LEN,
        cell_0.y() + 2. * DEFAULT_CELL_LEN,
        true,
    );

    let mut cells_grid = vec![
        cell_0, cell_1, cell_2, cell_3, cell_4, cell_5, cell_6, cell_7, cell_8
    ];

    update_cells(&mut cells_grid);

    // Cells that should not be alive
    for i in [0,2,3,6,7,8] {
        assert!(!cells_grid[i].is_alive());
    }

    // Cells that should be alive
    for i in [1,4,5] {
        assert!(cells_grid[i].is_alive());
    }
}
