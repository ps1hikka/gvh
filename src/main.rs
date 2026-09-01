use crate::game::Game;

mod engine;
mod game;

#[macroquad::main("gvh")]
async fn main() {
    let mut g = Game::new().await;
    g.start().await;
}
