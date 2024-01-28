use ggez::{ContextBuilder, event, GameResult};
use ggez::conf::{WindowMode, WindowSetup};

use crate::game_state::GameState;

mod game_state;
mod grid;

const GRID_SIZE: (i16, i16) = (30, 20);
const GRID_CELL_SIZE: (i16, i16) = (32, 32);
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);


fn main() -> GameResult {
    let (ctx, events_loop) = ContextBuilder::new("sand", "someone")
        .window_setup(WindowSetup::default().title("Sand!"))
        .window_mode(WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1)).build()?;


    let state = GameState::new();
    // And finally we actually run our game, passing in our context and state.
    event::run(ctx, events_loop, state)
}


