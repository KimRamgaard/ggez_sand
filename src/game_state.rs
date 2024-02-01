use ggez::{Context, event, GameError, graphics};
use ggez::event::MouseButton;
use ggez::graphics::Canvas;
use oorandom::Rand32;

use crate::{DESIRED_FPS, GRID_CELL_SIZE};

pub struct GameState {
    /// Our RNG state
    pub rng: Rand32,
    pub rectangles: Vec<graphics::Rect>,
}

impl GameState {
    pub fn new() -> Self {
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("Could not create RNG seed");
        let rng = Rand32::new(u64::from_ne_bytes(seed));
        // empty vec!
        let rectangles = Vec::new();

        GameState { rng, rectangles }
    }
}

impl event::EventHandler<GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        while _ctx.time.check_update_time(DESIRED_FPS) {
            let mut updated_rectangles = Vec::new();

            for rect in &self.rectangles {
                let new_y = rect.y + GRID_CELL_SIZE.1; // Move down by 1 unit
                let updated_rect = graphics::Rect::new(rect.x, new_y, rect.w, rect.h);
                updated_rectangles.push(updated_rect);
            }

            self.rectangles = updated_rectangles;
        }
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = Canvas::from_frame(_ctx, graphics::Color::WHITE);


        for rect in &self.rectangles {
            canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                .dest_rect(*rect)
                .color(graphics::Color::from_rgb(
                    (self.rng.rand_u32() & 0xFF) as u8,
                    (self.rng.rand_u32() & 0xFF) as u8,
                    (self.rng.rand_u32() & 0xFF) as u8,
                ))); // Example color: blue
        }

        canvas.finish(_ctx)?;

        ggez::timer::yield_now();

        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) -> Result<(), GameError> {// Create a new rectangle at the mouse position with the specified size

        let x_pos = ((_x / GRID_CELL_SIZE.0).floor()) * GRID_CELL_SIZE.0;
        let y_pos = ((_y / GRID_CELL_SIZE.1).floor()) * GRID_CELL_SIZE.1;

        let new_rect = graphics::Rect::new(x_pos, y_pos, GRID_CELL_SIZE.0, GRID_CELL_SIZE.1);

        self.rectangles.push(new_rect);

        println!("Mouse Button Down at x: {}, y: {}, x_pos: {}", _x, _y, x_pos);
        Ok(())
    }
}