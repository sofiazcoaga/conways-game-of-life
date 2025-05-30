use std::{thread, time};
use macroquad::prelude::*;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let mut game_on = false;
    let mut mock_grid: Vec<Square> = Vec::new();
    let default_x = screen_width() / 2.;
    let default_y = screen_height() / 2.;
    let mut initial_mock_squares: Vec<(f32, f32)> = vec![(default_x, default_y), (default_x + 100., default_y), (default_x, default_y + 75.), (default_x + 25., default_y + 80.)];

    loop {
        clear_background(WHITE);
        if !game_on {
            // arbitrary for now
            //draw_text("Choose 5 squares and press ENTER", 50., 50., 40., WHITE);
            for p in &initial_mock_squares {
                draw_rectangle(p.0, p.1, 30., 30., PINK);
            }

            if is_key_down(KeyCode::Enter) {
                game_on = true;
            }
        } else {
            thread::sleep(time::Duration::from_millis(1000));
            draw_grid(&mock_grid);
            // update_grid(); --> calculate which cells should live and which should die
        }
            next_frame().await;
        }
}

fn draw_grid(grid: &Vec<Square>) {
    for s in grid {
        if s.alive {
            draw_rectangle(s.start_x, s.start_y, s.len, s.len, PINK);
        }
    }
}

#[derive(Clone)]
struct Square {
    start_x: f32,
    start_y: f32,
    len: f32,
    alive: bool
}
