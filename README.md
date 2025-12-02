# Nonogram Solver

A command-line tool written in Rust to solve Nonogram (Picross, Griddlers) puzzles.

It works by generating all possible line configurations (possibilities) and iteratively refining the solution using the

cross-constraints from rows and columns until the map converges.

## ⚙️ Arguments (Clap)

| Short | 	Long              | 	Default Value | 	Description                                                                            |
|:------|:-------------------|:---------------|:----------------------------------------------------------------------------------------|
| -r    | --rows-file        | N/A            | (Required) Path to the file containing row clues.                                       |
| -c    | --cols-file        | N/A            | (Required) Path to the file containing column clues.                                    |
| -p    | --process          | false          | If set, the map will be printed after each iteration step, showing the solving process. |
| -E    | --empty-symbol     | " "  (space)   | Symbol used to display an 'Empty' cell.                                                 |
| -U    | --uncertain-symbol | ?              | Symbol used to display an 'Uncertain' cell.                                             |
| -F    | --filled-symbol    | X              | Symbol used to display a 'Filled' cell.                                                 |
