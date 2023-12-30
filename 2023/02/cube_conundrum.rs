use std::io::{self, BufRead};
use std::cmp::max;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let (sum_part_1, sum_part_2) = evaluate_games(lines)?;

    println!("sum of possible game ids: {}", sum_part_1);
    println!("sum of powers: {}", sum_part_2);

    Ok(())
}

fn evaluate_games(lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let mut sum_of_possible_game_ids: u32 = 0;
    let mut sum_of_powers: u32 = 0;
    for line in lines {
        let line = line?;
        let (game_id, possible) = eval_game_part_1(&line, (12, 13, 14)).ok_or("invalid line")?;
        let (_, power) = eval_game_part_2(&line).ok_or("invalid line")?;
        if possible {
            sum_of_possible_game_ids +=  game_id;
        }
        sum_of_powers += power;
    }
    Ok((sum_of_possible_game_ids, sum_of_powers))
}

fn eval_game_part_1(line: &str, constraints: (u32, u32, u32)) -> Option<(u32, bool)>{
    let (game_id, reveals) = line[5..].split_once(":")?;
    let game_id = game_id.parse::<u32>().ok()?;
    let reveals = reveals.split(";");
    for reveal in reveals {
        let counts = reveal.split(",");
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for count in counts {
            let (count, color) = count.trim().split_once(" ")?;
            let count = count.parse::<u32>().ok()?;
            match color {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => (),
            }
        }
        if constraints.0 < red || constraints.1 < green || constraints.2 < blue {
            return Some((game_id, false));
        }
    }
    Some((game_id, true))
}

fn eval_game_part_2(line: &str) -> Option<(u32, u32)>{
    let (game_id, reveals) = line[5..].split_once(":")?;
    let game_id = game_id.parse::<u32>().ok()?;
    let reveals = reveals.split(";");
    let (mut req_red, mut req_green, mut req_blue) = (0, 0, 0);
    for reveal in reveals {
        let counts = reveal.split(",");
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for count in counts {
            let (count, color) = count.trim().split_once(" ")?;
            let count = count.parse::<u32>().ok()?;
            match color {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => (),
            }
        }
        req_red = max(req_red, red);
        req_green = max(req_green, green);
        req_blue = max(req_blue, blue);
    }
    Some((game_id, req_red * req_green * req_blue))
}