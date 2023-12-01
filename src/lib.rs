#![allow(unused)]
use std::{
    fs::File,
    io::{self, BufRead},
};

mod calibration;

fn read_lines_from_file(file: &str) -> Vec<String> {
    let file = File::open(file).unwrap();
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
        assert_eq!(solve_part1_from_file("inputs/day1_example1.txt"), 142);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1_from_file("inputs/day1.txt"), 54951);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2_from_file("inputs/day1_example2.txt"), 281);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2_from_file("inputs/day1.txt"), 55218);
    }
}
