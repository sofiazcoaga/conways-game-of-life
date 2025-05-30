const DEFAULT_SQUARE_AREA: f32 = 400.; // arbitrary value for now
const DEFAULT_SQUARE_LEN: f32 = 20.; // arbitrary value for now

#[derive(Clone)]
struct Square {
    start_x: f32,
    start_y: f32,
    len: f32,
    alive: bool
}

// Function to generate the initial grid (none of the cells alive before the game starts)
fn generate_squares(screen_width: f32, screen_height: f32) -> Vec<Square> {
    // Calculate how many cells there will be based on the screen area and the default square area
    let squares_amount = ((screen_height*screen_width) / DEFAULT_SQUARE_AREA) as i32;
    let mut offset_x = 0.;
    let mut offset_y = 0.;
    let mut squares: Vec<Square> = Vec::new();

    for _ in 0..squares_amount {
        let new_square = Square{
            start_x: offset_x,
            start_y: offset_y,
            len: DEFAULT_SQUARE_LEN,
            alive: false,
        };
        squares.push(new_square);

        if offset_x == screen_width - DEFAULT_SQUARE_LEN {
            offset_x = 0.;
            offset_y += DEFAULT_SQUARE_LEN;
        } else {
            offset_x += DEFAULT_SQUARE_LEN;
        }
    }
    squares
}
