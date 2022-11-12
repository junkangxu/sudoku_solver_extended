use crate::constraints::constraint::Constraint;
use crate::GRID_SIZE;

pub struct ChessKnightConstraint;

impl ChessKnightConstraint {
    fn is_number_in_knight_move(
        &self, 
        grid: &[[usize; GRID_SIZE]; GRID_SIZE], 
        number: usize, 
        row: usize, 
        col: usize
    ) -> bool {
        let mut knight_moves = Vec::new();

        // Upper Right
        if row > 0 && col < 7 {
            knight_moves.push((row - 1, col + 2));
        }

        if row > 1 && col < 8 {
            knight_moves.push((row - 2, col + 1));
        }

        // Upper Left
        if row > 0 && col > 1 {
            knight_moves.push((row - 1, col - 2));
        }

        if row > 1 && col > 0 {
            knight_moves.push((row - 2, col - 1));
        }

        // Lower Right
        if row < 7 && col < 8 {
            knight_moves.push((row + 2, col + 1));
        }

        if row < 8 && col < 7 {
            knight_moves.push((row + 1, col + 2));
        }

        // Lower Left
        if row < 7 && col > 0 {
            knight_moves.push((row + 2, col - 1));
        }
        
        if row < 8 && col > 1 {
            knight_moves.push((row + 1, col - 2));
        }
        
        return knight_moves.into_iter().any(|knight_move| grid[knight_move.0][knight_move.1] == number);
    }
}

impl Constraint for ChessKnightConstraint {
    fn is_valid(&self, grid: &[[usize; GRID_SIZE]; GRID_SIZE], number: usize, row: usize, col: usize) -> bool {
        return !self.is_number_in_knight_move(grid, number, row, col);
    }
}