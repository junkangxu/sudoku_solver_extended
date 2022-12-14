use std::{fs::File, io::{BufReader, BufRead}};

use crate::{GRID_SIZE, constraints::{arrow_constraint::{ArrowConstraint, Arrow}, thermo_constraint::ThermoConstraint, sandwich_constraint::SandwichConstraint}};

/// Generic struct for the ReadResult from input file.
pub struct ReadResult {
    puzzle: [[usize; GRID_SIZE]; GRID_SIZE],
    arrow_constraint: Option<ArrowConstraint>,
    thermo_constraint: Option<ThermoConstraint>,
    sandwich_constraint: Option<SandwichConstraint>
}

impl ReadResult {
    /// Puzzle
    pub fn get_puzzle(&self) -> [[usize; GRID_SIZE]; GRID_SIZE] {
        return self.puzzle;
    }

    /// Arrow constraints
    pub fn get_arrow_constraint(&self) -> Option<ArrowConstraint> {
        return self.arrow_constraint.clone();
    }

    /// Thermo constraints
    pub fn get_thermo_constraint(&self) -> Option<ThermoConstraint> {
        return self.thermo_constraint.clone();
    }

    /// Sandwich constraints
    pub fn get_sandwich_constraint(&self) -> Option<SandwichConstraint> {
        return self.sandwich_constraint.clone();
    }
}

pub struct Reader;

impl Reader {

    /// Read from input file and parse
    /// - (Required) Puzzle
    /// - (Optional) Arrow Constraints
    /// - (Optional) Thermo Constraints
    /// - (Optional) Sandwich Constraints
    pub fn read(&self, path: &str) -> ReadResult {
        let file = convert_file_to_vector(&File::open(path).unwrap());
        return ReadResult { 
            puzzle: self.read_puzzle(&file),
            arrow_constraint: self.read_arrow_constraint(&file),
            thermo_constraint: self.read_thermo_constraint(&file),
            sandwich_constraint: self.read_sandwich_constraint(&file)
        }
    }

    /// Read Sandwich constraints
    fn read_sandwich_constraint(&self, file: &Vec<String>) -> Option<SandwichConstraint> {
        let optional_starting_index = get_starting_index(file, "[SandwichConstraint]");
        let starting_index = match optional_starting_index {
            Some(x) => x,
            None => return None
        };

        let row_sum_str = file.get(starting_index).unwrap().trim();
        let col_sum_str = file.get(starting_index + 1).unwrap().trim();

        let row_sum = row_sum_str[1..row_sum_str.len() - 1].split(",").map(|x| x.trim().parse::<usize>().unwrap()).collect();
        let col_sum = col_sum_str[1..col_sum_str.len() - 1].split(",").map(|x| x.trim().parse::<usize>().unwrap()).collect();

        return Some(SandwichConstraint { row_sum, col_sum });
    }

    /// Read Thermo constraints
    fn read_thermo_constraint(&self, file: &Vec<String>) -> Option<ThermoConstraint> {
        let optional_starting_index = get_starting_index(file, "[ThermoConstraint]");
        let starting_index = match optional_starting_index {
            Some(x) => x,
            None => return None
        };

        let mut arrows: Vec<Vec<(usize, usize)>> = Vec::new();
        for i in starting_index..file.len() {
            let mut arrow: Vec<(usize, usize)> = Vec::new();
            let line = file.get(i).unwrap().trim();
            if line.is_empty() {
                break;
            }

            let cells: Vec<&str> = line[1..line.len() - 1].split(", ").map(|x| x.trim()).collect();
            for cell in cells.iter() {
                let cell_tuple: Vec<&str> = cell[1..cell.len() - 1].split(',').collect();
                arrow.push((cell_tuple[0].parse::<usize>().unwrap() - 1, cell_tuple[1].parse::<usize>().unwrap() - 1));
            }

            arrows.push(arrow);
        }

        return Some(ThermoConstraint { arrows });
    }

    /// Read Arrow constraints
    fn read_arrow_constraint(&self, file: &Vec<String>) -> Option<ArrowConstraint> {
        let optional_starting_index = get_starting_index(file, "[ArrowConstraint]");
        let starting_index = match optional_starting_index {
            Some(x) => x,
            None => return None
        };

        let mut arrows: Vec<Arrow> = Vec::new();
        for i in starting_index..file.len() {
            let line = file.get(i).unwrap().trim();
            if line.is_empty() {
                break;
            }

            let line_splited_by_arrow: Vec<&str> = line.split(" => ").collect();
            assert!(line_splited_by_arrow.len() == 2);

            let arm_str = line_splited_by_arrow[0];
            let node_str = line_splited_by_arrow[1];

            let mut arm: Vec<(usize, usize)> = Vec::new();
            let arm_vec: Vec<&str> = arm_str[1..arm_str.len() - 1].split(", ").collect();
            for arm_cell_str in arm_vec {
                let arm_cell_tuple: Vec<&str> = arm_cell_str[1..node_str.len() - 1].split(',').collect();
                let cell = (arm_cell_tuple[0].parse::<usize>().unwrap() - 1, arm_cell_tuple[1].parse::<usize>().unwrap() - 1);
                arm.push(cell);
            }

            let node_tuple: Vec<&str> = node_str[1..node_str.len() - 1].split(',').collect();
            assert!(node_tuple.len() == 2);
            let node = (node_tuple[0].parse::<usize>().unwrap() - 1, node_tuple[1].parse::<usize>().unwrap() - 1);

            let arrow = Arrow {arm, node};
            arrows.push(arrow);
        }

        return Some(ArrowConstraint { arrows });
    }

    /// Read Puzzle
    fn read_puzzle(&self, file: &Vec<String>) -> [[usize; GRID_SIZE]; GRID_SIZE] {
        let starting_index = get_starting_index(file, "[Puzzle]").unwrap();
        let mut grid: Vec<Vec<usize>> = Vec::new();
        for i in starting_index..starting_index + 9 {
            let line = file.get(i).unwrap();
            let splited_line: Vec<&str> = line.split(' ').collect();
            let mut row: Vec<usize> = Vec::new();
            for word in splited_line.iter() {
                let number = word.parse::<usize>().unwrap();
                row.push(number);
            }

            grid.push(row);
        }

        return convert_vector_to_array(&grid);
    }



}

/// Get the starting index of the input file by keyword in format [<keyword>], for examples [Puzzle].
fn get_starting_index(file: &Vec<String>, text: &str) -> Option<usize> {
    for (line_num, line) in file.iter().enumerate() {
        if line.contains(text) {
            return Some(line_num + 1);
        }
    }

    None
}

/// convert file to a vector of String
fn convert_file_to_vector(file: &File) -> Vec<String> {
    let reader = BufReader::new(file);
    let mut result: Vec<String> = Vec::new();
    for line in reader.lines() {
        let unwrapped_line = line.unwrap();
        if unwrapped_line.is_empty() {
            continue;
        }

        result.push(unwrapped_line);
    }

    return result;
}

/// Generic method to convert Vector to Array
fn convert_vector_to_array<T>(vectors: &Vec<Vec<T>>) -> [[T; GRID_SIZE]; GRID_SIZE] where T: Default + Copy {
    assert!(vectors.len() == GRID_SIZE);
    assert!(vectors.get(0).unwrap().len() == GRID_SIZE);

    let mut result: [[T; GRID_SIZE]; GRID_SIZE] = [[T::default(); GRID_SIZE]; GRID_SIZE];

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            result[i][j] = vectors[i][j];
        }
    }

    return result;
}
