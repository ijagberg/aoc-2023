#![allow(unused)]
use std::{
    fs::File,
    io::{self, BufRead},
};

mod boat;
mod calibration;
mod cubes;
mod engine;
mod scratch;
mod soil;

fn read_lines_from_file(path: &str) -> impl Iterator<Item = String> {
    let path = format!("inputs/{path}");
    let file = File::open(path).unwrap();
    io::BufReader::new(file).lines().map(|l| l.unwrap())
}

#[cfg(test)]
mod day1 {
    use super::*;
    use crate::calibration::calibration_value;

    const INPUT: &str = include_str!("../inputs/day1/input.txt");
    const EXAMPLE_1: &str = include_str!("../inputs/day1/example1.txt");
    const EXAMPLE_2: &str = include_str!("../inputs/day1/example2.txt");

    fn solve_part1(input: &str) -> u32 {
        let sum = input
            .lines()
            .map(|l| {
                let v = calibration_value(&l, false).unwrap();
                println!("{}: {}", l, v);
                v
            })
            .sum();
        sum
    }

    fn solve_part2(input: &str) -> u32 {
        let sum = input
            .lines()
            .map(|l| calibration_value(&l, true).unwrap())
            .sum();
        sum
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1(EXAMPLE_1), 142);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 54951);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(solve_part2(EXAMPLE_2), 281);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 55218);
    }
}

#[cfg(test)]
mod day2 {
    use super::*;
    use cubes::CubeSet;

    const INPUT: &str = include_str!("../inputs/day2/input.txt");
    const EXAMPLE_1: &str = include_str!("../inputs/day2/example1.txt");

    fn parse_game(line: &str) -> (u32, Vec<CubeSet>) {
        let (game_id, game_grabs) = line.split_once(": ").unwrap();
        let id: u32 = game_id
            .strip_prefix("Game ")
            .map(|id| id.parse().unwrap())
            .unwrap();
        let sets: Vec<CubeSet> = game_grabs
            .split("; ")
            .map(|grab| {
                let mut cube_set = CubeSet::default();
                // 3 blue, 4 red
                let parts = grab.split(", ");
                for part in parts {
                    // 3 blue
                    let (num, color) = part.split_once(" ").unwrap();
                    let num = num.parse().unwrap();
                    match color {
                        "red" => cube_set.red = num,
                        "green" => cube_set.green = num,
                        "blue" => cube_set.blue = num,
                        _ => unreachable!(),
                    }
                }
                cube_set
            })
            .collect();

        (id, sets)
    }

    fn solve_part1(input: &str) -> u32 {
        let mut sum_of_possible_ids = 0;
        let limit = CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        };
        for line in input.lines() {
            let (id, cube_set) = parse_game(&line);
            if !cube_set.iter().any(|cs| !cubes::is_possible(limit, *cs)) {
                sum_of_possible_ids += id;
            }
        }
        sum_of_possible_ids
    }

    fn solve_part2(input: &str) -> u32 {
        input
            .lines()
            .map(|line| {
                let (id, grabs) = parse_game(&line);
                let red_required = grabs.iter().map(|g| g.red).max().unwrap_or(0);
                let green_required = grabs.iter().map(|g| g.green).max().unwrap_or(0);
                let blue_required = grabs.iter().map(|g| g.blue).max().unwrap_or(0);
                red_required * green_required * blue_required
            })
            .sum()
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1(EXAMPLE_1), 8);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 2476);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(EXAMPLE_1), 2286);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 54911);
    }
}

#[cfg(test)]
mod day3 {
    use super::*;
    use crate::engine::{Schematic, SchematicSymbol};
    use simple_grid::Grid;

    const INPUT: &str = include_str!("../inputs/day3/input.txt");
    const EXAMPLE_1: &str = include_str!("../inputs/day3/example1.txt");

    fn parse_schematic(input: &str) -> Schematic {
        let lines: Vec<&str> = input.lines().collect();
        let w = lines[0].len();
        let h = lines.len();
        let chars: Vec<SchematicSymbol> = lines
            .join("")
            .chars()
            .map(|c| match c.to_digit(10) {
                Some(digit) => SchematicSymbol::Number(digit),
                None if c == '.' => SchematicSymbol::Period,
                None => SchematicSymbol::Symbol(c),
            })
            .collect();

        Schematic::new(Grid::new(w, h, chars))
    }

    fn solve_part1(input: &str) -> u32 {
        let schematic = parse_schematic(input);
        let included_parts = schematic.part_numbers();
        included_parts.into_iter().sum()
    }

    fn solve_part2(input: &str) -> u32 {
        let schematic = parse_schematic(input);
        let gears = schematic.gear_ratios();
        gears.into_iter().sum()
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1(EXAMPLE_1), 4361);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 546563);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(EXAMPLE_1), 467835);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 91031374);
    }
}

#[cfg(test)]
mod day4 {
    use super::*;
    use scratch::Card;
    use std::collections::{HashMap, HashSet};

    const INPUT: &str = include_str!("../inputs/day4/input.txt");
    const EXAMPLE_1: &str = include_str!("../inputs/day4/example1.txt");

    fn parse_card_from_line(line: &str) -> Card {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let (id, numbers) = line
            .trim()
            .strip_prefix("Card ")
            .unwrap()
            .trim()
            .split_once(": ")
            .unwrap();
        let id = id.parse().unwrap();
        // numbers = 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let (winning, hand) = numbers.split_once(" | ").unwrap();
        let winning: HashSet<u32> = winning
            .split_whitespace()
            .map(|p| p.parse().unwrap())
            .collect();
        let hand: HashSet<u32> = hand
            .split_whitespace()
            .map(|p| p.parse().unwrap())
            .collect();

        Card::new(id, winning, hand)
    }

    fn parse_cards(input: &str) -> Vec<Card> {
        let cards = input
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| parse_card_from_line(&l))
            .collect();
        cards
    }

    fn solve_part1(input: &str) -> u32 {
        let cards = parse_cards(input);
        let mut value = 0;
        for card in cards {
            let winning_numbers = card.winning_numbers().count() as u32;
            if winning_numbers > 0 {
                value += 2_u32.pow(winning_numbers - 1);
            }
        }

        value
    }

    fn solve_part2(input: &str) -> u64 {
        let cards = parse_cards(input);

        Card::total_scratchcards(&cards)
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1(EXAMPLE_1), 13);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 32609);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(EXAMPLE_1), 30);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 14624680);
    }
}

#[cfg(test)]
mod day5 {
    use super::*;
    use crate::soil::{Almanac, Map, Mappings, Range};

    const INPUT: &str = include_str!("../inputs/day5/input.txt");
    const EXAMPLE_1: &str = include_str!("../inputs/day5/example1.txt");

    fn parse_seeds_from_line(line: &str) -> Vec<u64> {
        // seeds: 1 2 3
        line.strip_prefix("seeds: ")
            .unwrap()
            .split(" ")
            .map(|p| p.parse().unwrap())
            .collect()
    }

    fn parse_maps_from_lines(lines: &[&str]) -> Mappings {
        let mut maps = Vec::with_capacity(lines.len());
        for line in &lines[1..] {
            let parts: Vec<u64> = line.split(" ").map(|p| p.parse().unwrap()).collect();
            assert_eq!(parts.len(), 3);
            maps.push(Map::new(parts[0], parts[1], parts[2]));
        }

        Mappings::new(lines[0].to_string(), maps)
    }

    fn parse_almanac(input: &str) -> (Vec<u64>, Almanac) {
        let lines: Vec<&str> = input.lines().collect();

        let seeds = parse_seeds_from_line(&lines[0]);
        let map_lines: Vec<_> = lines[2..].split(|l| l.trim().is_empty()).collect();

        let mut map_tiers = Vec::with_capacity(map_lines.len());

        for l in map_lines {
            map_tiers.push(parse_maps_from_lines(l));
        }

        (seeds, Almanac::new(map_tiers))
    }

    fn solve_part1(input: &str) -> u64 {
        let (seeds, almanac) = parse_almanac(input);

        seeds
            .into_iter()
            .flat_map(|seed| almanac.convert(Range::new(seed, seed + 1)))
            .map(|r| r.start())
            .min()
            .unwrap()
    }

    fn solve_part2(input: &str) -> u64 {
        let (seeds, almanac) = parse_almanac(input);
        seeds
            .chunks(2)
            .into_iter()
            .flat_map(|w| almanac.convert(Range::new(w[0], w[0] + w[1])))
            .map(|r| r.start())
            .min()
            .unwrap()
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1(EXAMPLE_1), 35);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 313045984);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(EXAMPLE_1), 46);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 20283860);
    }
}

#[cfg(test)]
mod day6 {
    use super::*;
    use crate::boat::Race;

    const INPUT: &str = include_str!("../inputs/day6/input.txt");
    const EXAMPLE_1: &str = include_str!("../inputs/day6/example1.txt");

    fn parse_races_part1(input: &str) -> Vec<Race> {
        let lines: Vec<&str> = input.lines().collect();
        assert_eq!(lines.len(), 2);

        let times: Vec<u64> = lines[0]
            .strip_prefix("Time:")
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|p| p.parse().unwrap())
            .collect();

        let records: Vec<u64> = lines[1]
            .strip_prefix("Distance:")
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|p| p.parse().unwrap())
            .collect();

        assert_eq!(times.len(), records.len());

        times
            .into_iter()
            .zip(records.into_iter())
            .map(|(time, record)| Race::new(time, record))
            .collect()
    }

    fn parse_race_part2(input: &str) -> Race {
        let lines: Vec<&str> = input.lines().collect();
        assert_eq!(lines.len(), 2);

        let parts: Vec<&str> = lines[0]
            .strip_prefix("Time:")
            .unwrap()
            .trim()
            .split_whitespace()
            .collect();

        let time: u64 = parts.join("").parse().unwrap();

        let parts: Vec<&str> = lines[1]
            .strip_prefix("Distance:")
            .unwrap()
            .trim()
            .split_whitespace()
            .collect();

        let record: u64 = parts.join("").parse().unwrap();

        Race::new(time, record)
    }

    fn solve_part1(input: &str) -> u64 {
        let races = parse_races_part1(input);

        let mut product = 1;
        for race in races {
            let mut ways_to_beat = 0;
            for held in 1..race.time() {
                if race.beats_record(held) {
                    ways_to_beat += 1;
                }
            }
            product *= ways_to_beat;
        }

        product
    }

    fn solve_part2(input: &str) -> u32 {
        let race = parse_race_part2(input);

        let mut ways_to_beat = 0;
        let mid = race.time() / 2;
        for i in 0.. {
            let mut beat_record = false;
            if race.beats_record(mid + i + 1) {
                // go right
                beat_record = true;
                ways_to_beat += 1;
            }
            if race.beats_record(mid - i) {
                // go left
                beat_record = true;
                ways_to_beat += 1;
            }
            if !beat_record {
                break;
            }
        }

        ways_to_beat
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1(EXAMPLE_1), 288);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 1084752);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(EXAMPLE_1), 71503);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 28228952);
    }
}
