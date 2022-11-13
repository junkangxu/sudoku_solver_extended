use crate::GRID_SIZE;

/// A core component has a `is_valid` function, which contains the logic to determine
/// if the current puzzle satisfies the Sudoku rule or not.
pub trait Constraint {
    fn is_valid(&self, grid: &[[usize; GRID_SIZE]; GRID_SIZE], number: usize, row: usize, col: usize) -> bool;
}
