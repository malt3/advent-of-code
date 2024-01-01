use std::{collections::{HashSet, HashMap}, io::{self, BufRead}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let (sum_part_1, sum_part_2) = calculate_wins(lines)?;

    println!("part 1: {}", sum_part_1);
    println!("part 2: {}", sum_part_2);

    Ok(())
}

fn calculate_wins(lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<(u32, usize), Box<dyn std::error::Error>> {
    let mut card_counts: HashMap<usize, usize> = HashMap::new();
    let mut sum_part_1: u32 = 0;
    for (i, line) in lines.enumerate() {
        let line = line?;
        let (winning_numbers, have_numbers) = parse_line(&line).ok_or("invalid line")?;
        // part 1
        sum_part_1 += part_1(&winning_numbers, &have_numbers);
        // part 2
        let count = card_counts.get(&i).unwrap_or(&0).clone() + 1;
        card_counts.insert(i, count);
        part_2(i, &winning_numbers, &have_numbers, &mut card_counts);
    }
    Ok((
        sum_part_1,
        card_counts.into_iter().fold(0, |acc, (_, count)| acc + count)
    ))
}

fn part_1(winning_numbers: &HashSet<u32>, have_numbers: &HashSet<u32>) -> u32 {
    let count = winning_numbers.intersection(have_numbers).count();
    (1 << count as u32) >> 1
}

fn part_2(i: usize, winning_numbers: &HashSet<u32>, have_numbers: &HashSet<u32>, card_counts: &mut HashMap<usize, usize>) {
    // instances is the amount of same cards we have collected
    let instances = card_counts.get(&i).unwrap_or(&0).clone();
    // wins is the amount of numbers that match
    let wins = winning_numbers.intersection(have_numbers).count();
    for j in i+1..i+1+wins {
        match card_counts.get(&j) {
            Some(count) => { card_counts.insert(j, count + instances); }
            None => { card_counts.insert(j, instances); }
        }
    }
}

fn parse_line(line: &str) -> Option<(HashSet<u32>, HashSet<u32>)> {
    let line = line.split_once(':')?;
    let parts = line.1.split_once("|")?;
    let winning_part = parts.0.trim_end();
    let have_part = parts.1.trim_end();
    Some((parse_number_grid(winning_part), parse_number_grid(have_part)))
}   

fn parse_number_grid(grid: &str) -> HashSet<u32> {
    let mut numbers = HashSet::new();
    let safe_gridlen = grid.len() - (grid.len() % 3);
    for i in (0..safe_gridlen).step_by(3) {
        let mut number = 0;
        for j in 0..3 {
            number = (number * 10) + read_digit(grid.chars().nth(i + j).unwrap());
        }
        numbers.insert(number);
    }

    numbers
}

fn read_digit(c: char) -> u32 {
    match c {
        '0'..='9' => c as u32 - '0' as u32,
        _ => 0,
    }
}