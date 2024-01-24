extern crate ndarray;

use self::ndarray::Array2;
use fsquares::Cell;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_board_from_file(filename: &str) ->
        io::Result<(Array2<Cell>, Vec<usize>, Vec<usize>)> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut board: Vec<Vec<Cell>> = Vec::new();
    let mut dimensions: (usize, usize) = (0, 0);
    let mut section2: Vec<Vec<usize>> = Vec::new();
    let mut is_section2 = false;

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
        for char in line.chars() {
            match char {
                '|' => row.push(Cell::VerticalWall),
                '_' => row.push(Cell::HorizontalWall),
                num if num.is_digit(10) => {
                    let num: usize = num.to_string().parse().unwrap();
                    for _ in 0..num {
                        row.push(Cell::Empty);
                    }
                }
                _ => (),
            }
        }

        dimensions.1 = row.len();
        dimensions.0 = i + 1;
        board.push(row);
    }

    let flat_board: Vec<Cell> = board.into_iter().flatten().collect();
    let ndarray_board = Array2::from_shape_vec(dimensions, flat_board).unwrap();

    let section2_flat: Vec<usize> = section2.into_iter().flatten().collect();
    let col_hashes: Vec<usize> = section2_flat[..dimensions.1].to_vec();
    let row_hashes: Vec<usize> = section2_flat[dimensions.1..].to_vec();

    Ok((ndarray_board, col_hashes, row_hashes))
}
