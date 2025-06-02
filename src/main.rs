use conways_game_of_life::{Cell, DEFAULT_CELL_LEN, generate_cells, update_cells};
use macroquad::prelude::*;
use std::{thread, time::Duration};

const SCREEN_DELAY_IN_MILLIS: Duration = Duration::from_millis(50);

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let mut game_on = false;
    let mut cell_grid = generate_cells(screen_width(), screen_height());
    let mut initial_cells_position: Vec<Position> = Vec::new();

    loop {
        clear_background(WHITE);
        if !game_on {
            draw_text(
                "Choose some squares and press ENTER to start",
                50.,
                50.,
                30.,
                BLACK,
            );

            for pos in &initial_cells_position {
                draw_rectangle(pos.0, pos.1, DEFAULT_CELL_LEN, DEFAULT_CELL_LEN, BLUE);
            }

            if is_mouse_button_pressed(MouseButton::Left) {
                let mouse_position = mouse_position();
                let mapped_position = map_mouse_position_to_cell(mouse_position);

                //Update existing grid to include selected alive cells
                let cell = cell_grid
                    .iter_mut()
                    .find(|cell| cell.x() == mapped_position.0 && cell.y() == mapped_position.1);

                match cell {
                    None => println!("Selected cell is out of scope"),
                    Some(x) => {
                        x.set_alive(true);
                        initial_cells_position.push(Position(mapped_position.0, mapped_position.1))
                    }
                }
            } else if is_key_down(KeyCode::Enter) {
                game_on = true;
            }
        } else {
            // Include a very brief pause to see the effects more clearly (slower)
            thread::sleep(SCREEN_DELAY_IN_MILLIS);
            draw_grid(&cell_grid);
            update_cells(&mut cell_grid);
        }
        next_frame().await;
    }
}

#[derive(Clone, Copy)]
struct Position(f32, f32);

fn map_mouse_position_to_cell(mouse_position: (f32, f32)) -> (f32, f32) {
    let x = mouse_position.0 - (mouse_position.0 % DEFAULT_CELL_LEN);
    let y = mouse_position.1 - (mouse_position.1 % DEFAULT_CELL_LEN);
    (x, y)
}
fn draw_grid(grid: &Vec<Cell>) {
    for s in grid {
        if s.is_alive() {
            draw_rectangle(s.x(), s.y(), DEFAULT_CELL_LEN, DEFAULT_CELL_LEN, BLUE);
        }
    }
}
