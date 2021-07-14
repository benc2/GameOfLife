use std::ops::{Index, IndexMut};
use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufRead, BufWriter, Write};

#[derive(Clone)]
pub struct Board {
    boardsize_x: usize,
    boardsize_y: usize,
    board: Vec<Vec<bool>>
}


impl Board{
    pub fn new(boardsize_x: usize, boardsize_y: usize) -> Board {
        Board{boardsize_x: boardsize_x, 
              boardsize_y: boardsize_x, 
              board:       vec![vec![false; boardsize_x]; boardsize_y]}
    }

    pub fn read(filename: &str) -> Board {
        let f = BufReader::new(File::open(filename).unwrap());
        let matrix: Vec<Vec<bool>> = f.lines()
            .map(|l| l.unwrap().split(char::is_whitespace)
                    .map(|number| number.parse::<usize>().unwrap() != 0)
                    .collect())
            .collect();
        Board{
            boardsize_y: matrix.len(),
            boardsize_x: matrix[0].len(),
            board: matrix
        }
    }

    pub fn write(&self, filename: &str) {
        let mut f = BufWriter::new(File::create(filename).unwrap());
        for j in 0..self.boardsize_y {
            for i in 0..self.boardsize_x {
                if i < self.boardsize_x-1 {
                    f.write(format!("{} ", self[j][i] as usize).as_bytes()).unwrap();
                }else {
                    f.write(format!("{}", self[j][i] as usize).as_bytes()).unwrap();
                }
                
            }
            f.write(b"\n").unwrap();
        }
        f.flush().unwrap();
    }

    fn step(&mut self) {
        let mut new_board = self.clone();
        for i in 0..self.boardsize_x {
            for j in 0..self.boardsize_y {
                new_board[j][i] = self.new_cell_status(i, j);
            }
        }
        *self = new_board;
    }

    fn new_cell_status(&self, x: usize, y: usize) -> bool{
        let mut active_neighbors = 0;
        for i in 0..=2 {
            for j in 0..=2 {
                if i == 1 && j == 1 {
                    continue;
                }
                if x + i >= 1 && x + i <= self.boardsize_x && y + j >= 1 && y + j <= self.boardsize_y {
                    //println!("x: {}, y: {}, i: {}, j: {}", x, y, i, j);
                    if self.board[y+j-1][x+i-1] {
                        active_neighbors += 1;
                    }
                } 
            }
        }
        //println!("Neighbours of cell {} {}: {}", x, y, active_neighbors);
        if self.board[y][x] {  // if cell is populated
            return !(active_neighbors <= 1 || active_neighbors >= 4);
        }
        else{
            return active_neighbors == 3;
        }
    }

    pub fn do_steps_and_print(&mut self, steps: usize) {
        for i in 0..steps {
            println!("At step: {}\n{}", i, self);
            self.step();
        }
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