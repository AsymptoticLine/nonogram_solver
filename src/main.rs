use std::{error::Error, fs, process::ExitCode};

use clap::Parser;
use nonogram_solver::{config::Config, process::solve_nonogram, puzzle_info::PuzzleInfo};

#[derive(Parser, Clone)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    rows_file: String,

    #[arg(short, long)]
    cols_file: String,

    #[arg(short, long)]
    process: bool,

    #[arg(short = 'E', long, default_value = " ")]
    empty_symbol: String,

    #[arg(short = 'U', long, default_value = "?")]
    uncertain_symbol: String,

    #[arg(short = 'F', long, default_value = "X")]
    filled_symbol: String,
}

fn config_from_args(args: Args) -> Config {
    Config::new(
        args.process,
        args.empty_symbol,
        args.uncertain_symbol,
        args.filled_symbol,
    )
}

fn parse_blocks(filepath: String) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
    let content = fs::read_to_string(filepath)?;
    let blocks = content
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|group| group.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(blocks)
}

fn mapinfo_from_args(args: Args) -> Result<PuzzleInfo, Box<dyn Error>> {
    let row_blocks = parse_blocks(args.rows_file)?;
    let col_blocks = parse_blocks(args.cols_file)?;

    if row_blocks.is_empty() || col_blocks.is_empty() {
        Err("Invalid blocks".into())
    } else {
        Ok(PuzzleInfo::new(
            col_blocks.len(),
            col_blocks.len(),
            row_blocks,
            col_blocks,
        ))
    }
}

fn main() -> ExitCode {
    let cli = Args::parse();
    let config = config_from_args(cli.clone());

    match mapinfo_from_args(cli) {
        Ok(map_info) => match solve_nonogram(map_info, config) {
            Ok(_) => ExitCode::SUCCESS,
            Err(msg) => {
                eprintln!("Failed to solve nonogram. {:}", msg);
                ExitCode::FAILURE
            }
        },
        Err(msg) => {
            eprintln!("Failed to read and parse file. {:}", msg);
            ExitCode::FAILURE
        }
    }
}
