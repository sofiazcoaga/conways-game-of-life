pub use conways_game_of_life::*;
const TEST_SCREEN_WIDTH: f32 = 200.;
const TEST_SCREEN_HEIGHT: f32 = 200.;

#[test]
fn it_starts_cells_vector() {
    let mut cells_grid: Vec<Cell> = Vec::new();
    assert_eq!(cells_grid.len(), 0);
    cells_grid = generate_cells(TEST_SCREEN_WIDTH, TEST_SCREEN_HEIGHT);
    let cells_amount = (TEST_SCREEN_WIDTH * TEST_SCREEN_HEIGHT) / DEFAULT_CELL_AREA;
    assert_eq!(cells_grid.len(), cells_amount as usize);
}

#[test]
fn it_tells_whether_two_cells_are_adjacent() {
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
fn it_updates_cells_grid() {
    /* Let's pretend we have a grid of nine cells starting from origin (0;0).
    |x| |x|                     | |x| |
    | |x| |  --> should become  | |x|x|
    | | |x|                     | | | |
     */

    // first line
    let cell_1 = Cell::new(0., 0., true);
    let cell_2 = Cell::new(cell_1.x() + DEFAULT_CELL_LEN, cell_1.y(), false);
    let cell_3 = Cell::new(cell_1.x() + DEFAULT_CELL_LEN * 2., cell_1.y(), true);

    // second line
    let cell_4 = Cell::new(cell_1.x(), cell_1.y() + DEFAULT_CELL_LEN, false);
    let cell_5 = Cell::new(
        cell_1.x() + DEFAULT_CELL_LEN,
        cell_1.y() + DEFAULT_CELL_LEN,
        true,
    );
    let cell_6 = Cell::new(
        cell_1.x() + 2. * DEFAULT_CELL_LEN,
        cell_1.y() + DEFAULT_CELL_LEN,
        false,
    );

    //third line
    let cell_7 = Cell::new(cell_1.x(), cell_1.y() + 2. * DEFAULT_CELL_LEN, false);
    let cell_8 = Cell::new(
        cell_1.x() + DEFAULT_CELL_LEN,
        cell_1.y() + 2. * DEFAULT_CELL_LEN,
        false,
    );
    let cell_9 = Cell::new(
        cell_1.x() + 2. * DEFAULT_CELL_LEN,
        cell_1.y() + 2. * DEFAULT_CELL_LEN,
        true,
    );

    let mut cells_grid = vec![
        cell_1, cell_2, cell_3, cell_4, cell_5, cell_6, cell_7, cell_8, cell_9,
    ];

    update_cells(&mut cells_grid);

    assert!(!cells_grid[0].is_alive());
    assert!(cells_grid[1].is_alive());
    assert!(!cells_grid[2].is_alive());
    assert!(!cells_grid[3].is_alive());
    assert!(cells_grid[4].is_alive());
    assert!(cells_grid[5].is_alive());
    assert!(!cells_grid[6].is_alive());
    assert!(!cells_grid[7].is_alive());
    assert!(!cells_grid[8].is_alive());
}
