use macroquad::prelude::*;

// Re-export colors for brevity and finer control down the line
pub use macroquad::color::colors;

mod environment;
use environment::{world::World, element::*};

const WORLD_WIDTH: i32 = 400;
const WORLD_HEIGHT: i32 = 300;

#[macroquad::main("BasicShapes")]
async fn main() {

    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await;
    }
}

fn init() -> World {
    let mut world: World = World::new(WORLD_WIDTH, WORLD_HEIGHT);
}
