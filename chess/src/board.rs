use std::str::FromStr;
use std::char::from_digit;
use std::{error::Error, fmt};
use std::sync::Arc;

use crate::types::{Color, Piece, Square};

#[derive(Debug)]
pub enum FenError {
    UnknownPartCount,
    UnknownRankCount,
    UnknownPieceSymbol,
    TooManyFilesForRank,
    UnknownTurnSymbol,
    UnknownCastlingSymbol,
    UnknownEnPassantSquareSymbol,
    NoNumberForHalfMoveClock,
    NoNumberForFullMoveNumber,
}

impl Error for FenError {}

impl fmt::Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error during FEN parsing")
    }
}

#[derive(Clone)]
pub struct BoardState {
    castling_rights: String, // TODO: just as string for now
    en_passent_square: Option<Square>,
    previous_state: Option<Arc<BoardState>>,
}

impl BoardState {
    pub fn empty() -> Self {
        Self {
            castling_rights: String::from("KQkq"),
            en_passent_square: None,
            previous_state: None
        }
    }
}

pub struct Board {
    turn: Color,
    half_move_clock: u16,
    full_move_number: u16,

    mailbox: [Option<Piece>; 64],

    state: Arc<BoardState>,
}

impl Board {
    pub fn new(fen: &str) -> Result<Self, FenError> {
        Self::from_str(fen)
    }

    pub fn empty() -> Self {
        Self {
            turn: Color::White,
            half_move_clock: 0,
            full_move_number: 1,
            mailbox: [None; 64],
            state: Arc::new(BoardState::empty()),
        }
    }

    #[inline]
    pub fn place_piece(&mut self, piece: Piece, square: Square) {
        self.mailbox[square as usize] = Some(piece);

    }

    #[inline]
    pub fn remove_piece(&mut self, square: Square) -> Piece {
        let piece = self.mailbox[square as usize].unwrap();
        self.mailbox[square as usize] = None;

        piece
    }

    #[inline]
    pub fn get_piece(&self, square: &Square) -> Option<Piece> {
        self.mailbox[*square as usize]
    }

    pub fn console_render(&self) -> String {
        const RESET: &str = "\x1b[0m";
        const BACKGROUND_DARK: &str = "\x1b[48;5;236m";
        const BACKGROUND_LIGHT: &str = "\x1b[48;5;239m";
        const ICONS: [[&str; 6]; 2] = [
            ["\x1b[38;5;231m♙", "\x1b[38;5;231m♘", "\x1b[38;5;231m♗", "\x1b[38;5;231m♖", "\x1b[38;5;231m♕", "\x1b[38;5;231m♔"],
            ["\x1b[38;5;232m♙", "\x1b[38;5;232m♘", "\x1b[38;5;232m♗", "\x1b[38;5;232m♖", "\x1b[38;5;232m♕", "\x1b[38;5;232m♔"],
        ];

        let mut output = String::new();

        output.push_str("  a  b  c  d  e  f  g  h\n");
        for rank in (0..8).rev() {
            output.push(from_digit(rank+1, 10).unwrap());

            for file in 0..8 {
                let square = Square::from((file, rank as u8));
                let piece_icon: &str = match self.get_piece(&square) {
                    Some(piece) => ICONS[piece.color() as usize][piece.piece_type() as usize],
                    None => " ",
                };
                let background = match square.background_color() {
                    Color::Black => BACKGROUND_DARK,
                    Color::White => BACKGROUND_LIGHT,
                };

                output.push_str(background);
                output.push(' ');
                output.push_str(piece_icon);
                output.push(' ');
                output.push_str(RESET);
            }
            output.push('\n');
        }

        output
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        for rank in (0..8).rev() {
            let mut adjacent_empty_squares = 0;

            for file in 0..8 {
                let square = Square::from((file, rank as u8));
                match self.get_piece(&square) {
                    Some(piece) => {
                        if adjacent_empty_squares > 0 {
                            fen.push(from_digit(adjacent_empty_squares, 10).unwrap());
                            adjacent_empty_squares = 0;
                        }
                        fen.push(char::from(piece));
                    },
                    None => {adjacent_empty_squares += 1;},
                };
            }

            if adjacent_empty_squares > 0 {
                fen.push(from_digit(adjacent_empty_squares, 10).unwrap());
            }

            if rank > 0 {
                fen.push('/');
            }
        }

        fen.push(' ');
        fen.push(match self.turn {
            Color::White => 'w',
            Color::Black => 'b',
        });

        fen.push(' ');
        fen.push_str(self.state.castling_rights.as_str());

        fen.push(' ');
        fen.push_str(match self.state.en_passent_square {
            Some(_) => "x",
            None => "-",
        });

        fen.push(' ');
        fen.push_str(&self.half_move_clock.to_string());

        fen.push(' ');
        fen.push_str(&self.full_move_number.to_string());

        fen
    }
}

impl Default for Board {
    fn default() -> Board {
        Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").expect("Error loading default board position")
    }
}

impl FromStr for Board {
    type Err = FenError;
    fn from_str(s: &str) -> Result<Board, FenError> {
        let parts: Vec<_> = s.split_whitespace().collect();
        if parts.len() != 6 {
            return Err(FenError::UnknownPartCount);
        }

        let mut board = Board::empty();
        let ranks: Vec<_> = parts[0].split('/').collect();
        if ranks.len() != 8 {
            return Err(FenError::UnknownRankCount);
        }

        for (rank, piecelist) in ranks.iter().enumerate() {
            let mut file = 0;
            for character in piecelist.chars() {
                if file > 8 {
                    return Err(FenError::TooManyFilesForRank);
                }

                if let Some(piece) = Piece::from_symbol(character) {
                    let square = Square::from((file as u8, (7 - rank) as u8));
                    board.place_piece(piece, square);
                    file += 1;
                    continue;
                }

                match character {
                    '1'..='8' => {
                        file += character.to_digit(10).unwrap();
                        continue;
                    },
                    _ => return Err(FenError::UnknownPieceSymbol),
                }
            }
        }

        board.turn = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => { return Err(FenError::UnknownTurnSymbol) }
        };

        let castling_rights = match parts[2] {
            "KQkq" => "KQkq".to_string(),
            _ => { return Err(FenError::UnknownCastlingSymbol) }
        };

        let en_passent_square = match parts[3] {
            "-" => None,
            _ => { return Err(FenError::UnknownEnPassantSquareSymbol) }
        };

        let new_state = {
            let mut state: BoardState = BoardState::empty();
            state.castling_rights = castling_rights;
            state.en_passent_square = en_passent_square;
            state
        };
        board.state = Arc::new(new_state);

        board.half_move_clock = match parts[4].parse() {
            Ok(x) => x,
            Err(_) => {return Err(FenError::NoNumberForHalfMoveClock)},
        };

        board.full_move_number = match parts[5].parse() {
            Ok(x) => x,
            Err(_) => {return Err(FenError::NoNumberForFullMoveNumber)},
        };

        return Ok(board);
    }
}