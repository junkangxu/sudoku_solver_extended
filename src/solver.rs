use crate::GRID_SIZE;
use crate::constraints::constraint::Constraint;

pub struct Solver<'a> {
    constraints: &'a Vec<Box<dyn Constraint>>
}

impl Solver<'_> {

    pub fn new(constraints: &Vec<Box<dyn Constraint>>) -> Solver {
        Solver {constraints}
    }

    pub fn solve(&self, grid: &mut[[usize; GRID_SIZE]; GRID_SIZE]) -> bool {
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if grid[row][col] == 0 {
                    for number in 1..=9 {
                        let mut is_valid = true;
                        for constraint in self.constraints.iter() {
                            is_valid = is_valid && constraint.is_valid(&*grid, number, row, col)
                        }

                        if is_valid {
                            grid[row][col] = number;

                            if self.solve(grid) {
                                return true;
                            } else {
                                grid[row][col] = 0;
                            }
                        }
                    }

                    return false;
                }
            }
        }

        return true;
    }
}

