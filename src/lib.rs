#![allow(unused)]

use std::{
    fs::File,
    io::{self, BufRead},
};

mod assignment_pairs;
mod calories;
mod crate_stack;
mod file_system;
mod marker;
mod rock_paper_scissors;
mod rope;
mod rucksack;
mod trees;

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
            let elf = Elf::new(group.iter().map(|s| s.parse::<u32>().unwrap()).collect());
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
            let elf = Elf::new(group.iter().map(|s| s.parse::<u32>().unwrap()).collect());
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
            let rucksack1: HashSet<_> = r1.items().iter().collect();
            let rucksack2: HashSet<_> = r2.items().iter().collect();
            let rucksack3: HashSet<_> = r3.items().iter().collect();

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
}

mod day6 {
    use super::*;
    use crate::marker::Marker;

    fn solve_from_file(path: &str, marker_len: usize) -> usize {
        let mut lines = read_lines_from_file(path);
        assert_eq!(lines.len(), 1);
        let line = lines.remove(0);

        let marker = Marker::new(line);

        *marker.marker_indices(marker_len).first().unwrap()
    }

    fn solve_part1_from_file(path: &str) -> usize {
        solve_from_file(path, 4)
    }

    fn solve_part2_from_file(path: &str) -> usize {
        solve_from_file(path, 14)
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1_from_file("inputs/day6.txt"), 1343);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2_from_file("inputs/day6.txt"), 2193);
    }
}

mod day7 {
    use super::*;

    fn solve_part1_from_file(path: &str) -> u32 {
        let filesys = file_system::traverse_file_system(&read_lines_from_file(path));

        let sizes = filesys.dir_sizes();
        sizes
            .into_iter()
            .filter_map(|(dir, size)| if size <= 100000 { Some(size) } else { None })
            .sum()
    }

    fn solve_part2_from_file(path: &str) -> u32 {
        let filesys = file_system::traverse_file_system(&read_lines_from_file(path));

        let sizes = filesys.dir_sizes();
        let size_of_root = sizes["/"];
        let unused_space = 70000000 - size_of_root;
        let target = 30000000 - unused_space;

        println!("size of root is {size_of_root}, which means there is {unused_space} unused space, which means we need to free up at least {target}");

        let mut best = None;
        for (dir, size) in sizes {
            if size < target {
                continue;
            }
            let diff = size.abs_diff(target);
            if best.map(|(best_diff, _)| diff < best_diff).unwrap_or(true) {
                best = Some((diff, size));
            }
        }
        best.unwrap().1
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1_from_file("inputs/day7.txt"), 1315285);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2_from_file("inputs/day7.txt"), 9847279);
    }
}

mod day8 {
    use super::*;
    use crate::trees::{Tree, Trees};
    use simple_grid::Grid;

    fn populate_grid_from_file(path: &str) -> Trees {
        let lines = read_lines_from_file(path);
        let width = lines[0].len();
        let height = lines.len();
        let data: Vec<_> = lines
            .join("")
            .chars()
            .map(|c| Tree::new(c.to_digit(10).unwrap()))
            .collect();

        let grid = Grid::new(width, height, data);
        Trees::new(grid)
    }

    fn solve_part1_from_file(path: &str) -> usize {
        let trees = populate_grid_from_file(path);
        trees.count_visible()
    }

    fn solve_part2_from_file(path: &str) -> u32 {
        let trees = populate_grid_from_file(path);
        trees.best_scenic_score()
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1_from_file("inputs/day8_example.txt"), 21);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1_from_file("inputs/day8.txt"), 1785);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2_from_file("inputs/day8_example.txt"), 8);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2_from_file("inputs/day8.txt"), 345168);
    }
}

mod day9 {
    use super::*;
    use crate::rope::{Idx, Motion, RopeSim};

    fn solve_part1_from_file(path: &str) -> usize {
        let mut rope_sim = RopeSim::new(vec![Idx::new(0, 0); 2]);

        let motions: Vec<_> = read_lines_from_file(path)
            .iter()
            .map(|p| p.parse::<Motion>().unwrap())
            .collect();

        rope_sim.tail_visits(&motions)
    }

    fn solve_part2_from_file(path: &str) -> usize {
        let mut rope_sim = RopeSim::new(vec![Idx::new(0, 0); 10]);

        let motions: Vec<_> = read_lines_from_file(path)
            .iter()
            .map(|p| p.parse::<Motion>().unwrap())
            .collect();

        rope_sim.tail_visits(&motions)
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1_from_file("inputs/day9_example1.txt"), 13);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1_from_file("inputs/day9.txt"), 6023);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2_from_file("inputs/day9_example1.txt"), 1);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(solve_part2_from_file("inputs/day9_example2.txt"), 36);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2_from_file("inputs/day9.txt"), 2533);
    }
}
