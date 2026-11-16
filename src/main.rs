const BOARD_WIDTH: usize = 3;
const BOARD_HEIGHT: usize = 3;

fn print_board(board: [[i32; BOARD_WIDTH]; BOARD_HEIGHT]) {
    for i in 0..BOARD_HEIGHT {
        for j in 0..BOARD_WIDTH {
            print!("- ");
        }
        println!();
    }
}

fn main() {
    let mut board = [[0; 3]; 3];

    print_board(board);
}