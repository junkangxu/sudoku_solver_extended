# sudoku_solver_extended

## Description
a sudoku solver for one or more variants of Sudoku puzzles. The puzzles and constraints are all currently hardcoded in the code. Future plan in the Progress Section.

## Example
```bash
// solve classic sudoku
cargo run -- --input "./inputs/classic_sudoku.txt" --sudoku-type "classic"

// solve arrow sudoku
cargo run -- --input "./inputs/arrow_sudoku.txt" --sudoku-type "classic, arrow"

// solve thermo sudoku
cargo run -- --input "./inputs/thermo_sudoku.txt" --sudoku-type "classic, thermo"

// solve non-consecutive sudoku
cargo run -- --input "./inputs/non_consecutive_sudoku.txt" --sudoku-type "classic, nonConsecutive"

// solve chess knight sudoku
cargo run -- --input "./inputs/chess_knight_sudoku.txt" --sudoku-type "classic, chessKnight"

// solve chess king sudoku
cargo run -- --input "./inputs/chess_king_sudoku.txt" --sudoku-type "classic, chessKing"

// solve chess queen sudoku
cargo run -- --input "./inputs/chess_queen_sudoku.txt" --sudoku-type "classic, chessQueen"

// solve sandwich sudoku
cargo run -- --input "./inputs/sandwich_sudoku.txt" --sudoku-type "classic, sandwich"

// solve miracle sudoku
cargo run -- --input "./inputs/miracle_sudoku.txt" --sudoku-type "classic, miracle"
```

## Progress

### Features
- [X] Adding Clap integration
- [X] Adding required parameter `--input`
- [X] Adding optional parameter `--sudoku-type`. Default value: classic

### Puzzle Types
- [X] Classic
- [X] Arrow
- [X] Thermo
- [X] NonConsecutive
- [X] Chess
- [X] Sandwich
- [X] Miracle


### General
- [X] Add Doc
- [ ] Improve Arrow Sudoku performance
- [ ] Improve Sandwich Sudoku performance