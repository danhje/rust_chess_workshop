use crate::task_0::board::Board;

struct Game {
    board: Board,
}

impl Game {
    fn new() -> Self {
        Game { board: Board::new() }
    }

    fn play(&mut self) {
        self.board.print();
        println!("Kvit sin tur");
        println!("Det finnes inga brikker på brettet!");
    }
}

pub fn main() {
    let mut game = Game::new();
    game.play();
}