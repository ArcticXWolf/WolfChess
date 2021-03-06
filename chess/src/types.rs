use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self {
            piece_type,
            color
        }
    }

    pub fn from_symbol(character: char) -> Option<Self> {
        match character {
            'P' => Some(Piece::new(PieceType::Pawn, Color::White)),
            'R' => Some(Piece::new(PieceType::Rook, Color::White)),
            'N' => Some(Piece::new(PieceType::Knight, Color::White)),
            'B' => Some(Piece::new(PieceType::Bishop, Color::White)),
            'Q' => Some(Piece::new(PieceType::Queen, Color::White)),
            'K' => Some(Piece::new(PieceType::King, Color::White)),
            'p' => Some(Piece::new(PieceType::Pawn, Color::Black)),
            'r' => Some(Piece::new(PieceType::Rook, Color::Black)),
            'n' => Some(Piece::new(PieceType::Knight, Color::Black)),
            'b' => Some(Piece::new(PieceType::Bishop, Color::Black)),
            'q' => Some(Piece::new(PieceType::Queen, Color::Black)),
            'k' => Some(Piece::new(PieceType::King, Color::Black)),
            _ => None,
        }
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl From<Piece> for char {
    #[inline]
    fn from(piece: Piece) -> char {
        match (piece.color(), piece.piece_type()) {
            (Color::White, PieceType::Pawn) => 'P',
            (Color::White, PieceType::Rook) => 'R',
            (Color::White, PieceType::Knight) => 'N',
            (Color::White, PieceType::Bishop) => 'B',
            (Color::White, PieceType::Queen) => 'Q',
            (Color::White, PieceType::King) => 'K',
            (Color::Black, PieceType::Pawn) => 'p',
            (Color::Black, PieceType::Rook) => 'r',
            (Color::Black, PieceType::Knight) => 'n',
            (Color::Black, PieceType::Bishop) => 'b',
            (Color::Black, PieceType::Queen) => 'q',
            (Color::Black, PieceType::King) => 'k',
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Square {
    A1 =  0, B1 =  1, C1 =  2, D1 =  3, E1 =  4, F1 =  5, G1 =  6, H1 =  7,
    A2 =  8, B2 =  9, C2 = 10, D2 = 11, E2 = 12, F2 = 13, G2 = 14, H2 = 15,
    A3 = 16, B3 = 17, C3 = 18, D3 = 19, E3 = 20, F3 = 21, G3 = 22, H3 = 23,
    A4 = 24, B4 = 25, C4 = 26, D4 = 27, E4 = 28, F4 = 29, G4 = 30, H4 = 31,
    A5 = 32, B5 = 33, C5 = 34, D5 = 35, E5 = 36, F5 = 37, G5 = 38, H5 = 39,
    A6 = 40, B6 = 41, C6 = 42, D6 = 43, E6 = 44, F6 = 45, G6 = 46, H6 = 47,
    A7 = 48, B7 = 49, C7 = 50, D7 = 51, E7 = 52, F7 = 53, G7 = 54, H7 = 55,
    A8 = 56, B8 = 57, C8 = 58, D8 = 59, E8 = 60, F8 = 61, G8 = 62, H8 = 63,
}

impl Square {
    #[inline]
    pub const fn file(self) -> u8 {
        (self as u8) & 0x7
    }

    #[inline]
    pub const fn rank(self) -> u8 {
        (self as u8).wrapping_shr(3)
    }

    #[inline]
    pub const fn background_color(self) -> Color {
        match (self.file() + self.rank()) % 2 == 0 {
            true => Color::Black,
            false => Color::White,
        }
    }
}

impl From<(u8, u8)> for Square {
    /// Creates a square from a pair of coordinates, each in 0..8.
    #[inline]
    fn from(file_rank: (u8, u8)) -> Square {
        match FromPrimitive::from_u8(file_rank.0 + 8*file_rank.1) {
            None => panic!("Called square position out of bounds"),
            Some(sq) => sq
        }
    }
}

impl From<Square> for usize {
    /// Use the square as an index.
    #[inline]
    fn from(square: Square) -> usize {
        square as usize
    }
}