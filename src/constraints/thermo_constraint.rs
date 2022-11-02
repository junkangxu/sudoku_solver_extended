use crate::GRID_SIZE;

use super::constraint::Constraint;

#[derive(Clone)]
pub struct ThermoConstraint {
    pub arrows: Vec<Vec<(usize, usize)>>
}

impl Constraint for ThermoConstraint {

    fn is_valid(&self, grid: &[[usize; GRID_SIZE]; GRID_SIZE], number: usize, row: usize, col: usize) -> bool {
        let mut temp_grid = grid.clone();
        temp_grid[row][col] = number;

        for line in self.arrows.iter() {
            let mut prev_value = 0;
            for cell in line.iter() {
                let curr_value = temp_grid[cell.0][cell.1];
                if curr_value == 0 {
                    continue;
                }
                if prev_value >= curr_value {
                    return false;
                }

                prev_value = curr_value;
            }
        }

        return true;
    }

}
