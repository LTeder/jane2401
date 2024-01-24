extern crate clap;

mod helper;
mod fsquares;

use clap::Parser;
use fsquares::Board;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'f', long)]
    points_filename: String,

    #[arg(short, long, default_value_t = 100)]
    iterations: usize
}

fn main() {
    let args = Args::parse();
    match helper::read_board_from_file(&args.points_filename) {
        Ok((ndarray_board, col_hashes, row_hashes)) => {
            let mut board = Board::new(ndarray_board, row_hashes, col_hashes);
            // Run simulation
            let mut search = fsquares::RandomSearch::new(
                board,
                args.iterations,
            );
            search.run();
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        },
    }

}
