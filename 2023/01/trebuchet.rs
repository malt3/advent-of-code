use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let (sum_part_1, sum_part_2) = calculate_sums(lines)?;

    println!("part 1: {}\npart 2: {}", sum_part_1, sum_part_2);

    Ok(())
}

fn calculate_sums(lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let mut sum_part_1: u32 = 0;
    let mut sum_part_2: u32 = 0;
    for line in lines {
        let line = line?;
        sum_part_1 += recover_digit_calibration_value(&line);
        sum_part_2 += recover_spelled_calibration_value(&line);
    }
    Ok((sum_part_1, sum_part_2))
}


fn recover_digit_calibration_value(line: &str) -> u32 {
    let mut first_digit: u32 = 0;
    let mut last_digit: u32 = 0;
    for c in line.chars() {
        if c.is_digit(10) {
            first_digit = c.to_digit(10).unwrap();
            break;
        }
    }
    for c in line.chars().rev() {
        if c.is_digit(10) {
            last_digit = c.to_digit(10).unwrap();
            break;
        }
    }
    (first_digit * 10) + last_digit
}


fn recover_spelled_calibration_value(line: &str) -> u32 {
    let mut first_digit: u32 = 0;
    let mut last_digit: u32 = 0;
    for (i, _) in line.chars().enumerate() {
        if let Some(digit) = match_digit(&line[i..]) {
            first_digit = digit;
            break;
        }
    }

    for (i, _) in line.chars().rev().enumerate() {
        if let Some(digit) = match_late_digit(&line[..line.len() - i]) {
            last_digit = digit;
            break;
        }
    }
    (first_digit * 10) + last_digit
}


fn match_digit(substr: &str) -> Option<u32> {
    let mut chrs = substr.chars();
    let(c0, c1, c2, c3, c4) = match substr.len() {
        0 => ('_',  '_', '_', '_', '_'),
        1 => (chrs.next().unwrap(), '_', '_', '_', '_'),
        2 => (chrs.next().unwrap(), chrs.next().unwrap(), '_', '_', '_'),
        3 => (chrs.next().unwrap(), chrs.next().unwrap(), chrs.next().unwrap(), '_', '_'),
        4 => (chrs.next().unwrap(), chrs.next().unwrap(), chrs.next().unwrap(),  chrs.next().unwrap(), '_'),
        _ => (chrs.next().unwrap(), chrs.next().unwrap(), chrs.next().unwrap(),  chrs.next().unwrap(), chrs.next().unwrap()),
    };
    match (c0, c1, c2, c3, c4) {
        ('0', _, _, _, _) => Some(0),
        ('1', _, _, _, _) => Some(1),
        ('2', _, _, _, _) => Some(2),
        ('3', _, _, _, _) => Some(3),
        ('4', _, _, _, _) => Some(4),
        ('5', _, _, _, _) => Some(5),
        ('6', _, _, _, _) => Some(6),
        ('7', _, _, _, _) => Some(7),
        ('8', _, _, _, _) => Some(8),
        ('9', _, _, _, _) => Some(9),
        ('o', 'n', 'e', _, _) => Some(1),
        ('t', 'w', 'o', _, _) => Some(2),
        ('t', 'h', 'r', 'e', 'e') => Some(3),
        ('f', 'o', 'u', 'r', _) => Some(4),
        ('f', 'i', 'v', 'e', _) => Some(5),
        ('s', 'i', 'x', _, _) => Some(6),
        ('s', 'e', 'v', 'e', 'n') => Some(7),
        ('e', 'i', 'g', 'h', 't') => Some(8),
        ('n', 'i', 'n', 'e', _) => Some(9),
        _ => None,
    }
}

fn match_late_digit(substr: &str) -> Option<u32> {
    let mut chrs = substr.chars().rev();
    let(c0, c1, c2, c3, c4) = match substr.len() {
        0 => ('_',  '_', '_', '_', '_'),
        1 => (chrs.next().unwrap(), '_', '_', '_', '_'),
        2 => (chrs.next().unwrap(), chrs.next().unwrap(), '_', '_', '_'),
        3 => (chrs.next().unwrap(), chrs.next().unwrap(), chrs.next().unwrap(), '_', '_'),
        4 => (chrs.next().unwrap(), chrs.next().unwrap(), chrs.next().unwrap(), chrs.next().unwrap(), '_'),
        _ => (chrs.next().unwrap(), chrs.next().unwrap(), chrs.next().unwrap(), chrs.next().unwrap(), chrs.next().unwrap()),
    };
    match (c0, c1, c2, c3, c4) {
        ('0', _, _, _, _) => Some(0),
        ('1', _, _, _, _) => Some(1),
        ('2', _, _, _, _) => Some(2),
        ('3', _, _, _, _) => Some(3),
        ('4', _, _, _, _) => Some(4),
        ('5', _, _, _, _) => Some(5),
        ('6', _, _, _, _) => Some(6),
        ('7', _, _, _, _) => Some(7),
        ('8', _, _, _, _) => Some(8),
        ('9', _, _, _, _) => Some(9),
        ('e', 'n', 'o', _, _) => Some(1),
        ('o', 'w', 't', _, _) => Some(2),
        ('e', 'e', 'r', 'h', 't') => Some(3),
        ('r', 'u', 'o', 'f', _) => Some(4),
        ('e', 'v', 'i', 'f', _) => Some(5),
        ('x', 'i', 's', _, _) => Some(6),
        ('n', 'e', 'v', 'e', 's') => Some(7),
        ('t', 'h', 'g', 'i', 'e') => Some(8),
        ('e', 'n', 'i', 'n', _) => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_digit() {
        assert_eq!(match_digit(""), None);
        assert_eq!(match_digit("_"), None);
        assert_eq!(match_digit("0"), Some(0));
        assert_eq!(match_digit("1"), Some(1));
        assert_eq!(match_digit("2"), Some(2));
        assert_eq!(match_digit("3"), Some(3));
        assert_eq!(match_digit("4"), Some(4));
        assert_eq!(match_digit("5"), Some(5));
        assert_eq!(match_digit("6"), Some(6));
        assert_eq!(match_digit("7"), Some(7));
        assert_eq!(match_digit("8"), Some(8));
        assert_eq!(match_digit("9"), Some(9));
        assert_eq!(match_digit("one"), Some(1));
        assert_eq!(match_digit("two"), Some(2));
        assert_eq!(match_digit("three"), Some(3));
        assert_eq!(match_digit("four"), Some(4));
        assert_eq!(match_digit("five"), Some(5));
        assert_eq!(match_digit("six"), Some(6));
        assert_eq!(match_digit("seven"), Some(7));
        assert_eq!(match_digit("eight"), Some(8));
        assert_eq!(match_digit("nine"), Some(9));
        assert_eq!(match_digit("ten"), None);
        assert_eq!(match_digit("eleven"), None);
        assert_eq!(match_digit("twelve"), None);
        assert_eq!(match_digit("thirteen"), None);
        assert_eq!(match_digit("fourfoo"), Some(4));
        assert_eq!(match_digit("fivebar"), Some(5));
        assert_eq!(match_digit("6baz"), Some(6));
        assert_eq!(match_digit("x1"), None);
    }

    #[test]
    fn test_match_late_digit() {
        assert_eq!(match_late_digit(""), None);
        assert_eq!(match_late_digit("_"), None);
        assert_eq!(match_late_digit("0"), Some(0));
        assert_eq!(match_late_digit("a0"), Some(0));
        assert_eq!(match_late_digit("aa0"), Some(0));
        assert_eq!(match_late_digit("aaa0"), Some(0));
        assert_eq!(match_late_digit("aaaa0"), Some(0));
        assert_eq!(match_late_digit("aaaaa0"), Some(0));
        assert_eq!(match_late_digit("1"), Some(1));
        assert_eq!(match_late_digit("2"), Some(2));
        assert_eq!(match_late_digit("3"), Some(3));
        assert_eq!(match_late_digit("4"), Some(4));
        assert_eq!(match_late_digit("5"), Some(5));
        assert_eq!(match_late_digit("6"), Some(6));
        assert_eq!(match_late_digit("7"), Some(7));
        assert_eq!(match_late_digit("8"), Some(8));
        assert_eq!(match_late_digit("9"), Some(9));
        assert_eq!(match_late_digit("one"), Some(1));
        assert_eq!(match_late_digit("aone"), Some(1));
        assert_eq!(match_late_digit("aaone"), Some(1));
        assert_eq!(match_late_digit("aaaone"), Some(1));
        assert_eq!(match_late_digit("aaaaone"), Some(1));
        assert_eq!(match_late_digit("aaaaaone"), Some(1));
        assert_eq!(match_late_digit("aaaaaaone"), Some(1));
        assert_eq!(match_late_digit("two"), Some(2));
        assert_eq!(match_late_digit("three"), Some(3));
        assert_eq!(match_late_digit("four"), Some(4));
        assert_eq!(match_late_digit("five"), Some(5));
        assert_eq!(match_late_digit("six"), Some(6));
        assert_eq!(match_late_digit("seven"), Some(7));
        assert_eq!(match_late_digit("eight"), Some(8));
        assert_eq!(match_late_digit("nine"), Some(9));
        assert_eq!(match_late_digit("ten"), None);
        assert_eq!(match_late_digit("eleven"), None);
        assert_eq!(match_late_digit("twelve"), None);
        assert_eq!(match_late_digit("thirteen"), None);
        assert_eq!(match_late_digit("foofour"), Some(4));
        assert_eq!(match_late_digit("_five"), Some(5));
        assert_eq!(match_late_digit("baz6"), Some(6));
        assert_eq!(match_late_digit("1x"), None);
    }

    #[test]
    fn test_digit_lines() {
        assert_eq!(recover_digit_calibration_value("1abc2"), 12);
        assert_eq!(recover_digit_calibration_value("pqr3stu8vwx"), 38);
        assert_eq!(recover_digit_calibration_value("a1b2c3d4e5f"), 15);
        assert_eq!(recover_digit_calibration_value("treb7uchet"), 77);
        assert_eq!(recover_digit_calibration_value("sixfdqttpskdnbksqxg9three6bqqpngfhz"), 96);
    }

    #[test]
    fn test_spelled_lines() {
        assert_eq!(recover_spelled_calibration_value("two1nine"), 29);
        assert_eq!(recover_spelled_calibration_value("eightwothree"), 83);
        assert_eq!(recover_spelled_calibration_value("abcone2threexyz"), 13);
        assert_eq!(recover_spelled_calibration_value("xtwone3four"), 24);
        assert_eq!(recover_spelled_calibration_value("4nineeightseven2"), 42);
        assert_eq!(recover_spelled_calibration_value("zoneight234"), 14);
        assert_eq!(recover_spelled_calibration_value("7pqrstsixteen"), 76);
        assert_eq!(recover_spelled_calibration_value("sixfdqttpskdnbksqxg9three6bqqpngfhz"), 66);

    }

    #[test]
    fn test_calculate_digit_sum() {
        let lines_first = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        assert_eq!(calculate_sums(lines_first.into_iter().map(|s| Ok(s.to_string()))).unwrap(), (142, 142));
        let lines_second = vec!["two1nine", "eightwothree", "abcone2threexyz", "xtwone3four", "4nineeightseven2", "zoneight234", "7pqrstsixteen"];
        assert_eq!(calculate_sums(lines_second.into_iter().map(|s| Ok(s.to_string()))).unwrap(), (209, 281));
    }
}