extern crate ndarray;
extern crate rand;

use std::fmt;
use self::ndarray::{Array2, Axis};
use self::rand::Rng;

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
    coords: Vec<(usize, usize)>,
}

impl Area {
    pub fn new() -> Self {
        let coords: Vec<(usize, usize)> = Vec::new();
        Self {coords}
    }

    pub fn push_cell_loc(&mut self, cell: (usize, usize)) {
        self.coords.push(cell);
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

    pub fn get_score(&self) -> Result<usize, &'static str> {
        // Area score
        let areas = self.get_areas();
        let mut sums: Vec<usize> = Vec::new();
        for area in areas.iter() {
            let mut sum = 0;
            for (x, y) in area.coords.iter() {
                sum += self.grid[[*x, *y]].fi;
            }
            sums.push(sum);
        }
        let sums_same = match sums.first() { // Ensure every element is the same
            None => true,
            Some(first) => sums.iter().all(|&x| x == *first),
        };
        if !sums_same {
            return Err("Not all area sums are the same");
        }
        
        // Axis score
        let row_sums: Vec<usize> = self.grid.axis_iter(Axis(0))
            .map(|row| row.map(|cell| cell.fi).sum())
            .collect();
        let col_sums: Vec<usize> = self.grid.axis_iter(Axis(1))
            .map(|col| col.map(|cell| cell.fi).sum())
            .collect();
        if row_sums != self.row_hashes {
            return Err("Row sums do not match the board specification");
        }
        if col_sums != self.col_hashes {
            return Err("Column sums do not match the board specification");
        }

        // Blank cells score (product of the length of each blank area)
        let blank_areas = self.get_blank_areas();
        let mut product: usize = 1;
        for area in blank_areas.iter() {
            product *= area.coords.len();
        }
        Ok(product)
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

        area.push_cell_loc((j, i)); // get_areas and visited does y then x
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

        area.push_cell_loc((j, i)); // get_areas and visited does y then x
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

    pub fn reset_fis(&mut self){
        for cell in self.grid.iter_mut() {
            cell.fi = 0;
        }
    }
}

#[derive(Debug)]
struct FSquares {
    squares: Vec<Area>
}

impl FSquares {
    pub fn new() -> Self {
        let squares = Vec::new();
        Self {squares: squares}
    }

    pub fn add(&mut self, area: Area) {
        self.squares.push(area);
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

    fn attempt_generation(&mut self, squares: &mut FSquares) -> Result<(), &'static str> {
        let mut rng = rand::thread_rng();
        // Randomly select a side length 1 through 5 and rotation
        let size = rng.gen_range(1..=5);
        let side_length = size * 3;
        let rotation = rng.gen_range(0..=3);
        // Randomly select a square of cells in self.board using that side length
        let max_row_index = self.board.grid.nrows() - side_length;
        let max_col_index = self.board.grid.ncols() - side_length;
        // Define indexing values
        let start_row = rng.gen_range(0..=max_row_index);
        let start_col = rng.gen_range(0..=max_col_index);
        let center_start_row = start_row + side_length / 3; // p + s/3 = p + s/2 - s/3/2
        let center_start_col = start_col + side_length / 3;
        let start = (center_start_row, center_start_col);
        let mut is_occupied = false;
        // See if randomly-placed F-square is valid
        let locs = self.get_fshape_locs(start, size, rotation);
        for (i, j) in &locs {
            if self.board.grid[[*i, *j]].fi != 0 {
                is_occupied = true;
                break;
            }
        }
        if is_occupied { // Window is occupied, so we skip this iteration
            Err("Window is occupied")
        } else { // Place it
            let mut area = Area::new();
            for (i, j) in &locs {
                self.board.grid[[*i, *j]].fi = side_length;
                area.push_cell_loc((*i, *j));
            }
            squares.add(area);
            Ok(())
        }
    }

    fn get_fshape_locs(&self, start: (usize, usize), size: usize, rotation: usize) -> Vec<(usize, usize)> {
        let (start_row, start_col) = start;
        let mut cell_locs = Vec::new();
        cell_locs.push(start);
        for i in 0..size { // subcells
            let sri = start_row + i;
            let sci = start_col + i;
            match rotation { // same shape, rotated
                0 => {
                    cell_locs.push((sri,        sci - size)); // left
                    cell_locs.push((sri,        sci + size)); // right
                    cell_locs.push((sri + size, sci));        // bottom
                    cell_locs.push((sri - size, sci + size)); // above the right
                },
                1 => {
                    cell_locs.push((sri,        sci - size)); // left
                    cell_locs.push((sri - size, sci));        // top
                    cell_locs.push((sri + size, sci));        // bottom
                    cell_locs.push((sri + size, sci + size)); // right of bottom
                },
                2 => {
                    cell_locs.push((sri,        sci - size)); // left
                    cell_locs.push((sri - size, sci));        // top
                    cell_locs.push((sri,        sci + size)); // right
                    cell_locs.push((sri + size, sci - size)); // below the left
                },
                3 => {
                    cell_locs.push((sri - size, sci));        // top
                    cell_locs.push((sri,        sci + size)); // right
                    cell_locs.push((sri + size, sci));        // bottom
                    cell_locs.push((sri - size, sci - size)); // left of top
                },
                _ => (),
            }
        }
        cell_locs
    }

    fn generate_squares(&mut self) -> (FSquares, usize) {
        const MAX_ATTEMPTS: usize = 1000;
        let squares = FSquares::new();
        let score = 0;
        let mut _loop_count = 0;
        loop {
            self.board.reset_fis();
            let mut _error_count = 0;
            let mut squares = FSquares::new();
            for _ in 0..MAX_ATTEMPTS { // Try to build a board one f-square at a time
                let result = self.attempt_generation(&mut squares);
                if result.is_err() {
                    _error_count += 1;
                }
            }
            // Return the result or retry
            let score = self.board.get_score();
            if score.is_ok() {
                break;
            } else {
                _loop_count += 1;
                println!("Scoring error: {}", score.unwrap_err());
            }
        }
        println!("{} loops", _loop_count);
        (squares, score)
    }

    pub fn run(&mut self) {
        let mut squares: FSquares;
        let mut best = FSquares::new();
        let mut score = 99999999999999;
        let mut best_score = score;
        println!("\nBoard:\n{}", self.board);
        let pb = indicatif::ProgressBar::new(self.iterations as u64);
        for _ in 0..self.iterations {
            pb.inc(1);
            (squares, score) = self.generate_squares();
            if score > best_score {
                best_score = score;
                best = squares;
            }
        }
        pb.finish();
        println!("\nBest:\n{:?}\nScore: {}", best, best_score)
    }
}
