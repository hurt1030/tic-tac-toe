use std::io::stdin;

pub fn run() {
    println!("Welcome to Tic Tac Toe.");

    let mut board: [[i8; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    let mut is_win = false;
    let mut curr_player: i8 = 1;
    draw_board(&board);

    loop {
        println!("Player {}, it is your turn.", get_display_value(curr_player));

        let mut player_move: (u8, u8);
        loop {
            player_move = get_player_move();

            if move_is_valid(&board, player_move) {
                break;
            }
            println!("Please select an open space on the board.");
        }

        try_move(&mut board, player_move, curr_player);

        draw_board(&board);

        if check_win(&board, curr_player) {
            is_win = true;
            break;
        }

        if is_board_full(&board) {
            break;
        }

        if curr_player == 1 {
            curr_player = 2;
        } else {
            curr_player = 1;
        }
    }

    println!("Game is complete!");

    if is_win {
        println!("Player {} wins!", get_display_value(curr_player));
    } else  {
        println!("It is a tie!");
    }
}

fn draw_board(board: &[[i8; 3]; 3]) {
    println!(" {} | {} | {} ", get_display_value(board[0][0]), get_display_value(board[1][0]), get_display_value(board[2][0]));
    println!("---|---|---");
    println!(" {} | {} | {} ", get_display_value(board[0][1]), get_display_value(board[1][1]), get_display_value(board[2][1]));
    println!("---|---|---");
    println!(" {} | {} | {} ", get_display_value(board[0][2]), get_display_value(board[1][2]), get_display_value(board[2][2]));
}

fn get_display_value(val: i8) -> &'static str {
    match val {
        1 => return "X",
        2 => return "O",
        _ => return " "
    }
}

fn get_player_move() -> (u8, u8) {
    println!("Enter your move as a comma separated pair of 0-based indices");
    
    let mut x_res: Result<u8, _>;
    let mut y_res: Result<u8, _>;
    let mut x: u8;
    let mut y: u8;

    loop {
        let mut player_move: String = String::new();
        stdin().read_line(&mut player_move).unwrap();

        let decoded_move: Vec<&str> = player_move.trim().split(",").collect();

        if decoded_move.len() != 2 {
            println!("Please enter a valid move.");
            continue;
        }

        x_res = decoded_move[0].parse();
        y_res = decoded_move[1].parse();

        if x_res.is_err() || y_res.is_err() {
            println!("Please enter a valid move.");
        } else {
            x = x_res.unwrap();
            y = y_res.unwrap();
            println!("You have selected {}, {}", x, y);
            break;
        }
    }
    

    return (x, y);
}

fn try_move<'a>(board: &mut [[i8; 3]; 3], player_move: (u8, u8), val: i8) {
    board[player_move.0 as usize][player_move.1 as usize] = val;
}

fn check_win(board: &[[i8; 3]; 3], player: i8) -> bool {
   return check_columns(board, player) || check_rows(board, player) || check_diagonals(board, player);
}

fn check_rows(board: &[[i8; 3]; 3], player: i8) -> bool {
    for i in 0..3 {
        if (board[i][0] == player && board[i][1] == player && board[i][2] == player) {
            return true;
        }
    }
    return false;
}

fn check_columns(board: &[[i8; 3]; 3], player: i8) -> bool {
    for i in 0..3 {
        if (board[0][i] == player && board[1][i] == player && board[2][i] == player) {
            return true;
        }
    }
    return false;
}

fn check_diagonals(board: &[[i8; 3]; 3], player: i8) -> bool {
    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true
    }
    if board[2][0] == player && board[1][1] == player && board[0][2] == player {
        return true
    }
    return false;
}

fn is_board_full(board: &[[i8; 3]; 3]) -> bool {
    for x in 0..3 {
        for y in 0..3 {
            if board[y as usize][x as usize] == 0 {
                return false;
            }
        }
    }
    return true;
}

fn move_is_valid(board: &[[i8; 3]; 3], player_move: (u8, u8)) -> bool {
    if player_move.0 > 2 || player_move.1 > 2 {
        return false;
    }

    if board[player_move.1 as usize][player_move.0 as usize] != 0 {
        return false;
    }

    return true;
}