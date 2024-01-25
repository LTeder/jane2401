extern crate ndarray;

use self::ndarray::Array2;
use fsquares::{Cell, Board};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_board_from_file(filename: &str) -> io::Result<Board> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut grid: Vec<Vec<Cell>> = Vec::new();
    let mut dimensions: (usize, usize) = (0, 0);
    let mut section2: Vec<Vec<usize>> = Vec::new();
    let mut is_section2 = false;
    let mut previous_line: Vec<char> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if line.is_empty() {
            is_section2 = true;
            continue;
        }

        if is_section2 {
            let numbers: Vec<usize> = line.split_whitespace()
                                          .map(|s| s.parse().unwrap())
                                          .collect();
            section2.push(numbers);
            continue;
        }

        let mut row: Vec<Cell> = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        for (j, char) in chars.iter().enumerate() {
            if j % 2 == 0 {
                continue;
            }

            let top = previous_line.get(j).eq(&Some(&'_'));
            let bottom = char.eq(&'_');
            let left = chars.get(j - 1).eq(&Some(&'|'));
            let right = chars.get(j + 1).eq(&Some(&'|'));

            let cell = Cell::new(top, bottom, left, right);
            row.push(cell);
        }

        dimensions.1 = row.len();
        dimensions.0 = i + 1;
        grid.push(row);
        previous_line = chars;
    }

    let flat_grid: Vec<Cell> = grid.into_iter().flatten().collect();
    let ndarray_grid = Array2::from_shape_vec(dimensions, flat_grid).unwrap();

    let section2_flat: Vec<usize> = section2.into_iter().flatten().collect();
    let col_hashes: Vec<usize> = section2_flat[..dimensions.1].to_vec();
    let row_hashes: Vec<usize> = section2_flat[dimensions.1..].to_vec();
    
    let board = Board::new(ndarray_grid, col_hashes, row_hashes);
    Ok(board)
}
