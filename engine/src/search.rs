use std::collections::HashMap;

use chess::{Board, BoardStatus, ChessMove, MoveGen};

use crate::eval;

pub fn analyze_moves(board: &Board, depth_left: usize) -> HashMap<ChessMove, i32> {
    let mut result = HashMap::new();
    let movegen = MoveGen::new_legal(&board);

    for mv in movegen {
        let new_board = board.make_move_new(mv);
        let (mut new_score, ..) = alphabeta(
            &new_board,
            -eval::MAX_CP_SCORE,
            eval::MAX_CP_SCORE,
            depth_left - 1,
        );
        new_score = -new_score;
        result.insert(mv, new_score);
    }

    result
}

pub fn alphabeta(
    board: &Board,
    mut alpha: i32,
    beta: i32,
    depth_left: usize,
) -> (i32, Vec<ChessMove>, i32) {
    let mut best_score = -eval::MAX_CP_SCORE;
    let mut best_pricipal_variation = Vec::<ChessMove>::new();
    let mut total_leaves_searched = 0;

    match board.status() {
        BoardStatus::Stalemate => {
            return (0, Vec::new(), 1);
        }
        BoardStatus::Checkmate => {
            return (-eval::MAX_CP_SCORE, Vec::new(), 1);
        }
        _ => {}
    }

    if depth_left == 0 {
        let score = eval::evaluate_position(board);
        return (score, Vec::new(), 1);
    }

    let movegen = MoveGen::new_legal(&board);

    for mv in movegen {
        let new_board = board.make_move_new(mv);
        let (mut new_score, new_moves, leaves_searched) =
            alphabeta(&new_board, -beta, -alpha, depth_left - 1);
        new_score = -new_score;
        total_leaves_searched += leaves_searched;

        if new_score > best_score {
            best_pricipal_variation = new_moves;
            best_pricipal_variation.insert(0, mv);
            best_score = new_score;
        }

        if new_score > alpha {
            alpha = new_score;
        }

        if alpha >= beta {
            break;
        }
    }

    (alpha, best_pricipal_variation, total_leaves_searched)
}
