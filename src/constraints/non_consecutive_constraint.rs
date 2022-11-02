use crate::GRID_SIZE;

use super::constraint::Constraint;

#[derive(Clone)]
pub struct NonConsecutiveConstraint;

impl NonConsecutiveConstraint {

    fn is_adjacent_cell_consecutive(
        &self, grid: &[[usize; GRID_SIZE]; GRID_SIZE], number: usize, row: usize, col: usize
    ) -> bool {
        let adjacent_cells = self.get_adjacent_cells(row, col);
        let invalid_values = self.get_invalid_values(number);
        for adjacent_cell in adjacent_cells.iter() {
            for invalid_value in invalid_values.iter() {
                if grid[adjacent_cell.0][adjacent_cell.1] == *invalid_value {
                    return true;
                }
            }
        }

        return false;
    }

    fn get_adjacent_cells(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut adjacent_cells = Vec::new();
        // up
        if row != 0 {
            adjacent_cells.push((row - 1, col));
        }

        // down
        if row != 8 {
            adjacent_cells.push((row + 1, col));
        }

        // left
        if col != 0 {
            adjacent_cells.push((row, col - 1));
        }

        // right
        if col != 8 {
            adjacent_cells.push((row, col + 1));
        }

        return adjacent_cells;
    }

    fn get_invalid_values(&self, number: usize) -> Vec<usize> {
        let mut invalid_values = Vec::new();
        if number != 9 {
            invalid_values.push(number + 1);
        }

        if number != 1 {
            invalid_values.push(number - 1);
        }

        return invalid_values;
    }

}

impl Constraint for NonConsecutiveConstraint {
    fn is_valid(&self, grid: &[[usize; GRID_SIZE]; GRID_SIZE], number: usize, row: usize, col: usize) -> bool {
        return !self.is_adjacent_cell_consecutive(grid, number, row, col);
    }
}

