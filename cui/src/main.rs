use engine;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{self, UnboundedSender};
use vampirc_uci::{parse_one, UciMessage};

#[tokio::main]
async fn main() {
    let (broker_command_sender, broker_command_reciever) = mpsc::unbounded_channel();
    let (broker_output_sender, mut broker_output_reciever) = mpsc::unbounded_channel();
    let _broker_handle = tokio::spawn(engine::broker_loop(
        broker_command_reciever,
        broker_output_sender.clone(),
    ));
    tokio::spawn(spawn_uci(
        broker_command_sender,
        broker_output_sender.clone(),
    ));

    tokio::spawn(async move {
        while let Some(msg) = broker_output_reciever.recv().await {
            println!("{}", msg);
        }
    })
    .await
    .unwrap();
}

async fn spawn_uci(
    engine_command_sender: UnboundedSender<engine::EngineCommand>,
    output: UnboundedSender<UciMessage>,
) {
    let reader = BufReader::new(stdin());
    let mut lines = reader.lines();

    while let Ok(line) = lines.next_line().await {
        let msg = parse_one(&line.unwrap());
        match msg {
            UciMessage::Quit => {
                break;
            }
            _ => {
                tokio::spawn(handle_message(
                    msg,
                    engine_command_sender.clone(),
                    output.clone(),
                ));
            }
        }
    }
}

enum HandleMessageError {
    Engine(SendError<engine::EngineCommand>),
    Output(SendError<UciMessage>),
}

async fn handle_message(
    msg: UciMessage,
    mut engine_command_sender: UnboundedSender<engine::EngineCommand>,
    mut output: UnboundedSender<UciMessage>,
) -> Result<(), HandleMessageError> {
    match msg {
        UciMessage::Uci => {
            output
                .send(UciMessage::Id {
                    name: Some("WolfChess".to_string()),
                    author: None,
                })
                .map_err(|e| HandleMessageError::Output(e))?;
            output
                .send(UciMessage::Id {
                    name: None,
                    author: Some("Jan Niklas Richter".to_string()),
                })
                .map_err(|e| HandleMessageError::Output(e))?;
            output
                .send(UciMessage::UciOk)
                .map_err(|e| HandleMessageError::Output(e))?;
        }
        UciMessage::IsReady => {
            let command = engine::EngineCommand::IsReady;
            engine_command_sender
                .send(command)
                .map_err(|e| HandleMessageError::Engine(e))?;
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
            engine_command_sender
                .send(command)
                .map_err(|e| HandleMessageError::Engine(e))?;
        }
        UciMessage::Go {
            time_control,
            search_control,
        } => {
            let command = engine::EngineCommand::Search {
                time_control,
                search_control,
            };
            engine_command_sender
                .send(command)
                .map_err(|e| HandleMessageError::Engine(e))?;
        }
        UciMessage::Unknown(message_str, err) => match message_str.as_str() {
            "perft" => {
                let command = engine::EngineCommand::Perft { depth: 7 };
                engine_command_sender
                    .send(command)
                    .map_err(|e| HandleMessageError::Engine(e))?;
            }
            "eval" => {
                let command = engine::EngineCommand::EvalCurrentPosition;
                engine_command_sender
                    .send(command)
                    .map_err(|e| HandleMessageError::Engine(e))?;
            }
            "show" => {
                let command = engine::EngineCommand::ShowBoard;
                engine_command_sender
                    .send(command)
                    .map_err(|e| HandleMessageError::Engine(e))?;
            }
            _ => {
                output
                    .send(UciMessage::info_string(format!(
                        "Unknown message - {:#?} {}",
                        err, message_str
                    )))
                    .map_err(|e| HandleMessageError::Output(e))?;
            }
        },
        _ => {
            output
                .send(UciMessage::info_string(format!(
                    "Message not yet implemented - {}",
                    msg
                )))
                .map_err(|e| HandleMessageError::Output(e))?;
        }
    }
    Ok(())
}
