use crate::constraints::constraint::Constraint;
use crate::GRID_SIZE;

pub struct ChessKingConstraint;

impl ChessKingConstraint {
    fn is_number_in_king_move(
        &self, grid: &[[usize; GRID_SIZE]; GRID_SIZE], number: usize, row: usize, col: usize
    ) -> bool {
        let mut king_moves = Vec::new();

        if row > 0 {
            king_moves.push((row - 1, col));

            if col < 8 {
                king_moves.push((row - 1, col + 1));
            }
            
            if col > 0 {
                king_moves.push((row - 1, col - 1));
            }
        }

        if row < 8 {
            king_moves.push((row + 1, col));

            if col < 8 {
                king_moves.push((row + 1, col + 1));
            }

            if col > 0 {
                king_moves.push((row + 1, col - 1));
            }
        }

        if col > 0 {
            king_moves.push((row, col - 1));
        }

        if col < 8 {
            king_moves.push((row, col + 1));
        }

        return king_moves.into_iter().any(|king_move| grid[king_move.0][king_move.1] == number);
    }
}

impl Constraint for ChessKingConstraint {
    fn is_valid(&self, grid: &[[usize; GRID_SIZE]; GRID_SIZE], number: usize, row: usize, col: usize) -> bool {
        return !self.is_number_in_king_move(grid, number, row, col);
    }
}