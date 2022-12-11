use std::{collections::VecDeque, str::FromStr};

#[derive(Debug, Clone)]
pub struct Monkeys {
    monkeys: Vec<Monkey>,
}

impl Monkeys {
    pub fn new(monkeys: Vec<Monkey>) -> Self {
        Self { monkeys }
    }

    pub fn run_once(&mut self, worry_decrease: bool) {
        let acc_test = self.acc_test();
        for i in 0..self.monkeys.len() {
            let monkey = &mut self.monkeys[i];
            let mut throws = Vec::new();
            while let Some((item, target)) = monkey.take_turn(worry_decrease, acc_test) {
                throws.push((item, target));
            }

            for (item, target) in throws {
                self.monkeys[target].holding.push_back(item);
            }
        }
    }

    pub fn inspection_counts(&self) -> Vec<u128> {
        self.monkeys.iter().map(|m| m.total_inspections).collect()
    }

    fn acc_test(&self) -> u128 {
        self.monkeys.iter().map(|m| m.test).product()
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    id: usize,
    holding: VecDeque<Item>,
    operation: (OldOrConstant, Operand, OldOrConstant),
    test: u128,
    true_target: usize,
    false_target: usize,
    total_inspections: u128,
}

impl Monkey {
    pub fn new(
        id: usize,
        holding: Vec<u128>,
        operation: (OldOrConstant, Operand, OldOrConstant),
        test: u128,
        true_target: usize,
        false_target: usize,
    ) -> Self {
        Self {
            id,
            holding: holding.into_iter().map(|i| Item(i)).collect(),
            operation,
            test,
            true_target,
            false_target,
            total_inspections: 0,
        }
    }

    fn take_turn(&mut self, worry_decrease: bool, acc_test: u128) -> Option<(Item, usize)> {
        // println!("Monkey {}'s turn...", self.id);
        if let Some(Item(worry_value)) = self.holding.pop_front() {
            self.total_inspections += 1;
            let mut new_worry = self.get_new_value(worry_value);
            if worry_decrease {
                new_worry /= 3;
            }
            // println!(
            //     "{}: Inspecting item of value {}, new value is {}",
            //     self.id, worry_value, new_worry
            // );

            if new_worry % self.test == 0 {
                if worry_decrease {
                    Some((Item(new_worry), self.true_target))
                } else {
                    // println!(
                    //     "{}: {} is divisible by {}, throwing to {}",
                    //     self.id, new_worry, self.test, self.true_target
                    // );
                    Some((Item(new_worry % acc_test), self.true_target))
                }
            } else {
                if worry_decrease {
                    Some((Item(new_worry), self.false_target))
                } else {
                    // println!(
                    //     "{}: {} isn ot divisible by {}, throwing to {}",
                    //     self.id, new_worry, self.test, self.false_target
                    // );
                    Some((Item(new_worry % acc_test), self.false_target))
                }
            }
        } else {
            None
        }
    }

    fn get_new_value(&self, v: u128) -> u128 {
        let (l, op, r) = &self.operation;
        let left = match l {
            OldOrConstant::Old => v,
            OldOrConstant::Constant(c) => *c,
        };
        let right = match r {
            OldOrConstant::Old => v,
            OldOrConstant::Constant(c) => *c,
        };

        match op {
            Operand::Add => left + right,
            Operand::Mul => left * right,
        }
    }
}

#[derive(Debug, Clone)]
struct Item(u128);

#[derive(Debug, Clone)]
pub enum OldOrConstant {
    Old,
    Constant(u128),
}

impl FromStr for OldOrConstant {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Self::Old),
            num => Ok(Self::Constant(num.parse().map_err(|_| ())?)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operand {
    Add,
    Mul,
}

impl FromStr for Operand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            _e => Err(()),
        }
    }
}
