use crate::GRID_SIZE;

pub trait Constraint {
    fn is_valid(&self, grid: &[[usize; GRID_SIZE]; GRID_SIZE], number: usize, row: usize, col: usize) -> bool;
}
