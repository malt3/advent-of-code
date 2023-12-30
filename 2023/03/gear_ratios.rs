use std::io::{self, BufRead};

const MATRIX_SIZE: usize = 140;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let matrix = read_matrix(lines)?;

    let sum_part_1 = find_all_numbers(&matrix);
    let sum_part_2 = find_gears(&matrix);

    println!("sum of part numbers: {}", sum_part_1);
    println!("sum of gear ratios: {}", sum_part_2);

    Ok(())
}

fn read_matrix(lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<[[char; MATRIX_SIZE]; MATRIX_SIZE], Box<dyn std::error::Error>> {
    let mut matrix: [[char; MATRIX_SIZE]; MATRIX_SIZE] = [['\0'; MATRIX_SIZE]; MATRIX_SIZE];
    for (i, line) in lines.enumerate() {
        let line = line?;
        let mut j = 0;
        for c in line.chars() {
            matrix[i][j] = c;
            j += 1;
        }
    }
    Ok(matrix)
}

fn find_gears(matrix: &[[char; MATRIX_SIZE]; MATRIX_SIZE]) -> u32 {
    let mut sum: u32 = 0;
    let (mut i, mut j) = (0, 0);
    while let Some((next_i, next_j, gear_ratio)) = find_next_gear(matrix, i, j) {
        i = next_i;
        j = next_j;
        sum += gear_ratio;
        i = if j == (MATRIX_SIZE - 1) { i + 1 } else { i };
        j = (j + 1) % MATRIX_SIZE;
    }
    sum
}

fn find_all_numbers(matrix: &[[char; MATRIX_SIZE]; MATRIX_SIZE]) -> u32 {
    let mut sum: u32 = 0;
    let (mut i, mut j) = (0, 0);
    while let Some((next_i, next_j)) = find_next_digit(matrix, i, j) {
        i = next_i;
        j = next_j;
        let j_start = j;
        let mut j_end = j;
        while j_end < (MATRIX_SIZE - 1) && matrix[i][j_end+1].is_digit(10) {
            j_end += 1;
        }
        if cell_range_has_symbol_neighbor(matrix, i, j_start, j_end) {
            sum += matrix[i][j_start..=j_end].iter().collect::<String>().parse::<u32>().unwrap();
        }
        i = if j_end == (MATRIX_SIZE - 1) { i + 1 } else { i };
        j = (j_end + 1) % MATRIX_SIZE;
    }
    sum
}

fn find_next_gear(matrix: &[[char; MATRIX_SIZE]; MATRIX_SIZE], mut i: usize, mut j: usize) -> Option<(usize, usize, u32)> {
    while i < MATRIX_SIZE && j < MATRIX_SIZE {
        if let Some(gear_ratio) = is_gear_with_ratio(matrix, i, j){
            return Some((i, j, gear_ratio));
        }
        j += 1;
        if j == MATRIX_SIZE {
            j = 0;
            i += 1;
        }
    }
    None
}

fn find_next_digit(matrix: &[[char; MATRIX_SIZE]; MATRIX_SIZE], mut i: usize, mut j: usize) -> Option<(usize, usize)> {
    while i < MATRIX_SIZE && j < MATRIX_SIZE {
        if matrix[i][j].is_digit(10) {
            return Some((i, j));
        }
        j += 1;
        if j == MATRIX_SIZE {
            j = 0;
            i += 1;
        }
    }
    None
}

fn is_gear_with_ratio(matrix: &[[char; MATRIX_SIZE]; MATRIX_SIZE], i: usize, j: usize) -> Option<u32> {
    /*
    Check any position with a c:

    ccc
    c*c
    ccc

    */

    if matrix[i][j] != '*' {
        return None;
    }

    let mut part_numbers = 0;
    let mut gear_ratio = 1;

    // check left of the gear
    if let Some(part_number) = connected_part_number_left(matrix, i, j) {
        part_numbers += 1;
        gear_ratio *= part_number;
    }

    // check right of the gear
    if let Some(part_number) = connected_part_number_right(matrix, i, j) {
        part_numbers += 1;
        gear_ratio *= part_number;
    }

    // check above the gear
    if i > 0 {
        match connected_part_numbers_horizontal(matrix, i-1, j) {
            (Some(left_part_number), Some(right_part_number)) => {
                part_numbers += 2;
                gear_ratio *= left_part_number;
                gear_ratio *= right_part_number;
            },
            (Some(part_number), None) => {
                part_numbers += 1;
                gear_ratio *= part_number;
            },
            (None, Some(part_number)) => {
                part_numbers += 1;
                gear_ratio *= part_number;
            },
            (None, None) => (),
        }
    }

    // check below the gear
    if i < (MATRIX_SIZE - 1) {
        match connected_part_numbers_horizontal(matrix, i+1, j) {
            (Some(left_part_number), Some(right_part_number)) => {
                part_numbers += 2;
                gear_ratio *= left_part_number;
                gear_ratio *= right_part_number;
            },
            (Some(part_number), None) => {
                part_numbers += 1;
                gear_ratio *= part_number;
            },
            (None, Some(part_number)) => {
                part_numbers += 1;
                gear_ratio *= part_number;
            },
            (None, None) => (),
        }
    }

    if part_numbers == 2 {
        Some(gear_ratio)
    } else {
        None
    }
}

fn connected_part_number_left(matrix: &[[char; MATRIX_SIZE]; MATRIX_SIZE], i: usize, j: usize) -> Option<u32> {
    if j == 0 || !matrix[i][j-1].is_digit(10) {
        return None;
    }
    let mut part_number = 0;
    let mut multiplier = 1;
    for k in (0..j).rev() {
        if matrix[i][k].is_digit(10) {
            part_number += matrix[i][k].to_digit(10).unwrap() * multiplier;
            multiplier *= 10;
        } else {
            break;
        }
    }
    Some(part_number)
}

fn connected_part_number_right(matrix: &[[char; MATRIX_SIZE]; MATRIX_SIZE], i: usize, j: usize) -> Option<u32> {
    if j == (MATRIX_SIZE - 1) || !matrix[i][j+1].is_digit(10) {
        return None;
    }
    let mut part_number = 0;
    for k in j+1..MATRIX_SIZE {
        if matrix[i][k].is_digit(10) {
            part_number = (part_number * 10) + matrix[i][k].to_digit(10).unwrap();
        } else {
            break;
        }
    }
    Some(part_number)
}

fn connected_part_numbers_horizontal(matrix: &[[char; MATRIX_SIZE]; MATRIX_SIZE], i: usize, j: usize) -> (Option<u32>, Option<u32>) {
    if i >= MATRIX_SIZE {
        return (None, None);
    }
    let left = if j == 0 { '.' } else { matrix[i][j-1] };
    let center = matrix[i][j];
    let right = if j == (MATRIX_SIZE - 1) { '.' } else { matrix[i][j+1] };
    let first_part_number_start: Option<usize> = match (left.is_digit(10), center.is_digit(10), right.is_digit(10)) {
        (true, _, _) => {
            let mut part_number_start = j-1;
            for k in (0..j).rev() {
                if matrix[i][k].is_digit(10) {
                    part_number_start = k;
                } else {
                    break;
                }
            }
            Some(part_number_start)
        },
        (_, true, _) => Some(j),
        (_, _, true) => Some(j+1),
        (false, false, false) => None,
    };
    let second_part_number_start: Option<usize> = if left.is_digit(10) && !center.is_digit(10) && right.is_digit(10) {
        Some(j+1)
    } else {
        None
    };
    return (
        if let Some(j) = first_part_number_start {parse_part_number(matrix, i, j)} else { None },
        if let Some(j) = second_part_number_start {parse_part_number(matrix, i, j)} else { None },
    );
}

fn parse_part_number(matrix: &[[char; MATRIX_SIZE]; MATRIX_SIZE], i: usize, j: usize) -> Option<u32> {
    let mut part_number = 0;
    for k in j..MATRIX_SIZE {
        if matrix[i][k].is_digit(10) {
            part_number = (part_number * 10) + matrix[i][k].to_digit(10).unwrap();
        } else {
            break;
        }
    }
    Some(part_number)
}

fn cell_range_has_symbol_neighbor(matrix: &[[char; MATRIX_SIZE]; MATRIX_SIZE], i: usize, j_start: usize, j_end: usize) -> bool {
    /*
    Check any position with a c:

    ccccccc
    c12345c
    ccccccc

    where j_left is the index of 1 and j_right is the index of 5.
    */

    // check left of the range
    if j_start > 0 && is_symbol(matrix[i][j_start-1]) {
        return true;
    }
    // check right of the range
    if j_end < (MATRIX_SIZE - 1) && is_symbol(matrix[i][j_end+1]) {
        return true;
    }

    // check above the range
    if i > 0 {
        let j_start = if j_start == 0 { 0 } else { j_start - 1 };
        let j_end = if j_end == (MATRIX_SIZE - 1) { MATRIX_SIZE - 1 } else { j_end + 1 };
        for j in j_start..=j_end {
            if is_symbol(matrix[i-1][j]) {
                return true;
            }
        }
    }
    // check below the range
    if i < (MATRIX_SIZE - 1) {
        let j_start = if j_start == 0 { 0 } else { j_start - 1 };
        let j_end = if j_end == (MATRIX_SIZE - 1) { MATRIX_SIZE - 1 } else { j_end + 1 };
        for j in j_start..=j_end {
            if is_symbol(matrix[i+1][j]) {
                return true;
            }
        }
    }

    false
}

fn is_symbol(c: char) -> bool {
    match c {
        '0'..='9' => false,
        '.' => false,
        _ => true,
    }
}