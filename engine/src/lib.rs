mod eval;
mod search;
mod time_broker;
mod weights;

use chess::{Board, ChessMove, Error, MoveGen};
use std::convert::TryInto;
use std::str::FromStr;
use std::time::Instant;
use time_broker::TimeBroker;
use tokio::sync::mpsc::{error::SendError, UnboundedReceiver, UnboundedSender};
use vampirc_uci::{UciInfoAttribute, UciMessage, UciSearchControl, UciTimeControl};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum EngineCommand {
    /// Set a position as the current game
    SetPosition {
        /// If `true`, it denotes the starting chess position. Generally, if this property is `true`, then the value of
        /// the `fen` property will be `None`.
        startpos: bool,

        /// The [FEN format](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation) representation of a chess
        /// position.
        fen: Option<String>,

        /// A list of moves to apply to the position.
        moves: Vec<ChessMove>,
    },
    /// Start a perft search to the given depth
    Perft {
        /// The perft depth from the current position
        depth: usize,
    },
    /// Start a search for best move from the current position
    Search {
        time_control: Option<UciTimeControl>,
        search_control: Option<UciSearchControl>,
    },
    /// Stop search next possible
    StopSearch,
    /// Print evaluation score for current position
    EvalCurrentPosition,
    /// Print the current board
    ShowBoard,
    /// Return okay as soon as calculation is finished
    IsReady,
}

struct EngineBroker {
    current_game: Board,
    time_broker: TimeBroker,
}

pub async fn broker_loop(
    mut commands: UnboundedReceiver<EngineCommand>,
    mut output: UnboundedSender<UciMessage>,
) {
    println!("info string EngineBroker started");
    let mut broker = EngineBroker::new();

    while let Some(command) = commands.recv().await {
        broker.handle_command(command, &mut output).await;
    }
    println!("info string EngineBroker shutdown");
}

impl EngineBroker {
    fn new() -> EngineBroker {
        EngineBroker {
            current_game: Board::default(),
            time_broker: TimeBroker::new(),
        }
    }

    async fn handle_command(
        &mut self,
        command: EngineCommand,
        output: &UnboundedSender<UciMessage>,
    ) {
        match command {
            EngineCommand::SetPosition {
                startpos,
                fen,
                moves,
            } => {
                self.set_position(startpos, fen, moves).await.unwrap();
                let answer = UciMessage::info_string(format!("Board: {}", self.current_game));
                output.send(answer).unwrap();
            }
            EngineCommand::Perft { depth } => {
                let answer = UciMessage::info_string("Perft started.".to_string());
                output.send(answer).unwrap();
                self.perft_with_nps(depth, output).await.unwrap();
            }
            EngineCommand::EvalCurrentPosition => {
                let answer = UciMessage::info_string(format!(
                    "info cps {}",
                    eval::evaluate_position(&self.current_game)
                ));
                output.send(answer).unwrap();
            }
            EngineCommand::ShowBoard => {
                let answer = UciMessage::info_string(format!("Board: {}", self.current_game));
                output.send(answer).unwrap();
            }
            EngineCommand::Search {
                time_control,
                search_control,
            } => {
                self.search(time_control, search_control, output);
            }
            EngineCommand::StopSearch => {
                self.time_broker.send_stop();
            }
            EngineCommand::IsReady => {
                let answer = UciMessage::ReadyOk;
                output.send(answer).unwrap();
            }
        };
    }

    async fn set_position(
        &mut self,
        startpos: bool,
        fen: Option<String>,
        moves: Vec<ChessMove>,
    ) -> Result<(), Error> {
        if startpos {
            self.current_game = Board::default();
        }

        match fen {
            Some(fen_str) => {
                self.current_game = match Board::from_str(fen_str.as_str()) {
                    Ok(board) => board,
                    Err(e) => return Err(e),
                }
            }
            None => {}
        };

        for mv in moves {
            self.current_game = self.current_game.make_move_new(mv);
        }

        Ok(())
    }

    async fn perft_with_nps(
        &self,
        depth: usize,
        output: &UnboundedSender<UciMessage>,
    ) -> Result<(), SendError<UciMessage>> {
        let time = Instant::now();

        let nodes = perft(self.current_game, depth);

        let nps = (nodes as f64 / time.elapsed().as_secs_f64()) as u32;

        let answer = UciMessage::Info {
            0: vec![
                UciInfoAttribute::Nodes(nodes.try_into().unwrap()),
                UciInfoAttribute::Nps(nps.try_into().unwrap()),
            ],
        };
        output.send(answer).unwrap();

        Ok(())
    }

    fn search(
        &mut self,
        time_control: Option<UciTimeControl>,
        search_control: Option<UciSearchControl>,
        output: &UnboundedSender<UciMessage>,
    ) {
        let board = self.current_game.clone();
        let moved_output = output.clone();
        let mut cancel_receiver = self.time_broker.get_cancel_receiver();
        let max_depth = search_control.map_or(None, |d| d.depth.map(|d| d as usize));

        if let Some(tc) = time_control {
            self.time_broker
                .seed_time_control(board.side_to_move(), &tc);

            match tc {
                UciTimeControl::TimeLeft { .. } | UciTimeControl::MoveTime(..) => {
                    let new_receiver_option = self.time_broker.start_timer();
                    if let Some(new_receiver) = new_receiver_option {
                        cancel_receiver = new_receiver;
                    }
                }
                _ => {}
            }
        }

        tokio::spawn(async move {
            println!("info string SearchTask started");
            let result =
                search::iterative_deepening(&board, max_depth, cancel_receiver, &moved_output);

            let answer = UciMessage::BestMove {
                best_move: *result.pv.first().unwrap(),
                ponder: None,
            };
            moved_output.send(answer).unwrap();
            println!("info string SearchTask shutdown");
        });
    }
}

fn perft(board: Board, depth: usize) -> u32 {
    let movegen = MoveGen::new_legal(&board);

    if depth <= 1 {
        return movegen.len().try_into().unwrap();
    }

    let mut count_nodes_total = 0;
    for mv in movegen {
        let new_board = board.make_move_new(mv);
        let count_nodes_this_move = perft(new_board, depth - 1);
        count_nodes_total += count_nodes_this_move;
    }

    count_nodes_total
}
