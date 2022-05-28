use std::io;
use chess::board::Board;
use chess::types::{Color, PieceType, Square};

fn main() {
    let mut board = Board::default();

    create_default_piece_placement(&mut board);

    loop {
        println!("{}", board.console_render());

        let command = get_user_command();

        match command {
            CuiUserCommand::Quit => break,
            CuiUserCommand::ReadFen{fen} => println!("Loading fen {}", fen),
            CuiUserCommand::UnknownCommand => println!("Command not known"),
        }
    }
}

fn create_default_piece_placement(board: &mut Board) {
    board.place_piece(Color::White, PieceType::Pawn, Square::A2);
    board.place_piece(Color::White, PieceType::Pawn, Square::B2);
    board.place_piece(Color::White, PieceType::Pawn, Square::C2);
    board.place_piece(Color::White, PieceType::Pawn, Square::D2);
    board.place_piece(Color::White, PieceType::Pawn, Square::E2);
    board.place_piece(Color::White, PieceType::Pawn, Square::F2);
    board.place_piece(Color::White, PieceType::Pawn, Square::G2);
    board.place_piece(Color::White, PieceType::Pawn, Square::H2);

    board.place_piece(Color::White, PieceType::Rook,   Square::A1);
    board.place_piece(Color::White, PieceType::Knight, Square::B1);
    board.place_piece(Color::White, PieceType::Bishop, Square::C1);
    board.place_piece(Color::White, PieceType::Queen,  Square::D1);
    board.place_piece(Color::White, PieceType::King,   Square::E1);
    board.place_piece(Color::White, PieceType::Bishop, Square::F1);
    board.place_piece(Color::White, PieceType::Knight, Square::G1);
    board.place_piece(Color::White, PieceType::Rook,   Square::H1);

    board.place_piece(Color::Black, PieceType::Pawn, Square::A7);
    board.place_piece(Color::Black, PieceType::Pawn, Square::B7);
    board.place_piece(Color::Black, PieceType::Pawn, Square::C7);
    board.place_piece(Color::Black, PieceType::Pawn, Square::D7);
    board.place_piece(Color::Black, PieceType::Pawn, Square::E7);
    board.place_piece(Color::Black, PieceType::Pawn, Square::F7);
    board.place_piece(Color::Black, PieceType::Pawn, Square::G7);
    board.place_piece(Color::Black, PieceType::Pawn, Square::H7);

    board.place_piece(Color::Black, PieceType::Rook,   Square::A8);
    board.place_piece(Color::Black, PieceType::Knight, Square::B8);
    board.place_piece(Color::Black, PieceType::Bishop, Square::C8);
    board.place_piece(Color::Black, PieceType::Queen,  Square::D8);
    board.place_piece(Color::Black, PieceType::King,   Square::E8);
    board.place_piece(Color::Black, PieceType::Bishop, Square::F8);
    board.place_piece(Color::Black, PieceType::Knight, Square::G8);
    board.place_piece(Color::Black, PieceType::Rook,   Square::H8);
}

enum CuiUserCommand {
    ReadFen{fen: String},
    Quit,
    UnknownCommand,
}

fn get_user_command() -> CuiUserCommand {
    let mut input = String::new();
    print!("Command: ");
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    let parts: Vec<_> = input.split_whitespace().collect();

    if parts.len() <= 0 {
        return CuiUserCommand::UnknownCommand;
    }

    match parts[0].trim() {
        "quit" => CuiUserCommand::Quit,
        "fen" => CuiUserCommand::ReadFen{fen: parts[1..].join(" ")},
        _ => CuiUserCommand::UnknownCommand,
    }
}