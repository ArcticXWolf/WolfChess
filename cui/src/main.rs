use futures::{channel::mpsc, SinkExt};

use async_std::{
    io::{stdin, BufReader},
    prelude::*,
    task,
};
use engine;
use vampirc_uci::{parse_one, UciMessage};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() {
    let (broker_command_sender, broker_command_reciever) = mpsc::unbounded();
    let (broker_output_sender, mut broker_output_reciever) = mpsc::unbounded();
    let _broker_handle = task::spawn(engine::broker_loop(
        broker_command_reciever,
        broker_output_sender.clone(),
    ));
    task::spawn(spawn_uci(
        broker_command_sender,
        broker_output_sender.clone(),
    ));

    task::block_on(async {
        while let Some(msg) = broker_output_reciever.next().await {
            println!("{}", msg);
        }
    });
}

fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()>
where
    F: Future<Output = Result<()>> + Send + 'static,
{
    task::spawn(async move {
        if let Err(e) = fut.await {
            eprintln!("{}", e)
        }
    })
}

async fn spawn_uci(
    engine_command_sender: engine::Sender<engine::EngineCommand>,
    output: engine::Sender<UciMessage>,
) -> Result<()> {
    let reader = BufReader::new(stdin());
    let mut lines = reader.lines();

    while let Some(line) = lines.next().await {
        let msg = parse_one(&line.unwrap());
        match msg {
            UciMessage::Quit => {
                break;
            }
            _ => {
                spawn_and_log_error(handle_message(
                    msg,
                    engine_command_sender.clone(),
                    output.clone(),
                ));
            }
        }
    }
    Ok(())
}

async fn handle_message(
    msg: UciMessage,
    mut engine_command_sender: engine::Sender<engine::EngineCommand>,
    mut output: engine::Sender<UciMessage>,
) -> Result<()> {
    match msg {
        UciMessage::Uci => {
            output
                .send(UciMessage::Id {
                    name: Some("WolfChess".to_string()),
                    author: None,
                })
                .await?;
            output
                .send(UciMessage::Id {
                    name: None,
                    author: Some("Jan Niklas Richter".to_string()),
                })
                .await?;
            output.send(UciMessage::UciOk).await?;
        }
        UciMessage::IsReady => {
            let command = engine::EngineCommand::IsReady;
            engine_command_sender.send(command).await?;
        }
        UciMessage::Position {
            startpos,
            fen,
            moves,
        } => {
            let fen_str = match fen {
                Some(ucifen) => Some(ucifen.to_string()),
                None => None,
            };
            let command = engine::EngineCommand::SetPosition {
                startpos: startpos,
                fen: fen_str,
                moves: moves,
            };
            engine_command_sender.send(command).await?;
        }
        UciMessage::Go {
            time_control,
            search_control,
        } => {
            let command = engine::EngineCommand::Search {
                time_control,
                search_control,
            };
            engine_command_sender.send(command).await?;
        }
        UciMessage::Unknown(message_str, err) => match message_str.as_str() {
            "perft" => {
                let command = engine::EngineCommand::Perft { depth: 7 };
                engine_command_sender.send(command).await?;
            }
            "eval" => {
                let command = engine::EngineCommand::EvalCurrentPosition;
                engine_command_sender.send(command).await?;
            }
            "show" => {
                let command = engine::EngineCommand::ShowBoard;
                engine_command_sender.send(command).await?;
            }
            _ => {
                output
                    .send(UciMessage::info_string(format!(
                        "Unknown message - {:#?} {}",
                        err, message_str
                    )))
                    .await?;
            }
        },
        _ => {
            output
                .send(UciMessage::info_string(format!(
                    "Message not yet implemented - {}",
                    msg
                )))
                .await?;
        }
    }
    Ok(())
}
