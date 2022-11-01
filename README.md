# sudoku_solver_extended
## Description
a sudoku solver for one or more variants of Sudoku puzzles. The puzzles and constraints are all currently hardcoded in the code. Future plan in the Progress Section.

## Example
Arrow Sudoku Example
```bash
Puzzle:
-------------------
|0 8 0|0 0 0|0 7 0|
|0 0 0|0 3 0|0 0 4|
|0 0 5|0 8 0|0 0 0|
-------------------
|0 0 0|0 7 0|0 0 0|
|0 2 6|5 0 9|0 3 0|
|0 0 0|0 0 0|0 0 0|
-------------------
|0 0 0|0 0 0|8 0 0|
|6 0 0|0 4 0|0 0 5|
|0 0 0|0 0 0|0 4 0|
-------------------

Constraints:
((3,1), (4,1), (5,1)) => (2,1),
((1,4), (1,5), (1,6)) => (2,4),
((2,7), (3,7), (4,7)) => (5,7),
((5,5), (6,5), (7,5), (7,4)) => (7,3),
((9,1), (9,2), (9,3)) => (9,4),
((7,9), (8,8), (9,7)) => (7,8)

Solution:
-------------------
|9 8 4|2 6 1|5 7 3|
|7 6 1|9 3 5|2 8 4|
|2 3 5|7 8 4|1 9 6|
-------------------
|1 5 3|6 7 8|4 2 9|
|4 2 6|5 1 9|7 3 8|
|8 9 7|4 2 3|6 5 1|
-------------------
|3 4 9|1 5 7|8 6 2|
|6 7 8|3 4 2|9 1 5|
|5 1 2|8 9 6|3 4 7|
-------------------
```

## Progress

### Features
- [ ] Adding Clap integration
- [ ] Adding required parameter `--input`
- [ ] Adding optional parameter `--output`. Default value: None, print to standard out.
- [ ] Adding optional parameter `--types`. Default value: Classic

### Puzzle Types
- [X] Classic
- [X] Arrow
- [ ] Killer
- [ ] Chess
- [ ] Sandwich
- [ ] Thermo
- [ ] Miracle

## Development
```bash
cargo run
```