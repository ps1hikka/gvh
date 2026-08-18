use crate::game::Game;

pub mod game;

#[macroquad::main("gvh")]
async fn main() {
    let g = Game::new();
    g.start().await;
}

