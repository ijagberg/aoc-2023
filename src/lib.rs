#![allow(unused)]
use std::{
    fs::File,
    io::{self, BufRead, Read},
};

mod boat;
mod calibration;
mod card;
mod cubes;
mod engine;
mod history;
mod map;
mod pipes;
mod scratch;
mod soil;

fn input_data(day: &str, file: &str) -> String {
    format!("inputs/{day}/{file}.txt")
}

fn read_file_contents(path: &str) -> String {
    let mut s = String::new();
    std::fs::File::open(path)
        .expect(&format!("can't open '{}'", path))
        .read_to_string(&mut s)
        .unwrap();
    s
}

#[cfg(test)]
mod day1 {
    use super::*;
    use crate::calibration::calibration_value;

    fn example1() -> String {
        read_file_contents(&input_data("day1", "example1"))
    }

    fn example2() -> String {
        read_file_contents(&input_data("day1", "example2"))
    }

    fn input() -> String {
        read_file_contents(&input_data("day1", "input"))
    }

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
    fn part1() {
        assert_eq!(solve_part1(&example1()), 142);
        assert_eq!(solve_part1(&input()), 54951);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&example2()), 281);
        assert_eq!(solve_part2(&input()), 55218);
    }
}

#[cfg(test)]
mod day2 {
    use super::*;
    use cubes::CubeSet;

    fn example1() -> String {
        read_file_contents(&input_data("day2", "example1"))
    }

    fn input() -> String {
        read_file_contents(&input_data("day2", "input"))
    }

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
    fn part1() {
        assert_eq!(solve_part1(&example1()), 8);
        assert_eq!(solve_part1(&input()), 2476);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&example1()), 2286);
        assert_eq!(solve_part2(&input()), 54911);
    }
}

#[cfg(test)]
mod day3 {
    use super::*;
    use crate::engine::{Schematic, SchematicSymbol};
    use simple_grid::Grid;

    fn example1() -> String {
        read_file_contents(&input_data("day3", "example1"))
    }

    fn input() -> String {
        read_file_contents(&input_data("day3", "input"))
    }

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
    fn part1() {
        assert_eq!(solve_part1(&example1()), 4361);
        assert_eq!(solve_part1(&input()), 546563);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&example1()), 467835);
        assert_eq!(solve_part2(&input()), 91031374);
    }
}

#[cfg(test)]
mod day4 {
    use super::*;
    use scratch::Card;
    use std::collections::{HashMap, HashSet};

    fn example1() -> String {
        read_file_contents(&input_data("day4", "example1"))
    }

    fn input() -> String {
        read_file_contents(&input_data("day4", "input"))
    }

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
    fn part1() {
        assert_eq!(solve_part1(&example1()), 13);
        assert_eq!(solve_part1(&input()), 32609);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&example1()), 30);
        assert_eq!(solve_part2(&input()), 14624680);
    }
}

#[cfg(test)]
mod day5 {
    use super::*;
    use crate::soil::{Almanac, Map, Mappings, Range};

    fn example1() -> String {
        read_file_contents(&input_data("day5", "example1"))
    }

    fn input() -> String {
        read_file_contents(&input_data("day5", "input"))
    }

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
    fn part1() {
        assert_eq!(solve_part1(&example1()), 35);
        assert_eq!(solve_part1(&input()), 313045984);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&example1()), 46);
        assert_eq!(solve_part2(&input()), 20283860);
    }
}

#[cfg(test)]
mod day6 {
    use super::*;
    use crate::boat::Race;

    fn example1() -> String {
        read_file_contents(&input_data("day6", "example1"))
    }

    fn input() -> String {
        read_file_contents(&input_data("day6", "input"))
    }

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
    fn part1() {
        assert_eq!(solve_part1(&example1()), 288);
        assert_eq!(solve_part1(&input()), 1084752);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&example1()), 71503);
        assert_eq!(solve_part2(&input()), 28228952);
    }
}

#[cfg(test)]
mod day7 {
    use super::*;
    use crate::card::{Card, Cards};
    use std::cmp::Ordering;

    fn example1() -> String {
        read_file_contents(&input_data("day7", "example1"))
    }

    fn input() -> String {
        read_file_contents(&input_data("day7", "input"))
    }

    fn parse_cards(input: &str) -> Vec<Cards> {
        let mut cards = Vec::new();
        for line in input.lines() {
            let parts: Vec<_> = line.split(" ").collect();

            debug_assert_eq!(parts.len(), 2);

            let mut chars = parts[0].chars();
            let bid = parts[1].parse().unwrap();
            cards.push(Cards::new(
                [
                    Card::try_from(chars.next().unwrap()).unwrap(),
                    Card::try_from(chars.next().unwrap()).unwrap(),
                    Card::try_from(chars.next().unwrap()).unwrap(),
                    Card::try_from(chars.next().unwrap()).unwrap(),
                    Card::try_from(chars.next().unwrap()).unwrap(),
                ],
                bid,
            ));
        }

        cards
    }

    fn get_value_of_all_cards(cards: &[Cards]) -> u32 {
        let mut total = 0;
        for i in 0..cards.len() {
            let bid = cards[i].bid();
            total += bid * (i as u32 + 1);
        }
        total
    }

    fn solve_part1(input: &str) -> u32 {
        let mut cards = parse_cards(input);

        cards.sort();

        get_value_of_all_cards(&cards)
    }

    fn solve_part2(input: &str) -> u32 {
        let all_cards = parse_cards(input);
        let mut swapped_out_optimally: Vec<_> = all_cards
            .into_iter()
            .map(|cards| swap_out_joker(cards))
            .collect();

        swapped_out_optimally.sort_by(|(cards_a, joker_pos_a), (cards_b, joker_pos_b)| {
            if cards_a.hand_type().numeric_value() == cards_b.hand_type().numeric_value() {
                // compare by value, J is 1
                for i in 0..5 {
                    let card_a = if joker_pos_a[i] {
                        Card::new(1).unwrap()
                    } else {
                        cards_a.cards()[i]
                    };
                    let card_b = if joker_pos_b[i] {
                        Card::new(1).unwrap()
                    } else {
                        cards_b.cards()[i]
                    };
                    let ord = card_a.cmp(&card_b);
                    if ord.is_ne() {
                        return ord;
                    }
                }
                return Ordering::Equal;
            } else {
                cards_a.cmp(cards_b)
            }
        });

        let cards: Vec<Cards> = swapped_out_optimally
            .into_iter()
            .map(|(cards, _)| cards)
            .collect();
        get_value_of_all_cards(&cards)
    }

    fn swap_out_joker(cards: Cards) -> (Cards, [bool; 5]) {
        let joker = Card::new(11).unwrap();
        let all_jokers = [joker; 5];
        if cards.cards() == all_jokers {
            // JJJJJ always becomes AAAAA
            let ace = Card::new(14).unwrap();
            return (Cards::new([ace; 5], cards.bid()), [true; 5]);
        }
        let mut stack = vec![cards];
        let mut joker_positions = [false; 5];
        let mut candidates = Vec::new();
        while let Some(cards) = stack.pop() {
            let mut found_joker = false;

            for i in 0..5 {
                let card = cards.cards()[i];
                if card.card_value() == 11 {
                    found_joker = true;
                    joker_positions[i] = true;
                    for c in "23456789TQKA".chars() {
                        let replaced_joker_card = Card::try_from(c).unwrap();
                        let mut cards_clone = cards.cards();
                        cards_clone[i] = replaced_joker_card;
                        stack.push(Cards::new(cards_clone, cards.bid()));
                    }
                    break;
                }
            }

            if !found_joker {
                candidates.push(cards);
            }
        }

        candidates.sort();

        (candidates[candidates.len() - 1], joker_positions)
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&example1()), 6440);
        assert_eq!(solve_part1(&input()), 247823654);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&example1()), 5905);
        assert_eq!(solve_part2(&input()), 245461700);
    }
}

#[cfg(test)]
mod day8 {
    use super::*;
    use crate::map::{Direction, Map, Node};
    use std::{collections::HashMap, fs::DirEntry};

    fn example1() -> String {
        read_file_contents(&input_data("day8", "example1"))
    }

    fn example2() -> String {
        read_file_contents(&input_data("day8", "example2"))
    }

    fn example3() -> String {
        read_file_contents(&input_data("day8", "example3"))
    }

    fn input() -> String {
        read_file_contents(&input_data("day8", "input"))
    }

    fn parse_directions(input: &str) -> Vec<Direction> {
        input
            .chars()
            .map(|c| match c {
                'R' => Direction::Right,
                'L' => Direction::Left,
                _ => panic!(),
            })
            .collect()
    }

    fn parse_map(lines: &[&str]) -> Option<Map> {
        let mut nodes = HashMap::new();

        for line in lines {
            let (id, lr) = line.split_once(" = ").unwrap();
            let (left, right) = lr.strip_prefix("(")?.strip_suffix(")")?.split_once(", ")?;
            let id: [char; 3] = {
                let mut chars = id.chars();
                [chars.next()?, chars.next()?, chars.next()?]
            };
            let left: [char; 3] = {
                let mut chars = left.chars();
                [chars.next()?, chars.next()?, chars.next()?]
            };
            let right: [char; 3] = {
                let mut chars = right.chars();
                [chars.next()?, chars.next()?, chars.next()?]
            };

            nodes.insert(id, Node(left, right));
        }
        Some(Map::new(nodes))
    }

    fn solve_part1(input: &str) -> u64 {
        let lines: Vec<_> = input.lines().collect();
        let directions = parse_directions(lines[0]);
        let map = parse_map(&lines[2..]).unwrap();

        map.path_length(['A'; 3], ['Z'; 3], &directions)
    }

    fn solve_part2(input: &str) -> u64 {
        let lines: Vec<_> = input.lines().collect();
        let directions = parse_directions(lines[0]);
        let map = parse_map(&lines[2..]).unwrap();

        let mut lengths = Vec::new();
        for &id in map.nodes().keys() {
            if id[2] == 'A' {
                let mut current = id;
                for (steps, &dir) in (0..).zip(directions.iter().cycle()) {
                    if current[2] == 'Z' {
                        lengths.push(steps);
                        break;
                    }
                    current = map.step_once(current, dir);
                }
            }
        }

        lcm_of_n(&lengths)
    }

    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    fn lcm(a: u64, b: u64) -> u64 {
        a * b / gcd(a, b)
    }

    fn lcm_of_n(numbers: &[u64]) -> u64 {
        let mut result = 1;

        for &num in numbers {
            result = lcm(result, num);
        }

        result
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&example1()), 2);
        assert_eq!(solve_part1(&example2()), 6);
        assert_eq!(solve_part1(&input()), 11567);
    }

    #[test]
    fn part2() {
        // NOTE: this felt like cheating, LCM isn't guaranteed to work and I couldn't be bothered veryfying the input before running
        assert_eq!(solve_part2(&example3()), 6);
        assert_eq!(solve_part2(&input()), 9858474970153);
    }
}

mod day9 {
    use super::*;
    use history::*;

    fn example1() -> String {
        read_file_contents(&input_data("day9", "example1"))
    }

    fn input() -> String {
        read_file_contents(&input_data("day9", "input"))
    }

    fn parse_histories(input: &str) -> Vec<ValueHistory> {
        input
            .lines()
            .map(|l| ValueHistory::new(l.split(" ").map(|p| p.parse().unwrap()).collect()))
            .collect()
    }

    fn solve_part1(input: &str) -> i64 {
        let histories = parse_histories(input);

        histories.into_iter().map(|h| h.next_value()).sum()
    }

    fn solve_part2(input: &str) -> i64 {
        let histories = parse_histories(input);

        histories.into_iter().map(|h| h.prev_value()).sum()
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&example1()), 114);
        assert_eq!(solve_part1(&input()), 1637452029);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&example1()), 2);
        assert_eq!(solve_part2(&input()), 908);
    }
}

#[cfg(test)]
mod day10 {
    use super::*;
    use crate::pipes::{Pipes, Tile};
    use simple_grid::Grid;

    fn example1() -> String {
        read_file_contents(&input_data("day10", "example1"))
    }

    fn example2() -> String {
        read_file_contents(&input_data("day10", "example2"))
    }

    fn example3() -> String {
        read_file_contents(&input_data("day10", "example3"))
    }

    fn example4() -> String {
        read_file_contents(&input_data("day10", "example4"))
    }

    fn example5() -> String {
        read_file_contents(&input_data("day10", "example5"))
    }

    fn example6() -> String {
        read_file_contents(&input_data("day10", "example6"))
    }

    fn input() -> String {
        read_file_contents(&input_data("day10", "input"))
    }

    fn parse_pipes(input: &str) -> Pipes {
        let lines: Vec<_> = input.lines().collect();
        Pipes::new(Grid::new(
            lines[0].len(),
            lines.len(),
            lines
                .into_iter()
                .flat_map(|l| l.trim().chars())
                .map(|c| Tile::try_from(c).expect(&c.to_string()))
                .collect(),
        ))
    }

    fn solve_part1(input: &str) -> usize {
        let pipes = parse_pipes(input);

        let pipe_loop = pipes.travel_loop();

        pipe_loop.len() / 2
    }

    fn solve_part2(input: &str) -> usize {
        let pipes = parse_pipes(input);

        let coverage = pipes.loop_coverage();

        coverage.len()
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&example1()), 8);
        assert_eq!(solve_part1(&example2()), 4);
        assert_eq!(solve_part1(&input()), 6979);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&example3()), 10);
        assert_eq!(solve_part2(&example4()), 8);
        assert_eq!(solve_part2(&example5()), 4);
        assert_eq!(solve_part2(&example6()), 3);
        // assert_eq!(solve_part2(&input()), 10);
    }
}
