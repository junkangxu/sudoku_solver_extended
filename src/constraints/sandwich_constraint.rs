use crate::constraints::constraint::Constraint;
use crate::GRID_SIZE;

#[derive(Clone)]
pub struct SandwichConstraint {
    pub row_sum: Vec<usize>,
    pub col_sum: Vec<usize>
}

impl SandwichConstraint {
    fn is_row_sum_mismatched(&self, grid: &[[usize; GRID_SIZE]; GRID_SIZE], row: usize) -> bool {
        let mut index_of_one: Option<usize> = None;
        let mut index_of_nine: Option<usize> = None;
        for i in 0..GRID_SIZE {
            if grid[row][i] == 1 {
                index_of_one = Some(i);
            } else if grid[row][i] == 9 {
                index_of_nine = Some(i);
            }
        }

        if index_of_one == None || index_of_nine == None {
            return false;
        }

        let mut row_sum = 0;
        let starting_index = std::cmp::min(index_of_one.unwrap(), index_of_nine.unwrap());
        let ending_index = std::cmp::max(index_of_one.unwrap(), index_of_nine.unwrap());

        for i in starting_index + 1..ending_index {
            if grid[row][i] == 0 {
                return false;
            }

            row_sum += grid[row][i];
        }

        return row_sum != self.row_sum[row];
    }

    fn is_col_sum_mismatched(&self, grid: &[[usize; GRID_SIZE]; GRID_SIZE], col: usize) -> bool {
        let mut index_of_one: Option<usize> = None;
        let mut index_of_nine: Option<usize> = None;
        for i in 0..GRID_SIZE {
            if grid[i][col] == 1 {
                index_of_one = Some(i);
            } else if grid[i][col] == 9 {
                index_of_nine = Some(i);
            }
        }

        if index_of_one == None || index_of_nine == None {
            return false;
        }

        let mut col_sum = 0;
        let starting_index = std::cmp::min(index_of_one.unwrap(), index_of_nine.unwrap());
        let ending_index = std::cmp::max(index_of_one.unwrap(), index_of_nine.unwrap());

        for i in starting_index + 1..ending_index {
            if grid[i][col] == 0 {
                return false;
            }

            col_sum += grid[i][col];
        }

        return col_sum != self.col_sum[col];
    }

}

impl Constraint for SandwichConstraint {

    fn is_valid(&self, grid: &[[usize; GRID_SIZE]; GRID_SIZE], _number: usize, row: usize, col: usize) -> bool {
        return !self.is_row_sum_mismatched(grid, row) && !self.is_col_sum_mismatched(grid, col);
    }
}