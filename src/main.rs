use tetra::graphics::{ self, Color };
use tetra::{ Context, ContextBuilder, State };

struct GameState {}

impl State for GameState {}

fn main() -> tetra::Result {
    ContextBuilder::new("Othello", 1280, 720)
    .quit_on_escape(true)
    .build()?
    .run(|_| Ok(GameState {}))
}
