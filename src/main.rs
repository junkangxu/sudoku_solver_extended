mod constraints;
mod solver;
mod reader;

use constraints::constraint::Constraint;
use reader::Reader;

use crate::constraints::classic_constraint::ClassicConstraint;
use crate::solver::Solver;

pub const GRID_SIZE: usize = 9;

fn main() {

    let reader = Reader{};
    let read_result = reader.read("./inputs/arrow_sudoku.txt");
    let mut grid = read_result.get_puzzle();
    let arrow_constraint = read_result.get_arrow_constraint();
    
    println!("Puzzle:");
    print_board(&grid);

    // Construct Classic constraint. This is almost required for all situations.
    let classic_constraint = ClassicConstraint{};

    // Construct Constraints
    let mut constraints = Vec::new();
    constraints.push(Box::new(classic_constraint) as Box<dyn Constraint>);
    constraints.push(Box::new(arrow_constraint) as Box<dyn Constraint>);
    
    // Construct Solver
    let solver = Solver::new(&constraints);

    // Solve
    let solved = solver.solve(&mut grid);
    if solved {
        println!("\nSolution:");
        print_board(&grid);
    } else {
        println!("\nThis sudoku has no solution.")
    }
    
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
