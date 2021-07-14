use std::ops::{Index, IndexMut};
use std::fmt;

#[derive(Clone)]
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

    fn step(&mut self) {
        let new_board = self.clone();
        for i in 0..self.boardsize_x {
            for j in 0..self.boardsize_y {
                new_board[i][j] = self.new_cell_status(i, j);
            }
        }
        self = new_board;
    }

}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Board[{}, {},\n", self.boardsize_x, self.boardsize_y)?;
        for i in 0..self.boardsize_x {
            write!(f, "|")?;
            for j in 0..self.boardsize_y {
                if self[i][j] {
                    write!(f, "█")?;
                }else {
                    write!(f, "▒")?;
                }
                
            }
            write!(f, "|\n")?;
        }
        write!(f, "]")
    }
}

impl Index<usize> for Board {
    type Output = Vec<bool>;
    fn index(&self, index: usize) -> &Self::Output{
        &self.board[index]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.board[index]
    }
}

fn main() {
    let mut board = Board::new(10, 10);
    board[0][0] = true;
    board[0][1] = true;
    println!("{}", board[0][0]);
    println!("{}", board);
}
