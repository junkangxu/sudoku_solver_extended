use crate::constraints::constraint::Constraint;
use crate::GRID_SIZE;

pub struct ClassicConstraint;

impl ClassicConstraint {
    
    fn is_number_in_row(&self, grid: &[[usize; GRID_SIZE]; GRID_SIZE], number: usize, row: usize) -> bool {
        for col in 0..GRID_SIZE {
            if grid[row][col] == number {
                return true;
            }
        }

        return false;
    }

    fn is_number_in_col(&self, grid: &[[usize; GRID_SIZE]; GRID_SIZE], number: usize, col: usize) -> bool {
        for row in 0..GRID_SIZE {
            if grid[row][col] == number {
                return true;
            }
        }

        return false;
    }

    fn is_number_in_box(&self, grid: &[[usize; GRID_SIZE]; GRID_SIZE], number: usize, row: usize, col: usize) -> bool {
        let local_row = row - row % 3;
        let local_col = col - col % 3;
        for temp_row in local_row..local_row + 3 {
            for temp_col in local_col..local_col + 3 {
                if grid[temp_row][temp_col] == number {
                    return true;
                }
            }
        }

        return false;
    }

}

impl Constraint for ClassicConstraint {
    fn is_valid(&self, grid: &[[usize; GRID_SIZE]; GRID_SIZE], number: usize, row: usize, col: usize) -> bool {
        return !self.is_number_in_box(grid, number, row, col) 
            && !self.is_number_in_col(grid, number, col) 
            && !self.is_number_in_row(grid, number, row)
    }
}