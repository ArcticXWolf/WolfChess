use chess::{Board, MoveGen};
use std::time::Instant;

fn perft(board: Board, depth: i32) -> usize {
    let movegen = MoveGen::new_legal(&board);

    if depth <= 1 {
        return movegen.len();
    }

    let mut count_nodes_total = 0;
    for mv in movegen {
        let new_board = board.make_move_new(mv);
        let count_nodes_this_move = perft(new_board, depth - 1);
        count_nodes_total += count_nodes_this_move;
    }

    count_nodes_total
}

pub fn perft_with_nps(board: Board, depth: i32) -> (usize, usize) {
    let time = Instant::now();

    let nodes = perft(board, depth);

    let nps = (nodes as f64 / time.elapsed().as_secs_f64()) as usize;

    (nodes, nps)
}
