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
