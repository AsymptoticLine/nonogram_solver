use crate::{
    cell::{Cell, CellPattern},
    config::Config,
    generator::generate_line_possibilities,
    puzzle_info::PuzzleInfo,
};

/// Transposes a 2D vector (swaps rows and columns).
fn transpose<T: Copy>(v: &Vec<Vec<T>>) -> Result<Vec<Vec<T>>, Box<dyn std::error::Error>> {
    if v.is_empty() {
        return Err("Cannot transpose an empty vector.".into());
    }

    fn check_rectangular<T: Copy>(v: &Vec<Vec<T>>) -> Result<(), Box<dyn std::error::Error>> {
        let cols = v[0].len();
        for row in v.iter().skip(1) {
            if row.len() != cols {
                return Err(format!(
                    "Cannot transpose a non-rectangular vector: {} != {}",
                    row.len(),
                    cols
                )
                .into());
            }
        }
        Ok(())
    }

    check_rectangular(v)?;

    let cols = v[0].len();
    Ok((0..cols)
        .map(|i| v.iter().map(|row| row[i]).collect())
        .collect())
}

/// Merges all the possibilities for a single line into a CellPattern,
/// identifying cells that must be Empty or Filled across all valid possibilities.
fn merge_line_possibilities(
    possibilities: &Vec<Vec<Cell>>,
) -> Result<Vec<CellPattern>, Box<dyn std::error::Error>> {
    Ok(transpose(possibilities)?
        .iter()
        .map(|cell_possibilities| {
            let init = cell_possibilities[0].to_pattern();
            cell_possibilities[1..]
                .iter()
                .map(Cell::to_pattern)
                .fold(init, CellPattern::merge)
        })
        .collect())
}

/// Combines the definite cells derived from the row possibilities and the column possibilities.
/// Uses CellPattern::merge_known to resolve conflicts (only possible if one side is Uncertain).
fn merge_rows_and_cols(
    rows_patterns: &Vec<Vec<CellPattern>>,
    cols_patterns: &Vec<Vec<CellPattern>>,
) -> Result<Vec<Vec<CellPattern>>, Box<dyn std::error::Error>> {
    rows_patterns
        .iter()
        .zip(transpose(cols_patterns)?.iter())
        .map(|(line_from_rows, line_from_cols)| {
            line_from_rows
                .iter()
                .zip(line_from_cols.iter())
                .map(|(cell_from_rows, cell_from_cols)| {
                    CellPattern::merge_known(*cell_from_rows, *cell_from_cols)
                })
                .collect()
        })
        .collect()
}

/// Filters the possibilities for each line based on the currently known pattern (map state).
/// Any possibility that contradicts a 'Filled' or 'Empty' cell in the pattern is removed.
fn filter_lines_possibilities(
    patterns: &Vec<Vec<CellPattern>>,
    lines_possibilities: &Vec<Vec<Vec<Cell>>>,
) -> Vec<Vec<Vec<Cell>>> {
    lines_possibilities
        .iter()
        .zip(patterns.iter())
        .map(|(possibilities, pattern)| {
            possibilities
                .iter()
                .filter(|possibility| line_matches(possibility, pattern))
                .map(|possibility| possibility.clone())
                .collect()
        })
        .collect()
}

/// Checks if a single line possibility is consistent with the current known pattern.
fn line_matches(possibility: &Vec<Cell>, pattern: &Vec<CellPattern>) -> bool {
    possibility
        .iter()
        .zip(pattern.iter())
        .all(|cell_pair| match cell_pair {
            (Cell::Filled, CellPattern::Filled) => true,
            (Cell::Empty, CellPattern::Empty) => true,
            (_, CellPattern::Uncertain) => true,
            _ => false,
        })
}

/// Compares two map states and returns true if they are different (i.e., the map has changed).
fn has_map_changed(map1: &Vec<Vec<CellPattern>>, map2: &Vec<Vec<CellPattern>>) -> bool {
    !map1.iter().zip(map2.iter()).all(|(line1, line2)| {
        line1
            .iter()
            .zip(line2.iter())
            .all(|(cell1, cell2)| *cell1 == *cell2)
    })
}

/// Prints the current state of the Nonogram map to the console.
fn print_map(config: &Config, map: &Vec<Vec<CellPattern>>) {
    map.iter().for_each(|line| {
        let line_str = line
            .iter()
            .map(|cell| match cell {
                CellPattern::Empty => config.empty_symbol(),
                CellPattern::Uncertain => config.uncertain_symbol(),
                CellPattern::Filled => config.filled_symbol(),
            })
            .collect::<Vec<_>>()
            .join("");
        println!("{:}", line_str);
    });
}

/// The core iterative solving loop.
/// It repeatedly filters possibilities and merges patterns until the map converges (no change).
fn iterate(
    config: Config,
    map: Vec<Vec<CellPattern>>,
    rows_possibilities: Vec<Vec<Vec<Cell>>>,
    cols_possibilities: Vec<Vec<Vec<Cell>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let new_rows_possibilities = filter_lines_possibilities(&map, &rows_possibilities);

    let new_cols_possibilities = filter_lines_possibilities(&transpose(&map)?, &cols_possibilities);

    let new_rows_patterns: Vec<Vec<CellPattern>> = new_rows_possibilities
        .iter()
        .map(|row_possibilities| merge_line_possibilities(row_possibilities))
        .collect::<Result<_, _>>()?;

    let new_cols_patterns: Vec<Vec<CellPattern>> = new_cols_possibilities
        .iter()
        .map(|col_possibilities| merge_line_possibilities(col_possibilities))
        .collect::<Result<_, _>>()?;

    let new_map = merge_rows_and_cols(&new_rows_patterns, &new_cols_patterns)?;
    if has_map_changed(&map, &new_map) {
        if config.process() {
            print_map(&config, &new_map);
            println!();
        }

        iterate(
            config,
            new_map,
            new_rows_possibilities,
            new_cols_possibilities,
        )
    } else {
        print_map(&config, &map);
        Ok(())
    }
}

pub fn solve_nonogram(
    map_info: PuzzleInfo,
    config: Config,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Generate all initial possibilities for all rows and columns.
    let rows_possibilities: Vec<Vec<Vec<Cell>>> = map_info
        .row_blocks()
        .iter()
        .map(|blocks| generate_line_possibilities(blocks, map_info.width()))
        .collect();

    let cols_possibilities: Vec<Vec<Vec<Cell>>> = map_info
        .col_blocks()
        .iter()
        .map(|blocks| generate_line_possibilities(blocks, map_info.height()))
        .collect();

    // 2. Derive the initial definite pattern (CellPattern) from all possibilities.
    let row_patterns: Vec<Vec<CellPattern>> = map_info
        .row_blocks()
        .iter()
        .map(|blocks| {
            merge_line_possibilities(&generate_line_possibilities(&blocks, map_info.width()))
        })
        .collect::<Result<_, _>>()?;

    let col_patterns: Vec<Vec<CellPattern>> = map_info
        .col_blocks()
        .iter()
        .map(|blocks| {
            merge_line_possibilities(&generate_line_possibilities(blocks, map_info.height()))
        })
        .collect::<Result<_, _>>()?;

    // 3. Start the iterative refinement process.
    let map = merge_rows_and_cols(&row_patterns, &col_patterns)?;
    iterate(config, map, rows_possibilities, cols_possibilities)
}
