extern crate ndarray;
extern crate rand;

use std::fmt;
use self::ndarray::Array2;
//use self::rand::Rng;

pub struct Cell {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub ai: usize, // area index
    pub fi: usize, // f-square index
}

impl Cell {
    pub fn new(top: bool, bottom: bool, left: bool, right: bool,) -> Self {
        let ai = 0;
        let fi = 0;
        Self {top, bottom, left, right, ai, fi}
    }
}

#[derive(Debug)]
pub struct Area {
    xs: Vec<usize>,
    ys: Vec<usize>,
}

impl Area {
    pub fn new() -> Self {
        let xs = Vec::new();
        let ys = Vec::new();
        Self {xs, ys}
    }

    pub fn push_cell(&mut self, cell: (usize, usize)) {
        self.xs.push(cell.0 as usize);
        self.ys.push(cell.1 as usize);
    }
}

pub struct Board {
    grid: Array2<Cell>, // Indexed like an image
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
                match cell { // Build row based on cell contents
                    Cell { bottom: true, right: true, .. } if cell.bottom && cell.right => {
                        row_string.push_str("_|");
                    }
                    Cell { bottom: true, .. } => {
                        row_string.push_str("_ ");
                    }
                    Cell { right: true, .. } => {
                        row_string.push_str(" |");
                    }
                    _ => {
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
    pub fn get_score(&self) -> () {
        let areas = self.get_areas();
        for (i, area) in areas.iter().enumerate() {
            println!("\n{}", i);
            for (&x, &y) in area.xs.iter().zip(&area.ys) {
                println!("{:>2}, {:>2}", x, y);
            }
        }
    }
    // Sum area scores, rows, and columns 
    pub fn get_areas(&self) -> Vec<Area> {
        let mut areas: Vec<Area> = Vec::new();
        let mut visited = vec![vec![false; self.grid.ncols()]; self.grid.nrows()];
        // Check every cell
        for i in 0..self.grid.nrows() { // Going from top of y-axis down
            for j in 0..self.grid.ncols() { // x-axis
                if !visited[i][j] && !self.grid[[i, j]].bottom && !self.grid[[i, j]].right {
                    let mut area = Area::new(); // Blank
                    self.find_area(i, j, &mut visited, &mut area); // Completely fill
                    areas.push(area);
                }
            }
        }
        areas
    }

    fn find_area(&self, i: usize, j: usize, visited: &mut Vec<Vec<bool>>, area: &mut Area) -> () {
        if i >= self.grid.nrows() || j >= self.grid.ncols() || visited[i][j] {
            return;
        }

        area.push_cell((j, i)); // get_areas and visited does y then x
        visited[i][j] = true;

        let cell = &self.grid[[i, j]];
        if !cell.bottom {
            self.find_area(i + 1, j, visited, area);
        }
        if !cell.top && i > 0 { // i|j=0 should have top|left anyways
            self.find_area(i - 1, j, visited, area);
        }
        if !cell.right {
            self.find_area(i, j + 1, visited, area);
        }
        if !cell.left && j > 0 {
            self.find_area(i, j - 1, visited, area);
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
        //let mut rng = rand::thread_rng();
        println!("{}", self.board);
        println!("{:?}", self.board.get_score());
        for _ in 0..self.iterations {
            // Generate new board
            // Get score, compare to previous champion
        }
    }
}
