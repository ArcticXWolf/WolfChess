use chess::{Board, Color, Piece, Square, ALL_PIECES};

use crate::weights;

pub const MAX_CP_SCORE: i32 = 1000000;

pub fn evaluate_position(board: &Board) -> i32 {
    let result = evaluate_material_for_color(board, Color::White)
        - evaluate_material_for_color(board, Color::Black);

    match board.side_to_move() {
        Color::White => result,
        Color::Black => -result,
    }
}

fn evaluate_material_for_color(board: &Board, color: Color) -> i32 {
    let mut material_value: i32 = 0;

    for piece_type in ALL_PIECES {
        let pieces = board.pieces(piece_type) & board.color_combined(color);
        material_value += (pieces.popcnt() as i32) * get_piece_type_material_value(piece_type);
        material_value += get_pst_value_for_piece_type(color, piece_type, pieces);
    }

    material_value
}

fn get_piece_type_material_value(piece_type: Piece) -> i32 {
    match piece_type {
        Piece::King => 0,
        Piece::Queen => 900,
        Piece::Rook => 500,
        Piece::Bishop => 330,
        Piece::Knight => 320,
        Piece::Pawn => 100,
    }
}

fn get_pst_value_for_piece_type(color: Color, piece_type: Piece, pieces: chess::BitBoard) -> i32 {
    let mut material_value = 0;
    for square in pieces {
        material_value += get_pst_value_for_square(color, piece_type, square);
    }
    material_value
}

fn get_pst_value_for_square(color: Color, piece_type: Piece, square: Square) -> i32 {
    let square_adj = match color {
        Color::White => square.to_index(),
        Color::Black => square.to_index() ^ 56,
    };
    match piece_type {
        Piece::Pawn => *weights::PAWN_PST.get(square_adj).unwrap_or(&0),
        Piece::Knight => *weights::KNIGHT_PST.get(square_adj).unwrap_or(&0),
        Piece::Bishop => *weights::BISHOP_PST.get(square_adj).unwrap_or(&0),
        Piece::Rook => *weights::ROOK_PST.get(square_adj).unwrap_or(&0),
        Piece::Queen => *weights::QUEEN_PST.get(square_adj).unwrap_or(&0),
        Piece::King => *weights::KING_PST.get(square_adj).unwrap_or(&0),
    }
}
