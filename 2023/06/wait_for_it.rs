use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let input = read_race_part2(lines)?;

    let wins = calculate_number_of_wins_multiplied(&input);
    println!("number of wins multiplied: {}", wins);

    Ok(())
}

fn winning_range(current_best_distance: i64, time_limit: i64) -> i64 {
    let t = time_limit as f64;
    let d = current_best_distance as f64;
    let x1 = f64::ceil((t + f64::sqrt(t.powf(2.0) - 4.0 * d)) / 2.0);
    let x2 = f64::floor((t - f64::sqrt(t.powf(2.0) - 4.0 * d)) / 2.0) + 1.0;
    return (x1 - x2) as i64;
}

fn read_number_row(line: &str) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    let mut numbers = Vec::new();
    let line = line.split_once(":").ok_or("no : found")?.1;
    for number in line.split(" ") {
        if number == "" {
            continue;
        }
        numbers.push(number.parse::<i64>()?);
    }
    Ok(numbers)
}

fn read_single_number_row(line: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let line = line.split_once(":").ok_or("no : found")?.1;
    let cleaned_string = line.replace(" ", "");
    cleaned_string.parse::<i64>().map_err(|e| e.into())
}

#[allow(dead_code)]
fn read_races_part1(mut lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<Vec<(i64, i64)>, Box<dyn std::error::Error>> {
    let times = read_number_row(&lines.next().ok_or("no first line")??)?;
    let distances = read_number_row(&lines.next().ok_or("no second line")??)?;
    if times.len() != distances.len() {
        return Err("times and distances are not the same length".into());
    }

    let mut combined = Vec::new();
    for (time, distance) in times.iter().zip(distances.iter()) {
        combined.push((*time, *distance));
    }

    Ok(combined)
}

fn read_race_part2(mut lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<Vec<(i64, i64)>, Box<dyn std::error::Error>> {
    let time = read_single_number_row(&lines.next().ok_or("no first line")??)?;
    let distance = read_single_number_row(&lines.next().ok_or("no second line")??)?;
    Ok(vec![(time, distance)])
}

fn calculate_number_of_wins_multiplied(input: &[(i64, i64)]) -> i64 {
    let mut wins = 1;
    for game in input {
        wins *= winning_range(game.1, game.0);
    }
    wins
}