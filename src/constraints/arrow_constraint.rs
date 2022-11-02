use crate::GRID_SIZE;
use crate::constraints::constraint::Constraint;

#[derive(Clone)]
pub struct Arrow {
    pub arm: Vec<(usize, usize)>,
    pub node: (usize, usize)
}

#[derive(Clone)]
pub struct ArrowConstraint {
    pub arrows: Vec<Arrow>
}

impl Constraint for ArrowConstraint {

    fn is_valid(&self, grid: &[[usize; GRID_SIZE]; GRID_SIZE], number: usize, row: usize, col: usize) -> bool {
        let mut temp_grid = grid.clone();
        temp_grid[row][col] = number;

        'outer: for arrow in self.arrows.iter() {
            if !arrow.arm.contains(&(row, col)) && !arrow.node.eq(&(row, col)) {
                continue;
            }

            if temp_grid[arrow.node.0][arrow.node.1] == 0 {
                continue;
            }

            let mut sum = 0;
            for arm_cell in arrow.arm.iter() {
                if temp_grid[arm_cell.0][arm_cell.1] == 0 {
                    continue 'outer;
                } else {
                    sum += temp_grid[arm_cell.0][arm_cell.1];
                }
            }

            let node_value = temp_grid[arrow.node.0][arrow.node.1];
            if node_value != sum {
                return false;
            }
        }

        return true;
    }

}
