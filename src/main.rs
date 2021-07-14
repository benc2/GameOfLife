mod board;
use board::Board;

fn main() {
    let mut board = Board::read("example.con");
    board.do_steps_and_print(10);
    board.write("example.con");
}