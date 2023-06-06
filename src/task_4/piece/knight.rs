use std::collections::HashSet;

use crate::finished_game::color::Color;
use crate::finished_game::piece::Piece;
use crate::square::{Square, Squares};

const KNIGHT_NAME: &str = "springar";

#[derive(Clone)]
pub struct Knight {
    position: (u8, u8),
    color: Color,
}

impl Piece for Knight {
    fn new(color: Color, position: (u8, u8)) -> Self {
        Knight {
            color,
            position,
        }
    }

    fn print(&self) -> char {
        match self.color {
            Color::White => '♞',
            Color::Black => '♘',
        }
    }

    fn get_name(&self) -> String {
        String::from(KNIGHT_NAME)
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_position(&self) -> &(u8, u8) {
        &self.position
    }

    fn move_piece(&mut self, target: (u8, u8)) {
        self.position = target;
    }
    
    /// Returnerer et HashSet som inneholder gyldige posisjoner springeren kan flytte til. En posisjon
    /// defineres av et to-tuppel med koordinater, der f.eks (0, 1) korresponderer til feltet A2.
    /// `square.rs` inneholder hjelpefunksjoner for å konvertere f.eks `"a2"` til `(0, 1)` og omvendt.
    ///
    /// # Argumenter
    /// - `team` Referanse til et HashSet som inneholder dine brikkers posisjoner.
    /// - `rival_team` Referanse til et HashSet som inneholder posisjonene til motstanderens brikker.
    ///
    fn get_moves(&self, team: &HashSet<(u8, u8)>, _rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        // Du kan gjerne bruke din egen implementasjon fra oppgave 3 her
        let (x, y) = self.position.as_i8().unwrap();
        let moves: HashSet<(i8, i8)> = HashSet::from_iter([
                            (x - 1, y + 2), (x + 1, y + 2),
            (x - 2, y + 1),                                 (x + 2, y + 1),

            (x - 2, y - 1),                                 (x + 2, y - 1),
                            (x - 1, y - 2), (x + 1, y - 2),
        ]);
        moves.as_board_positions().difference(team).cloned().collect()
    }
}