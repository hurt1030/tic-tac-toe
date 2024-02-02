//use std::io::stdin;

enum GameState {
    Incomplete,
    Tie,
    Win,
}

enum Player {
    X,
    O,
}

impl Player {
    fn value(&self) -> i8 {
        match *self {
            Player::X => 1,
            Player::O => 2,
        }
    }
}

pub fn run() {
    println!("Welcome to Tic Tac Toe.");

    let player1 = Player::X;
    let player2 = Player::O;

    println!("player 1 value: {}", player1.value());
    let mut board: [[i8; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    let mut state = GameState::Incomplete;
    let mut isFinished = false;

    while !isFinished {
        draw_board(&board);
        isFinished = true;
    }
}

fn draw_board(board: &[[i8; 3]; 3]) {
    println!(" {} | {} | {} ", board[0][0], board[1][0], board[2][0]);
    println!("---|---|---");
    println!(" {} | {} | {} ", board[0][1], board[1][1], board[2][1]);
    println!("---|---|---");
    println!(" {} | {} | {} ", board[0][2], board[1][2], board[2][2]);
}
