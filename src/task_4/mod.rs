use crate::color::Color;
use crate::task_4::piece::Piece;

mod piece;
mod game;
mod board;

/// # Oppgave 4
/// 
/// I denne oppgaven skal vi implementere trekkene til kongen.
pub fn main() {
    println!("Køyrer game::main() i task_4");
    game::main()
}
