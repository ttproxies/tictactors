const BOARD_WIDTH: usize = 3;
const BOARD_HEIGHT: usize = 3;
const ALLOWED_MARKS: &str = "XO";

fn print_board(board: [[char; BOARD_WIDTH]; BOARD_HEIGHT]) {
    for i in 0..BOARD_HEIGHT {
        for j in 0..BOARD_WIDTH {
            print!("{} ", board[i][j]);
        }
        println!();
    }
}

fn main() {
    let human_mark = std::env::args().nth(1).expect("No mark specified [X, O]");

    if !ALLOWED_MARKS.contains(human_mark) {
        println!("Invalid mark specified (must be X or O");
    }
    
    let mut board = [['-'; 3]; 3];
    
    println!("Welcome to Tic-Tac-Toe! X Moves first.");
    print_board(board); 

    let player_turn = if human_mark == 'X' { true } else { false };
    loop {
        
    }
}