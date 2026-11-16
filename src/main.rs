const BOARD_WIDTH: usize = 3;
const BOARD_HEIGHT: usize = 3;

fn print_board(board: [[char; BOARD_WIDTH]; BOARD_HEIGHT]) {
    for i in 0..BOARD_HEIGHT {
        for j in 0..BOARD_WIDTH {
            print!("{}", board[i][j]);
        }
        println!();
    }
}

fn mark_tile() {
    
}

fn main() {
    let mut board = [['-'; 3]; 3];
    
    println!("Welcome to Tic-Tac-Toe! You (X) move first.");
    print_board(board);
    

}