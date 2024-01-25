extern crate clap;

mod helper;
mod fsquares;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "board.txt")]
    points_filename: String,

    #[arg(short, long, default_value_t = 100)]
    iterations: usize
}

fn main() {
    let args = Args::parse();
    match helper::read_board_from_file(&args.points_filename) {
        Ok(board) => {
            // Run simulation
            let mut search = fsquares::RandomSearch::new(
                board,
                args.iterations,
            );
            search.run();
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

}
