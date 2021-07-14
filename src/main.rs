use std::ops::{Index};


struct Board {
    boardsize_x: usize,
    boardsize_y: usize,
    board: Vec<Vec<bool>>
}


impl Board{
    fn new(boardsize_x: usize, boardsize_y: usize) -> Board {
        Board{boardsize_x: boardsize_x, 
              boardsize_y: boardsize_x, 
              board:       vec![vec![false; boardsize_x]; boardsize_y]}
    }

    fn new_cell_status(&self, x: usize, y: usize) -> bool{
        let active_neighbors = 0;
        for i in -1..=1 {  // offset in x direction
            for j in -1..=1 {
                if i == 0 && j == 0 {  // offset in y direction
                    continue;
                }
                if !(x + i < 0 || x + i > self.boardsize_x ||y + j < 0 || y + j > self.boardsize_y) { // don't check outside the board
                    if self.board[y+j][x+i] {
                        active_neighbors += 1;
                    }
                }

            }
        }
        if self.board[y][x] {  // if cell is populated
            return !(active_neighbors <= 1 || active_neighbors >= 4);
        }
        else{
            return active_neighbors == 2 || active_neighbors == 3;
        }
    }
}

impl Index<usize> for Board{
    type Output = Vec<bool>;
    fn index(&self, index: usize) -> &Self::Output{
        &self.board[index]
    }
}

fn main() {
    println!("Hello world");
    let board = Board::new(100, 100);
    let x: bool = board[0][0];
    if x {
        println!("true");
    }
    else{
        println!("false");
    }
    // println!("{}", );
}
