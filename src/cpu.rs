use std::{fmt::Display, str::FromStr};

use simple_grid::Grid;

pub struct Cpu {
    register: i64,
}

impl Cpu {
    pub fn new() -> Self {
        Self { register: 1 }
    }

    pub fn run(&mut self, instructions: &[Instruction]) -> (Grid<char>, i64) {
        let mut grid_data = vec!['.'; 40 * 6];
        let mut get_instruction = false;
        let mut instructions = instructions.iter();
        let mut instruction_progress = 0;
        let mut current_instruction = instructions.next().unwrap();
        let mut sum_of_signal_strength = 0;
        'run: for cycle in 1.. {
            if get_instruction {
                if let Some(instr) = instructions.next() {
                    get_instruction = false;
                    current_instruction = instr;
                    instruction_progress = 0;
                } else {
                    break 'run;
                }
            }
            instruction_progress += 1;

            if self.crt_overlaps(cycle - 1) {
                grid_data[cycle - 1] = '#';
            }

            if Self::is_interesting_cycle(cycle) {
                let signal_strength = cycle as i64 * self.register;
                sum_of_signal_strength += signal_strength;
            }

            if instruction_progress == current_instruction.cycle_length() {
                match current_instruction {
                    Instruction::Add(v) => self.register += v,
                    Instruction::NoOp => (),
                }
                get_instruction = true;
            }
        }
        (Grid::new(40, 6, grid_data), sum_of_signal_strength)
    }

    fn is_interesting_cycle(cycle: usize) -> bool {
        cycle >= 20 && (cycle - 20) % 40 == 0
    }

    fn crt_overlaps(&self, position: usize) -> bool {
        (self.register - 1..=self.register + 1).contains(&(position as i64 % 40))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Add(i64),
    NoOp,
}

impl Instruction {
    fn cycle_length(&self) -> u32 {
        match self {
            Instruction::Add(_) => 2,
            Instruction::NoOp => 1,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Add(v) => write!(f, "addx {}", v),
            Instruction::NoOp => write!(f, "noop"),
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();
        match parts[0] {
            "addx" => Ok(Self::Add(parts[1].parse().map_err(|_| ())?)),
            "noop" => Ok(Self::NoOp),
            _e => Err(()),
        }
    }
}
