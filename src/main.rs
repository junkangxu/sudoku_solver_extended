mod constraints;
mod solver;
mod reader;

use std::time::Instant;

use clap::{Parser, ValueEnum};
use constraints::constraint::Constraint;
use reader::Reader;

use crate::constraints::chess_king_constraint::ChessKingConstraint;
use crate::constraints::chess_knight_constraint::ChessKnightConstraint;
use crate::constraints::chess_queen_constraint::ChessQueenConstraint;
use crate::constraints::classic_constraint::ClassicConstraint;
use crate::constraints::non_consecutive_constraint::NonConsecutiveConstraint;
use crate::solver::Solver;

pub const GRID_SIZE: usize = 9;

#[derive(Clone, Debug, ValueEnum)]
enum SudokuType {
    Classic,
    Arrow,
    Thermo,
    NonConsecutive,
    ChessKnight,
    ChessKing,
    ChessQueen,
    Sandwich,
    Miracle
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
        "nonConsecutive" | "NonConsecutive" => SudokuType::NonConsecutive,
        "chessKnight" | "ChessKnight" => SudokuType::ChessKnight,
        "chessKing" | "ChessKing" => SudokuType::ChessKing,
        "chessQueen" | "ChessQueen" => SudokuType::ChessQueen,
        "sandwich" | "Sandwich" => SudokuType::Sandwich,
        "miracle" | "Miracle" => SudokuType::Miracle,
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
            SudokuType::NonConsecutive => constraints.push(Box::new(NonConsecutiveConstraint{}) as Box<dyn Constraint>),
            SudokuType::Arrow => constraints.push(Box::new(read_result.get_arrow_constraint().unwrap()) as Box<dyn Constraint>),
            SudokuType::Thermo => constraints.push(Box::new(read_result.get_thermo_constraint().unwrap()) as Box<dyn Constraint>),
            SudokuType::ChessKnight => constraints.push(Box::new(ChessKnightConstraint{}) as Box<dyn Constraint>),
            SudokuType::ChessKing => constraints.push(Box::new(ChessKingConstraint{}) as Box<dyn Constraint>),
            SudokuType::ChessQueen => constraints.push(Box::new(ChessQueenConstraint{}) as Box<dyn Constraint>),
            SudokuType::Sandwich => constraints.push(Box::new(read_result.get_sandwich_constraint().unwrap()) as Box<dyn Constraint>),
            SudokuType::Miracle => {
                constraints.push(Box::new(NonConsecutiveConstraint{}) as Box<dyn Constraint>);
                constraints.push(Box::new(ChessKnightConstraint{}) as Box<dyn Constraint>);
                constraints.push(Box::new(ChessKingConstraint{}) as Box<dyn Constraint>);
            }
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

    println!("Spend: {} millis", now.elapsed().as_millis());
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
