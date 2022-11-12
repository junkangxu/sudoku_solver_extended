use crate::constraints::constraint::Constraint;
use crate::GRID_SIZE;

pub struct ChessQueenConstraint;

impl ChessQueenConstraint {
    fn is_number_in_queen_move(
        &self,
        grid: &[[usize; GRID_SIZE]; GRID_SIZE],
        number: usize,
        row: usize,
        col: usize
    ) -> bool {
        if number != 9 {
            return false;
        }

        let mut queen_moves = Vec::new();

        if row == 0 {
            if col == 0 {
                // only lower right
                queen_moves.extend(self.get_lower_right_moves(row, col));
            } else if col == 8 {
                // only lower left
                queen_moves.extend(self.get_lower_left_moves(row, col));
            } else {
                // Lower right and lower left
                queen_moves.extend(self.get_lower_right_moves(row, col));
                queen_moves.extend(self.get_lower_left_moves(row, col));
            }
        } else if row == 8 {
            if col == 0 {
                // only upper right
                queen_moves.extend(self.get_upper_right_moves(row, col));
            } else if col == 8 {
                // only upper left
                queen_moves.extend(self.get_upper_left_moves(row, col));
            } else {
                // upper right an upper left
                queen_moves.extend(self.get_upper_right_moves(row, col));
                queen_moves.extend(self.get_upper_left_moves(row, col));
            }
        } else if col == 0 {
            // Only upper right and lower right
            queen_moves.extend(self.get_upper_right_moves(row, col));
            queen_moves.extend(self.get_lower_right_moves(row, col));
        } else if col == 8 {
            queen_moves.extend(self.get_upper_left_moves(row, col));
            queen_moves.extend(self.get_lower_left_moves(row, col));
        } else {
            // All of upper right and left and lower right and left
            queen_moves.extend(self.get_upper_right_moves(row, col));
            queen_moves.extend(self.get_upper_left_moves(row, col));
            queen_moves.extend(self.get_lower_right_moves(row, col));
            queen_moves.extend(self.get_lower_left_moves(row, col));
        }

        return queen_moves.into_iter().any(|queen_move| grid[queen_move.0][queen_move.1] == number);
    }

    fn get_lower_right_moves(&self, row: usize, col: usize) -> Vec<(usize, usize)>{
        let mut moves = Vec::new();
        for i in 1..GRID_SIZE {
            let new_row = row + i;
            let new_col = col + i;

            moves.push((new_row, new_col));
            if new_row <= 0 || new_row >= 8 || new_col <= 0 || new_col >= 8 {
                return moves;
            }
        }

        return moves;
    }

    fn get_lower_left_moves(&self, row: usize, col: usize) -> Vec<(usize, usize)>{
        let mut moves = Vec::new();

        for i in 1..GRID_SIZE {
            let new_row = row + i;
            let new_col = col - i;

            moves.push((new_row, new_col));
            if new_row <= 0 || new_row >= 8 || new_col <= 0 || new_col >= 8 {
                return moves;
            }
        }

        return moves;
    }

    fn get_upper_right_moves(&self, row: usize, col: usize) -> Vec<(usize, usize)>{
        let mut moves = Vec::new();

        for i in 1..GRID_SIZE {
            let new_row = row - i;
            let new_col = col + i;

            moves.push((new_row, new_col));
            if new_row <= 0 || new_row >= 8 || new_col <= 0 || new_col >= 8 {
                return moves;
            }
        }

        return moves;
    }

    fn get_upper_left_moves(&self, row: usize, col: usize) -> Vec<(usize, usize)>{
        let mut moves = Vec::new();
        for i in 1..GRID_SIZE {
            let new_row = row - i;
            let new_col = col - i;

            moves.push((new_row, new_col));
            if new_row <= 0 || new_row >= 8 || new_col <= 0 || new_col >= 8 {
                return moves;
            }
        }

        return moves;
    }
}

impl Constraint for ChessQueenConstraint {
    fn is_valid(&self, grid: &[[usize; GRID_SIZE]; GRID_SIZE], number: usize, row: usize, col: usize) -> bool {
        return !self.is_number_in_queen_move(grid, number, row, col);
    }
}