use chess::Board;
use engine;
use std::io::{self, BufRead};
use vampirc_uci::{parse_one, UciMessage};

fn main() {
    loop {
        for line in io::stdin().lock().lines() {
            let msg: UciMessage = parse_one(&line.unwrap());

            match msg {
                UciMessage::Quit => {
                    break;
                }
                UciMessage::Unknown(message_str, err) => {
                    println!(
                        "error parsing unknown uci message: {:?} - {}",
                        err, message_str
                    )
                }
                _ => {
                    println!("uci message not yet implemented: {}", msg)
                }
            }
        }
    }
}

fn perft() {
    println!("Running perft:");
    let (count_nodes, nps) = engine::perft_with_nps(Board::default(), 7);
    println!("Count: {}, nps {}", count_nodes, nps);
}
