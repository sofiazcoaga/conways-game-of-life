const DEFAULT_SQUARE_AREA: f32 = 400.; // arbitrary value for now
const DEFAULT_SQUARE_LEN: f32 = 20.; // arbitrary value for now

#[derive(Clone)]
struct Square {
    start_x: f32,
    start_y: f32,
    len: f32,
    alive: bool
}
impl Square {
    // Initial function to calculate whether two cells are adjacent -- TODO make smaller
    fn is_adjacent_and_alive(&self, square: &Square) -> bool {
        let x = square.start_x;
        let y = square.start_y;
        let x_plus_len = x == self.start_x + DEFAULT_SQUARE_LEN;
        let x_minus_len = x == self.start_x - DEFAULT_SQUARE_LEN;
        let y_equal = y.eq(&self.start_y);
        let y_minus_len = y == self.start_y + DEFAULT_SQUARE_LEN;
        let y_plus_len = y == self.start_y + DEFAULT_SQUARE_LEN;
        let x_equal = x.eq(&self.start_x);
        let cond_1 = (x_minus_len || x_plus_len) && (y_equal || y_minus_len || y_plus_len);
        let cond_2 = x_equal && (y_minus_len || y_plus_len);
        (cond_1 || cond_2) || square.alive
    }
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
