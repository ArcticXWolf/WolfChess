use std::time::{Duration, Instant};

use chess::{Board, BoardStatus, ChessMove, MoveGen};
use tokio::sync::{mpsc::UnboundedSender, watch::Receiver};
use vampirc_uci::{UciInfoAttribute, UciMessage};

use crate::eval;

pub struct SearchInfo {
    pub score: i32,
    pub pv: Vec<ChessMove>,
    pub nodes: u32,
    pub nps: u32,
    pub depth: usize,
    pub time: Duration,
}

pub fn iterative_deepening(
    board: &Board,
    max_depth: Option<usize>,
    cancel_receiver: Receiver<bool>,
    output: &UnboundedSender<UciMessage>,
) -> SearchInfo {
    let time = Instant::now();

    let mut depth = 0;
    let mut result = SearchInfo {
        score: 0,
        pv: vec![],
        nodes: 0,
        nps: 0,
        depth: 1,
        time: Duration::ZERO,
    };

    loop {
        depth += 1;

        let (score, moves, nodes, cancelled) = alphabeta(
            board,
            -eval::MAX_CP_SCORE,
            eval::MAX_CP_SCORE,
            &cancel_receiver,
            depth,
            0,
        );

        if cancelled || max_depth.map_or_else(|| false, |md| depth > md) {
            break;
        }

        result.score = score;
        result.pv = moves;
        result.nodes += nodes as u32;
        result.depth = depth;

        let answer = UciMessage::Info {
            0: vec![
                UciInfoAttribute::Score {
                    cp: Some(result.score),
                    mate: None,
                    lower_bound: None,
                    upper_bound: None,
                },
                UciInfoAttribute::Pv(result.pv.clone()),
                UciInfoAttribute::Nodes(result.nodes.try_into().unwrap()),
                UciInfoAttribute::Depth(result.depth.try_into().unwrap()),
                UciInfoAttribute::Time(
                    vampirc_uci::Duration::from_std(time.elapsed())
                        .unwrap_or(vampirc_uci::Duration::zero()),
                ),
            ],
        };
        output.send(answer).unwrap();
    }

    result.nps = (result.nodes as f64 / time.elapsed().as_secs_f64()) as u32;
    result.time = time.elapsed();

    let answer = UciMessage::Info {
        0: vec![
            UciInfoAttribute::Score {
                cp: Some(result.score),
                mate: None,
                lower_bound: None,
                upper_bound: None,
            },
            UciInfoAttribute::Pv(result.pv.clone()),
            UciInfoAttribute::Nodes(result.nodes.try_into().unwrap()),
            UciInfoAttribute::Nps(result.nps.try_into().unwrap()),
            UciInfoAttribute::Depth(result.depth.try_into().unwrap()),
            UciInfoAttribute::Time(
                vampirc_uci::Duration::from_std(result.time)
                    .unwrap_or(vampirc_uci::Duration::zero()),
            ),
        ],
    };
    output.send(answer).unwrap();

    result
}

pub fn alphabeta(
    board: &Board,
    mut alpha: i32,
    beta: i32,
    cancel_receiver: &Receiver<bool>,
    depth_left: usize,
    ply: usize,
) -> (i32, Vec<ChessMove>, i32, bool) {
    let mut best_score = -eval::MAX_CP_SCORE;
    let mut best_pricipal_variation = Vec::<ChessMove>::new();
    let mut total_leaves_searched = 0;

    match board.status() {
        BoardStatus::Stalemate => {
            return (0, Vec::new(), 1, false);
        }
        BoardStatus::Checkmate => {
            return (-eval::MAX_CP_SCORE + ply as i32, Vec::new(), 1, false);
        }
        _ => {}
    }

    if depth_left == 0 {
        let score = eval::evaluate_position(board);
        return (score, Vec::new(), 1, false);
    }

    let movegen = MoveGen::new_legal(&board);
    let mut cancelled = false;

    for mv in movegen {
        if cancelled || (cancel_receiver.has_changed().unwrap_or(true) && *cancel_receiver.borrow())
        {
            return (0, Vec::new(), 1, true);
        }

        let new_board = board.make_move_new(mv);
        let (mut new_score, new_moves, leaves_searched, new_cancelled) = alphabeta(
            &new_board,
            -beta,
            -alpha,
            cancel_receiver,
            depth_left - 1,
            ply + 1,
        );
        new_score = -new_score;
        cancelled = cancelled || new_cancelled;
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

    (
        alpha,
        best_pricipal_variation,
        total_leaves_searched,
        cancelled,
    )
}
