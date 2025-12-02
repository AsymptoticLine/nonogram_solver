use crate::cell::Cell;

/// Generates all possible valid placements (possibilities) for a single row or column.
///
/// This uses a backtracking approach.
///
/// @param clues The set of consecutive filled block sizes (e.g., [2, 1, 3]).
/// @param line_size The total number of cells in the line.
/// @return A vector containing all valid line configurations.
pub fn generate_line_possibilities(blocks: &Vec<usize>, line_size: usize) -> Vec<Vec<Cell>> {
    let mut results = Vec::new();

    fn generate_possibilities_recursive(
        clues: &Vec<usize>,
        clue_index: usize,
        current_pos: usize,
        current_line: &mut Vec<Cell>,
        line_size: usize,
        min_line_length: usize,
        results: &mut Vec<Vec<Cell>>,
    ) {
        if current_pos + min_line_length > line_size {
            return;
        }

        if clue_index == clues.len() {
            // Fill the remaining space with Cell::Empty
            current_line.resize(line_size, Cell::Empty);
            results.push(current_line.clone());

            // Backtrack: truncate current_line to the state before this call
            current_line.truncate(current_pos);
            return;
        }

        let block_size = clues[clue_index];

        let max_start_pos = line_size - (min_line_length - block_size);

        for start_pos in current_pos..=max_start_pos {
            let num_blanks = start_pos - current_pos;
            current_line.extend(std::iter::repeat(Cell::Empty).take(num_blanks));

            current_line.extend(std::iter::repeat(Cell::Filled).take(block_size));

            let next_pos_after_block = start_pos + block_size;
            let mut next_pos = next_pos_after_block;

            if clue_index < clues.len() - 1 {
                current_line.push(Cell::Empty);
                next_pos += 1;
            }

            let next_block_index = clue_index + 1;

            let new_min_required_len = if next_block_index < clues.len() {
                clues[next_block_index..].iter().sum::<usize>()
                    + (clues.len() - next_block_index - 1)
            } else {
                0
            };

            generate_possibilities_recursive(
                clues,
                next_block_index,
                next_pos,
                current_line,
                line_size,
                new_min_required_len,
                results,
            );

            current_line.truncate(current_pos);
        }
    }

    // Calculate the minimum required length for all clues and their separators
    let min_required_len: usize = if blocks.is_empty() {
        0
    } else {
        blocks.iter().sum::<usize>() + blocks.len() - 1
    };

    if min_required_len > line_size {
        return Vec::new();
    }

    let mut current_line: Vec<Cell> = Vec::with_capacity(line_size);

    generate_possibilities_recursive(
        &blocks,
        0,
        0,
        &mut current_line,
        line_size,
        min_required_len,
        &mut results,
    );

    results
}
