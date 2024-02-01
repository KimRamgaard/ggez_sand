use ggez::{ContextBuilder, event, GameResult};
use ggez::conf::{WindowMode, WindowSetup};

use crate::game_state::GameState;

mod game_state;
mod grid;

const DESIRED_FPS: u32 = 20;
const GRID_SIZE: (f32, f32) = (80.0, 80.0);
const GRID_CELL_SIZE: (f32, f32) = (10.0, 10.0);
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 * GRID_CELL_SIZE.0,
    GRID_SIZE.1 * GRID_CELL_SIZE.1,
);


fn main() -> GameResult {
    let (ctx, events_loop) = ContextBuilder::new("sand", "someone")
        .window_setup(WindowSetup::default().title("Sand!"))
        .window_mode(WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1)).build()?;


    let state = GameState::new();
    // And finally we actually run our game, passing in our context and state.
    event::run(ctx, events_loop, state)
}


