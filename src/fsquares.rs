extern crate ndarray;
extern crate rand;

use std::fmt;
use self::ndarray::Array2;
//use self::rand::Rng;

#[derive(Clone)]
pub struct Cell {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub fi: usize, // f-square index
}

impl Cell {
    pub fn new(top: bool, bottom: bool, left: bool, right: bool,) -> Self {
        let fi = 0;
        Self {top, bottom, left, right, fi}
    }
}

#[derive(Debug)]
struct Area {
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

#[derive(Debug)]
pub struct FSquares {
    squares: Vec<Area>
}

impl FSquares {
    pub fn new() -> Self {
        let squares = Vec::new();
        Self {squares: squares}
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

    pub fn get_score(&self) -> usize {
        //println!("Getting area scores...");
        let areas = self.get_areas();
        let mut sum: usize;
        for (i, area) in areas.iter().enumerate() {
            sum = 0;
            for (&x, &y) in area.xs.iter().zip(&area.ys) {
                sum += self.grid[[x, y]].fi;
            }
            //println!("{:>2}: {}", i, sum);
        }
        
        //println!("Getting axis scores...");
        let row_sums: Vec<usize> = self.grid.axis_iter(ndarray::Axis(0))
            .map(|row| row.map(|cell| cell.fi).sum())
            .collect();
        let col_sums: Vec<usize> = self.grid.axis_iter(ndarray::Axis(1))
            .map(|col| col.map(|cell| cell.fi).sum())
            .collect();
        //println!("{:?}\n{:?}", row_sums, col_sums);

        //println!("Getting blank cells...");
        let blank_areas = self.get_blank_areas();
        //println!("Getting product of the length of each blank area...");
        let mut product: usize = 1;
        for area in blank_areas.iter() {
            let length = area.xs.len();
            product *= length;
        }
        //println!("{:?}", blank_areas);
        product
    }
    
    fn get_areas(&self) -> Vec<Area> {
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

    fn find_area(&self, i: usize, j: usize, visited: &mut Vec<Vec<bool>>, area: &mut Area) {
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
    
    fn get_blank_areas(&self) -> Vec<Area> {
        let mut blank_areas: Vec<Area> = Vec::new();
        let mut visited = vec![vec![false; self.grid.ncols()]; self.grid.nrows()];
        // Check every cell as in get_areas
        for i in 0..self.grid.nrows() {
            for j in 0..self.grid.ncols() {
                if self.grid[[i, j]].fi == 0 && !visited[i][j] {
                    let mut area = Area::new();
                    self.find_blank_area(i, j, &mut visited, &mut area);
                    blank_areas.push(area);
                }
            }
        }
        blank_areas
    }

    fn find_blank_area(&self, i: usize, j: usize, visited: &mut Vec<Vec<bool>>, area: &mut Area) {
        if i >= self.grid.nrows() || j >= self.grid.ncols() || visited[i][j] {
            return;
        }

        area.push_cell((j, i)); // get_areas and visited does y then x
        visited[i][j] = true;

        if i + 1 < self.grid.nrows() && self.grid[[i + 1, j]].fi == 0 {
            self.find_blank_area(i + 1, j, visited, area);
        }
        if i > 0 && self.grid[[i - 1, j]].fi == 0 { // i|j=0 should have top|left anyways
            self.find_blank_area(i - 1, j, visited, area);
        }
        if j + 1 < self.grid.ncols() && self.grid[[i, j + 1]].fi == 0 {
            self.find_blank_area(i, j + 1, visited, area);
        }
        if j > 0 && self.grid[[i, j - 1]].fi == 0 {
            self.find_blank_area(i, j - 1, visited, area);
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

    fn attempt_generation(&self) {

    }

    fn generate_squares(&self) -> FSquares {
        const MAX_ATTEMPTS: usize = 1000;
        let mut attempts = 0;
        while attempts < MAX_ATTEMPTS {
            attempts += 1;
        }
        let squares = FSquares::new();
        squares
    }

    pub fn run(&mut self) {
        //let mut rng = rand::thread_rng();
        let mut squares: FSquares = self.generate_squares();
        let mut score = self.board.get_score();
        let mut best = squares;
        let mut best_score = score;
        println!("\nBoard:\n{}\nFirst Score: {}", self.board, score);
        for _ in 0..self.iterations {
            squares = self.generate_squares();
            score = self.board.get_score();
            if score > best_score {
                best_score = score;
                best = squares;
            }
        }
        println!("\nBest:\n{:?}\nScore: {}", best, best_score)
    }
}
