#![allow(unused)]
use std::{
    fs::File,
    io::{self, BufRead},
};

mod calibration;
mod cubes;
mod engine;
mod scratch;

fn read_lines_from_file(path: &str) -> Vec<String> {
    let path = format!("inputs/{path}");
    let file = File::open(path).unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    lines
}

mod day1 {
    use super::*;
    use crate::calibration::calibration_value;

    fn solve_part1_from_file(path: &str) -> u32 {
        let lines = read_lines_from_file(path);
        let sum = lines
            .iter()
            .map(|l| {
                let v = calibration_value(l, false).unwrap();
                println!("{}: {}", l, v);
                v
            })
            .sum();
        sum
    }

    fn solve_part2_from_file(path: &str) -> u32 {
        let lines = read_lines_from_file(path);
        let sum = lines
            .iter()
            .map(|l| calibration_value(l, true).unwrap())
            .sum();
        sum
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1_from_file("day1/example1.txt"), 142);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1_from_file("day1/input.txt"), 54951);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2_from_file("day1/example2.txt"), 281);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2_from_file("day1/input.txt"), 55218);
    }
}

mod day2 {
    use super::*;
    use cubes::CubeSet;

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

    fn solve_part1_from_file(path: &str) -> u32 {
        let mut sum_of_possible_ids = 0;
        let limit = CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        };
        for line in read_lines_from_file(path) {
            let (id, cube_set) = parse_game(&line);
            if !cube_set.iter().any(|cs| !cubes::is_possible(limit, *cs)) {
                sum_of_possible_ids += id;
            }
        }
        sum_of_possible_ids
    }

    fn solve_part2_from_file(path: &str) -> u32 {
        read_lines_from_file(path)
            .iter()
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
        assert_eq!(solve_part1_from_file("day2/example1.txt"), 8);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1_from_file("day2/input.txt"), 2476);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2_from_file("day2/example1.txt"), 2286);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2_from_file("day2/input.txt"), 54911);
    }
}

mod day3 {
    use super::*;
    use crate::engine::{Schematic, SchematicSymbol};
    use simple_grid::Grid;

    fn parse_schematic_from_file(path: &str) -> Schematic {
        let lines = read_lines_from_file(path);
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

    fn solve_part1_from_file(path: &str) -> u32 {
        let schematic = parse_schematic_from_file(path);
        let included_parts = schematic.part_numbers();
        included_parts.into_iter().sum()
    }

    fn solve_part2_from_file(path: &str) -> u32 {
        let schematic = parse_schematic_from_file(path);
        let gears = schematic.gear_ratios();
        gears.into_iter().sum()
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1_from_file("day3/example1.txt"), 4361);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1_from_file("day3/input.txt"), 546563);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2_from_file("day3/example1.txt"), 467835);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2_from_file("day3/input.txt"), 91031374);
    }
}

mod day4 {
    use super::*;
    use scratch::Card;
    use std::collections::{HashMap, HashSet};

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

    fn parse_cards_from_file(path: &str) -> Vec<Card> {
        let cards = read_lines_from_file(path)
            .iter()
            .filter(|l| !l.trim().is_empty())
            .map(|l| parse_card_from_line(&l))
            .collect();
        cards
    }

    fn solve_part1_from_file(path: &str) -> u32 {
        let cards = parse_cards_from_file(path);
        let mut value = 0;
        for card in cards {
            let winning_numbers = card.winning_numbers().count() as u32;
            if winning_numbers > 0 {
                value += 2_u32.pow(winning_numbers - 1);
            }
        }

        value
    }

    fn solve_part2_from_file(path: &str) -> u64 {
        let cards = parse_cards_from_file(path);

        Card::total_scratchcards(&cards)
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1_from_file("day4/example1.txt"), 13);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1_from_file("day4/input.txt"), 32609);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2_from_file("day4/example1.txt"), 30);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2_from_file("day4/input.txt"), 14624680);
    }
}
