use conways_game_of_life::{GameCell, generate_cells, update_cells};
use macroquad::prelude::*;

const SCREEN_WIDTH_IN_CELLS: u16 = 50;
const SCREEN_HEIGHT_IN_CELLS: u16 = 50;
const DEFAULT_CELL_LEN: f32 = 20.;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let mut game_on = false;
    let mut cell_grid = generate_cells(SCREEN_WIDTH_IN_CELLS, SCREEN_HEIGHT_IN_CELLS);
    let mut initial_cells_position: Vec<Position> = Vec::new();

    loop {
        if game_on {
            break;
        }
        clear_background(WHITE);

        draw_text(
            "Choose some squares and press ENTER to start",
            50.,
            50.,
            30.,
            BLACK,
        );

        for cell in &initial_cells_position {
            let position = map_cell_to_position((cell.0, cell.1), DEFAULT_CELL_LEN);
            draw_rectangle(
                position.0,
                position.1,
                DEFAULT_CELL_LEN,
                DEFAULT_CELL_LEN,
                BLUE,
            );
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_position = mouse_position();
            let mapped_position = map_mouse_position_to_cell(mouse_position);

            // Update existing grid to include selected alive cells
            let cell = cell_grid
                .iter_mut()
                .find(|cell| cell.x() == mapped_position.0 && cell.y() == mapped_position.1);

            match cell {
                None => println!("Selected cell is out of scope"),
                Some(x) => {
                    x.set_alive(true);
                    initial_cells_position.push(Position(mapped_position.0, mapped_position.1));
                }
            }
        } else if is_key_down(KeyCode::Enter) {
            game_on = true;
        }
        next_frame().await;
    }
    loop {
        clear_background(WHITE);
        draw_grid(&cell_grid, DEFAULT_CELL_LEN);
        update_cells(&mut cell_grid);
        next_frame().await;
    }
}

#[derive(Clone, Copy)]
struct Position(u16, u16);

/// Given a mouse position map it to its cell's relative position
fn map_mouse_position_to_cell(mouse_position: (f32, f32)) -> (u16, u16) {
    let x = (mouse_position.0 - (mouse_position.0 % DEFAULT_CELL_LEN)) / DEFAULT_CELL_LEN;
    let y = (mouse_position.1 - (mouse_position.1 % DEFAULT_CELL_LEN)) / DEFAULT_CELL_LEN;
    (x as u16, y as u16)
}

/// Draw every cell that is alive in the grid
fn draw_grid(grid: &Vec<GameCell>, cell_len: f32) {
    for s in grid {
        if s.is_alive() {
            let position = map_cell_to_position((s.x(), s.y()), cell_len);
            draw_rectangle(position.0, position.1, cell_len, cell_len, BLUE);
        }
    }
}

/// Given a relative cell's position map it to screen absolute position
fn map_cell_to_position(cell_pos: (u16, u16), cell_len: f32) -> (f32, f32) {
    (cell_pos.0 as f32 * cell_len, cell_pos.1 as f32 * cell_len)
}
