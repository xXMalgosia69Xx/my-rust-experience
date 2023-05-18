use std::io;

fn print_board(board: [[i8; 3]; 3], player: i8) {
    // clear the screen
    print!("{}[2J", 27 as char);
    println!("{}'s turn", if player == 1 { "X" } else { "O" });
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == 0 { print!("{}\t", i * 3 + j + 1); }
            else if board[i][j] == 1 { print!("X\t"); }
            else { print!("O\t"); }
        }
        println!("\n");
    }
}

fn did_win(board: [[i8; 3]; 3], player: i8) -> bool {
    for i in 0..3 {
        if board[i][0] == player && board[i][1] == player && board[i][2] == player { return true; }
        if board[0][i] == player && board[1][i] == player && board[2][i] == player { return true; }
    }
    if board[0][0] == player && board[1][1] == player && board[2][2] == player { return true; }
    if board[0][2] == player && board[1][1] == player && board[2][0] == player { return true; }
    return false;
}

fn is_draw(board: [[i8; 3]; 3]) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == 0 { return false; }
        }
    }
    return true;
}

fn main() {
    let mut board: [[i8; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    let mut player: i8 = 1;

    loop {
        // print board
        print_board(board, player);
        // get input
        // check if input is valid
        // accept input
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input: i8 = input.trim().parse().expect("Please type a number!");

            if input < 1 || input > 9 {
                println!("Please type a number between 1 and 9");
                continue;
            } else if board[((input - 1) / 3) as usize][((input - 1) % 3) as usize] != 0 {
                println!("This field is already taken");
                continue;
            } else {
                board[((input - 1) / 3) as usize][((input - 1) % 3) as usize] = player;
                break;
            }
        }
        // check if there is a winner
        if did_win(board, player) {
            print_board(board, player);
            println!("{} won!", if player == 1 { "X" } else { "O" });
            break;
        }
        // check if it's a draw
        if is_draw(board) {
            print_board(board, player);
            println!("It's a draw!");
            break;
        }
        // change player
        player = if player == 1 { 2 } else { 1 };
    }
}
