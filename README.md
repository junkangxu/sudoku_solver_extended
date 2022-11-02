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
- [ ] Killer
- [ ] Chess
- [ ] Sandwich
- [ ] Miracle

### General
- [ ] Add unit tests
- [ ] Add Doc
- [ ] Improve Arrow Sudoku performance