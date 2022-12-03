#![allow(unused)]

use std::{
    fs::File,
    io::{self, BufRead},
};

mod calories;
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
    use std::collections::HashSet;

    use super::*;
    use crate::{
        rock_paper_scissors::{RockPaperScissors, RockPaperScissorsResult},
        rucksack::Rucksack,
    };

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
