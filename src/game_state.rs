use ggez::{Context, event, GameError, graphics};
use ggez::graphics::Canvas;
use oorandom::Rand32;
use crate::grid::GridPosition;
use crate::GRID_SIZE;

pub struct GameState {
    /// Our RNG state
    pub rng: Rand32,
    pub square_pos: GridPosition,
}

impl GameState {
    pub fn new() -> Self {
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("Could not create RNG seed");
        let mut rng = Rand32::new(u64::from_ne_bytes(seed));

        let square_pos = GridPosition::get_random_position(&mut rng, GRID_SIZE.0, GRID_SIZE.1);
        GameState { rng, square_pos }
    }
}

impl event::EventHandler<GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = Canvas::from_frame(_ctx, graphics::Color::BLUE);

        // Draw the square at its current position
        self.square_pos;

        canvas.draw(&graphics::Quad, graphics::DrawParam::new()
            .dest_rect(self.square_pos.into())
            .color(graphics::Color::WHITE));

        canvas.finish(_ctx)?;

        Ok(())
    }
}