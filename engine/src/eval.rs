use chess::{Board, Color, Piece, ALL_PIECES};

pub const MAX_CP_SCORE: i32 = 1000000;

pub fn evaluate_position(board: &Board) -> i32 {
    evaluate_material_for_color(board, Color::White)
        - evaluate_material_for_color(board, Color::Black)
}

fn evaluate_material_for_color(board: &Board, color: Color) -> i32 {
    let mut material_value: i32 = 0;

    for piece_type in ALL_PIECES {
        let pieces = board.pieces(piece_type) & board.color_combined(color);
        material_value += (pieces.popcnt() as i32) * get_piece_type_material_value(piece_type);
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
