use std::str::FromStr;
use std::char::from_digit;
use std::{error::Error, fmt};

use crate::types::{Color, PieceType, Square};

#[derive(Debug)]
pub enum FenError {
    UnknownPartCount,
    UnknownRankCount,
    UnknownPieceSymbol,
    TooManyFilesForRank,
}

impl Error for FenError {}

impl fmt::Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error during FEN parsing")
    }
}


pub struct Board {
    mailbox: [Option<(PieceType, Color)>; 64],
}

impl Board {
    pub fn new(fen: &str) -> Result<Self, FenError> {
        Board::from_str(fen)
    }

    pub fn empty() -> Board {
        Board {
            mailbox: [None; 64]
        }
    }

    #[inline]
    pub fn place_piece(&mut self, color: Color, piece: PieceType, square: Square) {
        self.mailbox[square as usize] = Some((piece, color));

    }

    #[inline]
    pub fn remove_piece(&mut self, square: Square) -> (PieceType, Color) {
        let (piece, color) = self.mailbox[square as usize].unwrap();
        self.mailbox[square as usize] = None;

        (piece, color)
    }

    #[inline]
    pub fn get_piece(&self, square: &Square) -> Option<(PieceType, Color)> {
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
                    Some((piece, color)) => ICONS[color as usize][piece as usize],
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

                if let Some((piece, color)) = PieceType::from_symbol(character) {
                    let square = Square::from((file as u8, (7 - rank) as u8));
                    board.place_piece(color, piece, square);
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

        return Ok(board);
    }
}