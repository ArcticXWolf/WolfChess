use chess::{Board, ChessMove};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Game {
    current_position: Board,
    moves: Vec<ChessMove>,
}

impl Game {
    /// Create a new `Game` with the initial position.
    pub fn new() -> Game {
        Game {
            current_position: Board::default(),
            moves: vec![],
        }
    }

    /// Create a new `Game` with a specific starting position.
    pub fn new_with_board(board: Board) -> Game {
        Game {
            current_position: board,
            moves: vec![],
        }
    }

    /// Get all actions made in this game (moves, draw offers, resignations, etc.)
    pub fn moves(&self) -> &Vec<ChessMove> {
        &self.moves
    }

    /// Get position
    pub fn position(&self) -> &Board {
        &self.current_position
    }

    /// Create a new `Game` object from an FEN string.
    pub fn new_from_fen(fen: &str) -> Option<Game> {
        Game::from_str(fen).ok()
    }

    /// Get the current position on the board from the `Game` object.
    pub fn make_move_new(&self, mv: ChessMove) -> Game {
        let copy = self.current_position.make_move_new(mv);

        Game {
            current_position: copy,
            moves: self.moves.clone(),
        }
    }
}

impl FromStr for Game {
    type Err = chess::Error;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        Ok(Game::new_with_board(Board::from_str(fen)?))
    }
}
