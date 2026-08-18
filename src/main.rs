use crate::game::Game;

mod game;
mod engine;

#[macroquad::main("gvh")]
async fn main() {
    let mut g = Game::new();
    g.start().await;
}

