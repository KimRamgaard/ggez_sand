use ggez::{Context, event, GameError, graphics};
use ggez::event::MouseButton;
use ggez::graphics::Canvas;
use oorandom::Rand32;
use crate::grid::GridPosition;
use crate::GRID_SIZE;

pub struct GameState {
    /// Our RNG state
    pub rng: Rand32,
    //pub rectangles : Vec<graphics::Rect>,
}

impl GameState {
    pub fn new() -> Self {
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("Could not create RNG seed");
        let rng = Rand32::new(u64::from_ne_bytes(seed));

        //let square_pos = GridPosition::get_random_position(&mut rng, GRID_SIZE.0, GRID_SIZE.1);
        GameState { rng }
    }
}

impl event::EventHandler<GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    // fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) -> Result<(), GameError> {
    //
    //     // Define the rectangle size and position
    //     let new_rect = graphics::Rect::new(100.0, 100.0, 50.0, 50.0); // Example position and size
    //     self.rectangles.push(new_rect);
    //
    //     Ok(())
    // }
    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = Canvas::from_frame(_ctx, graphics::Color::WHITE);

        // Draw each rectangle in `self.rectangles`
        /*
        for rect in &self.rectangles {
            canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                .dest_rect(*rect)
                .color(graphics::Color::from_rgb(0, 0, 255))); // Example color: blue
        }
        */



        for x in 0..GRID_SIZE.0 {
            for y in 0..GRID_SIZE.1 {
                let color = graphics::Color::from_rgb(
                    (self.rng.rand_u32() & 0xFF) as u8,
                    (self.rng.rand_u32() & 0xFF) as u8,
                    (self.rng.rand_u32() & 0xFF) as u8,
                );
                // Convert GridPosition to a rectangle for drawing
                let rect: graphics::Rect = GridPosition::new(x, y).into();

                // Use the rectangle to draw the quad
                canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                    .dest_rect(rect)
                    .color(color));
            }
        }


        canvas.finish(_ctx)?;

        Ok(())
    }
}