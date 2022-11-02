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

    fn is_valid(&self, grid: &[[usize; GRID_SIZE]; GRID_SIZE], _number: usize, _row: usize, _col: usize) -> bool {
        'outer: for arrow in self.arrows.iter() {
            if grid[arrow.node.0][arrow.node.1] == 0 {
                continue;
            }

            let mut sum = 0;
            for arm_cell in arrow.arm.iter() {
                if grid[arm_cell.0][arm_cell.1] == 0 {
                    continue 'outer;
                } else {
                    sum += grid[arm_cell.0][arm_cell.1];
                }
            }

            let node_value = grid[arrow.node.0][arrow.node.1];
            if node_value != sum {
                return false;
            }
        }

        return true;
    }

}
