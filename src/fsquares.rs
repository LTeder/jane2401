extern crate ndarray;
extern crate rand;

use std::fmt;
use self::ndarray::Array2;
use self::rand::Rng;

pub struct Cell {
    pub top: bool,
    pub bottom: bool,
    left: bool,
    right: bool,
}

impl Cell {
    pub fn new(top: bool, bottom: bool, left: bool, right: bool,) -> Self {
        Self {top, bottom, left, right}
    }
}

#[derive(Debug)]
pub struct Area {
    xs: Vec<usize>,
    ys: Vec<usize>,
    fis: Vec<usize> // f-square indecies
}

impl Area {
    pub fn new(xs: Vec<usize>, ys: Vec<usize>) -> Self {
        let fis: Vec<usize> = Vec::new();
        Self {xs, ys, fis}
    }

    pub fn push_cell(&mut self, cell: (usize, usize)) {
        self.xs.push(cell.0 as usize);
        self.ys.push(cell.1 as usize);
    }
}

pub struct Board {
    grid: Array2<Cell>,
    row_hashes: Vec<usize>,
    col_hashes: Vec<usize>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print the top border
        writeln!(f, " {}", "_".repeat(2 * self.grid.ncols() + 2))?;
        // Iterate over the grid
        for (i, row) in self.grid.outer_iter().enumerate() {
            // Print the row hash at the start of each row
            write!(f, "|{:>2}|", self.row_hashes[i])?;

            let mut row_string = String::new();
            for cell in row {
                // Handle the cases based on the cell properties
                match cell {
                    Cell { bottom: true, right: true, .. } if cell.bottom && cell.right => {
                        // If the cell has a top wall and a left wall
                        row_string.push_str("_|");
                    }
                    Cell { bottom: true, .. } => {
                        // If the cell has a top wall, print an underscore
                        row_string.push_str("_ ");
                    }
                    Cell { right: true, .. } => {
                        // If the cell has a left wall
                        row_string.push_str(" |");
                    }
                    _ => {
                        // If the cell has neither a top wall nor a left wall, print two spaces
                        row_string.push_str("  ");
                    }
                }
            }
            // Replace the last space with a vertical bar and add a newline
            let len = row_string.len();
            row_string = format!("{}|", &row_string[..len-1]);
            writeln!(f, "{}", row_string)?;
        }

        // Print the bottom border
        writeln!(f, " {}", "‾".repeat(2 * self.grid.ncols() + 2))?;

        // Print the col_hashes
        write!(f, "    ")?;
        for &col_hash in &self.col_hashes {
            write!(f, "{:>2}", col_hash)?;
        }
        writeln!(f)?;

        Ok(())
    }
}

impl Board {
    pub fn new(grid: Array2<Cell>, row_hashes: Vec<usize>, col_hashes: Vec<usize>) -> Self {
        Self {
            grid,
            row_hashes,
            col_hashes,
        }
    }
    pub fn get_score(self) -> () {
        //println!("{:?}", self.get_areas());
    }
    // Sum area scores, rows, and columns 
    pub fn get_areas(&self) -> Vec<Area> {
        let mut areas: Vec<Area> = Vec::new();
        let mut visited = vec![vec![false; self.grid.ncols()]; self.grid.nrows()];

        for i in 0..self.grid.nrows() {
            for j in 0..self.grid.ncols() {
                if !visited[i][j] && !self.grid[[i, j]].bottom && !self.grid[[i, j]].right {
                    //let area: Area = self.find_area(i, j, &mut visited);
                    //areas.push(area);
                }
            }
        }
        areas
    }

    fn find_area(&self, i: usize, j: usize, visited: &mut Vec<Vec<bool>>) -> () {
        if i >= self.grid.nrows() || j >= self.grid.ncols() || visited[i][j] {
            return;
        }

        visited[i][j] = true;

        if !self.grid[[i, j]].bottom && !self.grid[[i, j]].right {
            //area.push_cell((i, j));

            self.find_area(i + 1, j, visited);
            if i > 0 {
                self.find_area(i - 1, j, visited);
            }
            self.find_area(i, j + 1, visited);
            if j > 0 {
                self.find_area(i, j - 1, visited);
            }
        }
    }
}

pub struct RandomSearch {
    board: Board,
    iterations: usize,
}

impl RandomSearch {
    pub fn new(board: Board, iterations: usize) -> Self {
        Self {
            board,
            iterations,
        }
    }

    pub fn run(&mut self) {
        let mut rng = rand::thread_rng();
        println!("{}", self.board);
        for _ in 0..self.iterations {
            // Create and evaluate solutions here
        }
    }
}
