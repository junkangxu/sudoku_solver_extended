mod constraints;
mod solver;

use constraints::arrow_constraint::{Arrow, ArrowConstraint};
use constraints::constraint::Constraint;

use crate::constraints::classic_constraint::ClassicConstraint;
use crate::solver::Solver;

pub const GRID_SIZE: usize = 9;

fn main() {
    // Construct the initial grid and print it to console. TODO: Should read from source.
    let mut grid = [[0usize; GRID_SIZE]; GRID_SIZE];
    grid[0][1] = 8;
    grid[0][7] = 7;
    grid[1][4] = 3;
    grid[1][8] = 4;
    grid[2][2] = 5;
    grid[2][4] = 8;
    grid[3][4] = 7;
    grid[4][1] = 2;
    grid[4][2] = 6;
    grid[4][3] = 5;
    grid[4][5] = 9;
    grid[4][7] = 3;
    grid[6][6] = 8;
    grid[7][0] = 6;
    grid[7][4] = 4;
    grid[7][8] = 5;
    grid[8][7] = 4;

    println!("Puzzle:");
    print_board(&grid);

    // Construct Classic constraint. This is almost required for all situations.
    let classic_constraint = ClassicConstraint{};

    // Construct Arrow constraint. TODO: Should read from source.
    let arrow_one = Arrow {
        arm: vec!((2,0), (3,0), (4,0)),
        node: (1,0)
    };

    let arrow_two = Arrow {
        arm: vec!((0,3), (0,4), (0,5)),
        node: (1,3)
    };

    let arrow_three = Arrow {
        arm: vec!((1,6), (2,6), (3,6)),
        node: (4,6)
    };

    let arrow_four = Arrow {
        arm: vec!((4,4), (5,4), (6,4), (6,3)),
        node: (6,2)
    };

    let arrow_five = Arrow {
        arm: vec!((8,0), (8,1), (8,2)),
        node: (8,3)
    };

    let arrow_six = Arrow {
        arm: vec!((6,8), (7,7), (8,6)),
        node: (6,7)
    };
    
    let arrow_constraint = ArrowConstraint{
        arrows: vec!(arrow_one, arrow_two, arrow_three, arrow_four, arrow_five, arrow_six)
    };

    // Construct Constraints
    let mut constraints = Vec::new();
    constraints.push(Box::new(classic_constraint) as Box<dyn Constraint>);
    constraints.push(Box::new(arrow_constraint) as Box<dyn Constraint>);
    
    // Construct Solver
    let solver = Solver::new(&constraints);

    // Solve
    solver.solve(&mut grid);

    // Print out result
    println!("\nSolution:");
    print_board(&grid);
}

fn print_board(grid: &[[usize; GRID_SIZE]; GRID_SIZE]) {
    println!("{}", "-".repeat(19));

    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            if col == 0 {
                print!("|{} ", grid[row][col]);
            } else if (col + 1) % 3 == 0 {
                print!("{}|", grid[row][col]);
            } else {
                print!("{} ", grid[row][col]);
            }
        }

        println!("");
        if (row + 1) % 3 == 0 {
            println!("{}", "-".repeat(19));
        }
    }
}
