mod constraints;
mod solver;
mod reader;

use std::time::Instant;

use clap::{Parser, ValueEnum};
use constraints::constraint::Constraint;
use reader::Reader;

use crate::constraints::classic_constraint::ClassicConstraint;
use crate::solver::Solver;

pub const GRID_SIZE: usize = 9;

#[derive(Clone, Debug, ValueEnum)]
enum SudokuType {
    Classic,
    Arrow,
    Thermo
}

#[derive(Parser, Debug)]
#[command(author , about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    sudoku_type: String
}

fn parse_sudoku_types(sudoku_type_str: &str) -> Vec<SudokuType> {
    if sudoku_type_str.is_empty() {
        return vec![SudokuType::Classic]
    }

    return sudoku_type_str.split(',').map(|x| x.trim()).map(|x| match x {
        "classic" | "Classic" => SudokuType::Classic,
        "arrow" | "Arrow" => SudokuType::Arrow,
        "thermo" | "Thermo" => SudokuType::Thermo,
        _ => panic!("Not supported sudoku type: {:?}", x)
    }).collect();
}

fn main() {

    let args = Args::parse();
    let input = args.input;
    let sudoku_types = parse_sudoku_types(&args.sudoku_type);

    let reader = Reader{};
    let read_result = reader.read(&input);
    let mut grid = read_result.get_puzzle();
    println!("Puzzle:");
    print_board(&grid);

    let mut constraints = Vec::new();
    for sudoku_type in sudoku_types.iter() {
        match sudoku_type {
            SudokuType::Classic => constraints.push(Box::new(ClassicConstraint{}) as Box<dyn Constraint>),
            SudokuType::Arrow => constraints.push(Box::new(read_result.get_arrow_constraint().unwrap()) as Box<dyn Constraint>),
            SudokuType::Thermo => constraints.push(Box::new(read_result.get_thermo_constraint().unwrap()) as Box<dyn Constraint>)
        }
    }
    
    let now = Instant::now();

    let solver = Solver::new(&constraints);
    let solved = solver.solve(&mut grid);
    if solved {
        println!("\nSolution:");
        print_board(&grid);
    } else {
        println!("\nThis sudoku has no solution.")
    }

    println!("Spend: {} seconds", now.elapsed().as_secs());
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
