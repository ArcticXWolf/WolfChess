use std::io::{self, Write};
use chess::board::Board;

fn main() {
    let mut board = Board::default();

    loop {
        println!("{}", board.console_render());

        let command = get_user_command();

        match command {
            CuiUserCommand::Quit => break,
            CuiUserCommand::ReadFen{fen} => {board = Board::new(fen.as_str()).expect("Error during FEN load")},
            CuiUserCommand::UnknownCommand => println!("Command not known"),
        }
    }
}

enum CuiUserCommand {
    ReadFen{fen: String},
    Quit,
    UnknownCommand,
}

fn get_user_command() -> CuiUserCommand {
    let mut input = String::new();
    print!("Command: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    let parts: Vec<_> = input.split_whitespace().collect();

    if parts.len() <= 0 {
        return CuiUserCommand::UnknownCommand;
    }

    match parts[0].trim() {
        "quit" | "exit" => CuiUserCommand::Quit,
        "fen" => CuiUserCommand::ReadFen{fen: parts[1..].join(" ")},
        _ => CuiUserCommand::UnknownCommand,
    }
}