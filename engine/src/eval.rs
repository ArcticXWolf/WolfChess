use chess::{Board, Color, MoveGen, Piece, Square, ALL_PIECES};

use crate::weights;

pub const MAX_CP_SCORE: i32 = 1000000;

pub fn evaluate_position(board: &Board) -> i32 {
    let result_white = evaluate_material_for_color(board, Color::White)
        + evaluate_modifiers_for_color(board, Color::White);
    let result_black = evaluate_material_for_color(board, Color::Black)
        + evaluate_modifiers_for_color(board, Color::Black);

    match board.side_to_move() {
        Color::White => result_white - result_black,
        Color::Black => result_black - result_white,
    }
}

fn evaluate_material_for_color(board: &Board, color: Color) -> i32 {
    let mut material_value: i32 = 0;
    let gamephase = get_gamephase(board);

    for piece_type in ALL_PIECES {
        let pieces = board.pieces(piece_type) & board.color_combined(color);
        material_value += (pieces.popcnt() as i32) * get_piece_type_material_value(piece_type);
        material_value += get_pst_value_for_piece_type(color, piece_type, pieces, gamephase);
    }

    material_value
}

fn evaluate_modifiers_for_color(board: &Board, color: Color) -> i32 {
    return evaluate_pair_modifier_for_color(board, color)
        + evaluate_mobility_modifier_for_color(board, color)
        + evaluate_tempo_modifier_for_color(board, color);
}

fn evaluate_pair_modifier_for_color(board: &Board, color: Color) -> i32 {
    let mut score = 0;

    if (board.pieces(Piece::Bishop) & board.color_combined(color)).popcnt() >= 2 {
        score += weights::PAIR_MOD_BISHOP;
    }
    if (board.pieces(Piece::Knight) & board.color_combined(color)).popcnt() >= 2 {
        score += weights::PAIR_MOD_KNIGHT;
    }
    if (board.pieces(Piece::Rook) & board.color_combined(color)).popcnt() >= 2 {
        score += weights::PAIR_MOD_ROOK;
    }

    return score;
}

fn evaluate_mobility_modifier_for_color(board: &Board, color: Color) -> i32 {
    let adj_board = if board.side_to_move() == color {
        Some(*board)
    } else {
        board.null_move()
    };

    let mobility = adj_board.map_or(0, |b| MoveGen::new_legal(&b).len());
    return mobility as i32 * weights::MOBILITY_MOD;
}

fn evaluate_tempo_modifier_for_color(board: &Board, color: Color) -> i32 {
    return if board.side_to_move() == color {
        weights::TEMPO_MOD
    } else {
        0
    };
}

fn get_gamephase(board: &Board) -> i32 {
    (board.pieces(Piece::Knight).popcnt()
        + board.pieces(Piece::Bishop).popcnt()
        + 2 * board.pieces(Piece::Rook).popcnt()
        + 4 * board.pieces(Piece::Queen).popcnt()) as i32
}

fn get_piece_type_material_value(piece_type: Piece) -> i32 {
    match piece_type {
        Piece::King => weights::KING_MV,
        Piece::Queen => weights::QUEEN_MV,
        Piece::Rook => weights::ROOK_MV,
        Piece::Bishop => weights::BISHOP_MV,
        Piece::Knight => weights::KNIGHT_MV,
        Piece::Pawn => weights::PAWN_MV,
    }
}

fn get_pst_value_for_piece_type(
    color: Color,
    piece_type: Piece,
    pieces: chess::BitBoard,
    gamephase: i32,
) -> i32 {
    let mut material_value = 0;
    for square in pieces {
        material_value += get_pst_value_for_square(color, piece_type, square, gamephase);
    }
    material_value
}

fn get_pst_value_for_square(
    color: Color,
    piece_type: Piece,
    square: Square,
    gamephase: i32,
) -> i32 {
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
        Piece::King => {
            (gamephase * *weights::KING_PST.get(square_adj).unwrap_or(&0)
                + (24 - gamephase) * *weights::KING_PST_EG.get(square_adj).unwrap_or(&0))
                / 24
        }
    }
}
