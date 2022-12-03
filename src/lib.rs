#![allow(unused)]

use std::{
    fs::File,
    io::{self, BufRead},
};

mod calories;

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
    use crate::calories::{Elf, Elves};

    fn solve_part1_from_file(path: &str) -> u32 {
        let lines = read_lines_from_file(path);
        let groups: Vec<_> = lines.split(|s| s.is_empty()).collect();
        let mut elves = Vec::new();
        for group in groups {
            let elf = Elf::new(
                group
                    .into_iter()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect(),
            );
            elves.push(elf);
        }

        let c = Elves::new(elves);

        c.highest_total_calories()
    }

    fn solve_part2_from_file(path: &str) -> u32 {
        let lines = read_lines_from_file(path);
        let groups: Vec<_> = lines.split(|s| s.is_empty()).collect();
        let mut elves = Vec::new();
        for group in groups {
            let elf = Elf::new(
                group
                    .into_iter()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect(),
            );
            elves.push(elf);
        }

        let c = Elves::new(elves);

        c.top_three()
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1_from_file("inputs/day1.txt"), 66616);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2_from_file("inputs/day1.txt"), 199172);
    }
}
