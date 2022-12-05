#![allow(unused)]

use std::{
    fs::File,
    io::{self, BufRead},
};

mod assignment_pairs;
mod calories;
mod crate_stack;
mod rock_paper_scissors;
mod rucksack;

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

mod day2 {
    use super::*;
    use crate::rock_paper_scissors::{RockPaperScissors, RockPaperScissorsResult};

    fn solve_part1_from_file(path: &str) -> u32 {
        let lines = read_lines_from_file(path);

        let mut total_score: u32 = 0;

        for line in lines {
            let parts: Vec<_> = line.split(' ').collect();
            let they_play = match parts[0] {
                "A" => RockPaperScissors::Rock,
                "B" => RockPaperScissors::Paper,
                "C" => RockPaperScissors::Scissors,
                e => panic!(),
            };
            let i_play = match parts[1] {
                "X" => RockPaperScissors::Rock,
                "Y" => RockPaperScissors::Paper,
                "Z" => RockPaperScissors::Scissors,
                e => panic!(),
            };

            total_score += i_play.score();

            total_score += i_play.play_against(they_play).score();
        }

        total_score
    }

    fn solve_part2_from_file(path: &str) -> u32 {
        let lines = read_lines_from_file(path);

        let mut total_score: u32 = 0;

        for line in lines {
            let parts: Vec<_> = line.split(' ').collect();
            let they_play = match parts[0] {
                "A" => RockPaperScissors::Rock,
                "B" => RockPaperScissors::Paper,
                "C" => RockPaperScissors::Scissors,
                e => panic!(),
            };
            let i_should = match parts[1] {
                "X" => RockPaperScissorsResult::Loss,
                "Y" => RockPaperScissorsResult::Draw,
                "Z" => RockPaperScissorsResult::Win,
                e => panic!(),
            };

            let i_play = they_play.result_against(i_should);

            total_score += i_play.score();

            total_score += i_play.play_against(they_play).score();
        }

        total_score
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1_from_file("inputs/day2.txt"), 11906);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2_from_file("inputs/day2.txt"), 11186);
    }
}

mod day3 {
    use super::*;
    use crate::{
        rock_paper_scissors::{RockPaperScissors, RockPaperScissorsResult},
        rucksack::Rucksack,
    };
    use std::collections::HashSet;

    fn solve_part1_from_file(path: &str) -> u32 {
        let mut priority_sum = 0;
        for line in read_lines_from_file(path) {
            let rucksack = Rucksack::new(line);
            let (comp1, comp2) = rucksack.compartments();
            let comp1: HashSet<_> = comp1.iter().copied().collect();
            let comp2: HashSet<_> = comp2.iter().copied().collect();

            let intersection: Vec<_> = comp1.intersection(&comp2).collect();
            assert_eq!(intersection.len(), 1, "{:?} \n {:?}", comp1, comp2);
            let duplicate_item = intersection[0];

            priority_sum += duplicate_item.priority();
        }
        priority_sum
    }

    fn solve_part2_from_file(path: &str) -> u32 {
        let mut priority_sum = 0;
        for chunk in read_lines_from_file(path).chunks(3) {
            let r1 = Rucksack::new(chunk[0].clone());
            let r2 = Rucksack::new(chunk[1].clone());
            let r3 = Rucksack::new(chunk[2].clone());
            let rucksack1: HashSet<_> = r1.items().into_iter().collect();
            let rucksack2: HashSet<_> = r2.items().into_iter().collect();
            let rucksack3: HashSet<_> = r3.items().into_iter().collect();

            let intersection_1_2: HashSet<_> =
                rucksack1.intersection(&rucksack2).copied().collect();
            let intersection: Vec<_> = intersection_1_2.intersection(&rucksack3).collect();
            assert_eq!(intersection.len(), 1);

            priority_sum += intersection[0].priority();
        }

        priority_sum
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1_from_file("inputs/day3.txt"), 7903);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2_from_file("inputs/day3.txt"), 2548);
    }
}

mod day4 {
    use super::*;
    use crate::assignment_pairs::AssignmentPair;

    fn solve_part1_from_file(path: &str) -> u32 {
        let mut overlaps = 0;
        for line in read_lines_from_file(path) {
            let parts: Vec<_> = line.split(',').collect();
            assert_eq!(parts.len(), 2);
            let left_parts: Vec<usize> = parts[0].split('-').map(|p| p.parse().unwrap()).collect();
            let right_parts: Vec<usize> = parts[1].split('-').map(|p| p.parse().unwrap()).collect();

            assert_eq!(left_parts.len(), 2);
            assert_eq!(right_parts.len(), 2);

            let pair = AssignmentPair::new(
                (left_parts[0], left_parts[1]),
                (right_parts[0], right_parts[1]),
            );

            if pair.overlaps() {
                overlaps += 1;
            }
        }

        overlaps
    }

    fn solve_part2_from_file(path: &str) -> u32 {
        let mut overlaps = 0;
        for line in read_lines_from_file(path) {
            let parts: Vec<_> = line.split(',').collect();
            assert_eq!(parts.len(), 2);
            let left_parts: Vec<usize> = parts[0].split('-').map(|p| p.parse().unwrap()).collect();
            let right_parts: Vec<usize> = parts[1].split('-').map(|p| p.parse().unwrap()).collect();

            assert_eq!(left_parts.len(), 2);
            assert_eq!(right_parts.len(), 2);

            let pair = AssignmentPair::new(
                (left_parts[0], left_parts[1]),
                (right_parts[0], right_parts[1]),
            );

            if pair.partially_overlaps() {
                overlaps += 1;
            }
        }

        overlaps
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1_from_file("inputs/day4.txt"), 567);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2_from_file("inputs/day4.txt"), 907);
    }
}

mod day5 {
    use super::*;
    use crate::crate_stack::{Crate, CrateStacks};

    fn parse_start_from_file(start_path: &str) -> CrateStacks {
        let start_lines = read_lines_from_file(start_path);
        let stacks_count = start_lines.last().unwrap().split_ascii_whitespace().count();
        let mut crate_stacks = CrateStacks::new(stacks_count);

        for (line_idx, line) in start_lines.into_iter().rev().skip(1).enumerate() {
            for (stack_idx, crate_identifier) in line.chars().skip(1).step_by(4).enumerate() {
                if stack_idx >= stacks_count {
                    break;
                }
                if crate_identifier.is_alphabetic() {
                    crate_stacks.add_crate(stack_idx, Crate::new(crate_identifier));
                }
            }
        }
        crate_stacks
    }

    fn solve_part1_from_files(start_path: &str, instructions_path: &str) -> String {
        let mut crate_stacks = parse_start_from_file(start_path);

        let instruction_lines = read_lines_from_file(instructions_path);

        for line in instruction_lines {
            // move 16 from 7 to 2
            let parts: Vec<_> = line.split(' ').collect();
            let crate_count: usize = parts[1].parse().unwrap();
            let from_stack: usize = parts[3].parse().unwrap();
            let to_stack: usize = parts[5].parse().unwrap();
            for _ in 0..crate_count {
                crate_stacks.move_crate(from_stack - 1, to_stack - 1);
            }
        }

        crate_stacks.get_top_crates_string()
    }

    fn solve_part2_from_files(start_path: &str, instructions_path: &str) -> String {
        let mut crate_stacks = parse_start_from_file(start_path);

        let instruction_lines = read_lines_from_file(instructions_path);

        for line in instruction_lines {
            // move 16 from 7 to 2
            let parts: Vec<_> = line.split(' ').collect();
            let crate_count: usize = parts[1].parse().unwrap();
            let from_stack: usize = parts[3].parse().unwrap();
            let to_stack: usize = parts[5].parse().unwrap();
            crate_stacks.move_crates(from_stack - 1, to_stack - 1, crate_count);
        }

        crate_stacks.get_top_crates_string()
    }

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1_from_files("inputs/day5_start.txt", "inputs/day5_instructions.txt"),
            "WSFTMRHPP"
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            solve_part2_from_files("inputs/day5_start.txt", "inputs/day5_instructions.txt"),
            "GSLCMFBRP"
        );
    }

    #[test]
    fn example1() {
        assert_eq!(
            solve_part2_from_files(
                "inputs/day5_example_start.txt",
                "inputs/day5_example_instructions.txt"
            ),
            "MCD"
        );
    }
}
