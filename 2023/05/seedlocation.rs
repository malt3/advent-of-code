use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let input = read_maps(lines)?;

    let lowest_location_part_1 = part_1(&input)?;
    let lowest_location_part_2 = part_2_brute_backward(&input)?;
    println!("lowest location of individual seeds {}", lowest_location_part_1);
    println!("lowest location of seed ranges {}", lowest_location_part_2);

    Ok(())
}

fn part_1(input: &Input) -> Result<usize, Box<dyn std::error::Error>> {
    let mut smallest_location: usize = usize::MAX;
    for seed in &input.seeds {
        let converted = input.convert(*seed);
        if converted < smallest_location {
            smallest_location = converted;
        }
    }
    Ok(smallest_location)
}

// theoretically, this is the simplest solution for day 2, however, it is too slow by some orders of magnitudes.
#[allow(dead_code)]
fn part_2_brute_forward(input: &Input) -> Result<usize, Box<dyn std::error::Error>> {
    let mut smallest_location: usize = usize::MAX;
    for seed_range in input.seeds.chunks_exact(2) {
        for seed in seed_range[0]..seed_range[0]+seed_range[1] {
            let converted = input.convert(seed);
            if converted < smallest_location {
                smallest_location = converted;
            }
        }
    }
    Ok(smallest_location)
}

fn part_2_brute_backward(input: &Input) -> Result<usize, Box<dyn std::error::Error>> {
   for location in 0.. {
        let converted = input.reverse_convert(location);
        if input.seed_in_range(converted) {
            return Ok(location);
        }
    }
    Err("no solution found".into())
}

fn read_maps(mut lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<Input, Box<dyn std::error::Error>> {
    let seeds = read_seeds(&lines.next().ok_or("invalid input: expected a line with seeds")??).ok_or("invalid input")?;
    lines.next(); // skip empty line
    let mut maps = std::collections::HashMap::new();
    let mut reverse_maps = std::collections::HashMap::new();
    while let Some((map, reverse_map)) = read_map(&mut lines)? {
        maps.insert(map.from.clone(), map);
        reverse_maps.insert(reverse_map.from.clone(), reverse_map);
    }
    let input = Input {
        seeds: seeds.0,
        seed_ranges: seeds.1,
        maps: maps,
        reverse_maps: reverse_maps,
    };
    Ok(input)
}

fn read_seeds(line: &str) -> Option<(Vec<usize>, SeedRanges)> {
    let mut seeds = Vec::new();
    if !line.starts_with("seeds: ") {
        return None;
    }
    let line = line[7..].trim();
    for seed in line.split_whitespace() {
        seeds.push(seed.parse::<usize>().ok()?);
    }
    let mut seed_ranges = SeedRanges::new();
    for seed_range in seeds.chunks_exact(2) {
        seed_ranges.insert(seed_range[0], seed_range[1]);
    }
    Some((seeds, seed_ranges))
}

fn read_conversion_entry(line: &str) -> Option<ConversionEntry> {
    let mut parts = line.split_whitespace();
    let destination_range_start = parts.next()?.parse::<usize>().ok()?;
    let source_range_start = parts.next()?.parse::<usize>().ok()?;
    let range_length = parts.next()?.parse::<usize>().ok()?;
    Some(ConversionEntry {
        destination_range_start,
        source_range_start,
        range_length,
    })
}

fn read_map(mut lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<Option<(Conversion, Conversion)>, Box<dyn std::error::Error>> {
    let description = lines.next(); // first line with description of mapping "KEY-to-VALUE map:"
    if description.is_none() {
        return Ok(None);
    }
    let description = description.ok_or("invalid map")??;
    let key_to_value = description.split_once(" ").ok_or("invalid map description line: expected space")?;
    let mut parts = key_to_value.0.split("-");
    let key = parts.next().ok_or("invalid map description line: expected map key")?;
    parts.next();
    let value = parts.next().ok_or("invalid map description line: expected map value")?;

    let mut conversion_fwd = Conversion::new(&key, &value);
    let mut conversion_reverse = Conversion::new(&value, &key);
    for line in lines {
        let line = line?;
        if line.is_empty() {
            break;
        }
        if let Some(conv) = read_conversion_entry(&line) {
            conversion_fwd.insert(conv.source_range_start, conv.clone());
            let reverse_conv = ConversionEntry {
                destination_range_start: conv.source_range_start,
                source_range_start: conv.destination_range_start,
                range_length: conv.range_length,
            };
            conversion_reverse.insert(reverse_conv.source_range_start, reverse_conv);
        }
    }
    Ok(Some((conversion_fwd, conversion_reverse)))
}

#[derive(Clone, Debug, PartialEq)]
struct ConversionEntry {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}
type ConversionMap = std::collections::BTreeMap<usize, ConversionEntry>;

type SeedRanges = std::collections::BTreeMap<usize, usize>;

struct Conversion{
    map: ConversionMap,
    from: String,
    to: String,
}

impl Conversion {
    fn new(from: &str, to: &str) -> Self {
        Self {
            map: ConversionMap::new(),
            from: from.to_string(),
            to: to.to_string(),
        }
    }

    fn insert(&mut self, key: usize, value: ConversionEntry) {
        self.map.insert(key, value);
    }

    fn lookup(&self, i: usize) -> usize {
        let conv = match self.map.range(..=i).next_back() {
            None => return i,
            Some(conv) => conv.1,
        };
        if i > (conv.source_range_start + conv.range_length) {
            return i;
        }
        i - conv.source_range_start + conv.destination_range_start
    }
}

struct Input {
    seeds: Vec<usize>,
    seed_ranges: SeedRanges,
    maps: std::collections::HashMap<String, Conversion>,
    reverse_maps: std::collections::HashMap<String, Conversion>,
}

impl Input {
    fn seed_in_range(&self, seed: usize) -> bool {
        let range = match self.seed_ranges.range(..=seed).next_back() {
            None => return false,
            Some(range) => range,
        };
        return seed > *range.0 && seed < (range.0 + range.1);
    }
    fn convert(&self, seed: usize) -> usize {
        let mut current = seed;
        let mut map_name = "seed".to_string();
        while let Some(map) = self.maps.get(&map_name) {
            current = map.lookup(current);
            map_name = map.to.clone();
        }
        current
    }
    fn reverse_convert(&self, seed: usize) -> usize {
        let mut current = seed;
        let mut map_name = "location".to_string();
        while let Some(map) = self.reverse_maps.get(&map_name) {
            current = map.lookup(current);
            map_name = map.to.clone();
        }
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_iter() -> impl Iterator<Item = Result<String, io::Error>> {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#
        .split("\n")
        .map(|s| Ok(s.to_string()));
        
        input
    }

    #[test]
    fn test_parser() {
        let input = read_maps(example_iter()).unwrap();
        assert_eq!(input.seeds, vec![79, 14, 55, 13]);
        assert_eq!(input.maps.len(), 7);
        let seed_to_soil = input.maps.get("seed").unwrap();
        assert_eq!(seed_to_soil.from, "seed");
        assert_eq!(seed_to_soil.to, "soil");
        assert_eq!(seed_to_soil.map.len(), 2);
        assert_eq!(*seed_to_soil.map.get(&98).unwrap(), ConversionEntry { destination_range_start: 50, source_range_start: 98, range_length: 2 });
        assert_eq!(*seed_to_soil.map.get(&50).unwrap(), ConversionEntry { destination_range_start: 52, source_range_start: 50, range_length: 48 });
        assert_eq!(input.maps.get("soil").unwrap().map.len(), 3);
        assert_eq!(input.maps.get("fertilizer").unwrap().map.len(), 4);
        assert_eq!(input.maps.get("water").unwrap().map.len(), 2);
        assert_eq!(input.maps.get("light").unwrap().map.len(), 3);
        assert_eq!(input.maps.get("temperature").unwrap().map.len(), 2);
        assert_eq!(input.maps.get("humidity").unwrap().map.len(), 2);
    }

    #[test]
    fn test_lookup() {
        let input = read_maps(example_iter()).unwrap();
        assert_eq!(input.maps.get("seed").unwrap().lookup(79), 81);
        assert_eq!(input.maps.get("seed").unwrap().lookup(14), 14);
        assert_eq!(input.maps.get("seed").unwrap().lookup(55), 57);
        assert_eq!(input.maps.get("seed").unwrap().lookup(13), 13);
    }

    #[test]
    fn test_convert() {
        let input = read_maps(example_iter()).unwrap();
        assert_eq!(input.convert(79), 82);
        assert_eq!(input.convert(14), 43);
        assert_eq!(input.convert(55), 86);
        assert_eq!(input.convert(13), 35);
    }

    #[test]
    fn test_part_1() {
        let input = read_maps(example_iter()).unwrap();
        assert_eq!(part_1(&input).unwrap(), 35);
    }

    #[test]
    fn test_part_2() {
        let input = read_maps(example_iter()).unwrap();
        assert_eq!(part_2_brute_forward(&input).unwrap(), 46);
        assert_eq!(part_2_brute_backward(&input).unwrap(), 46);
    }

}