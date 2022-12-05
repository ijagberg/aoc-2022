use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct CrateStacks {
    stacks: Vec<Vec<Crate>>,
}

impl CrateStacks {
    pub fn new(stacks_count: usize) -> Self {
        Self {
            stacks: vec![vec![]; stacks_count],
        }
    }

    pub fn add_crate(&mut self, stack: usize, crate_identifier: Crate) {
        self.stacks[stack].push(crate_identifier);
    }

    pub fn move_crate(&mut self, from: usize, to: usize) {
        if let Some(top) = self.stacks[from].pop() {
            self.stacks[to].push(top);
        }
    }

    pub fn move_crates(&mut self, from: usize, to: usize, count: usize) {
        let idx = self.stacks[from].len().saturating_sub(count);
        let mut crates = self.stacks[from].split_off(idx);

        self.stacks[to].append(&mut crates);
    }

    pub fn get_top_crates_string(&self) -> String {
        let mut s = String::new();
        for stack in &self.stacks {
            if let Some(Crate(c)) = stack.last() {
                s.push(*c);
            }
        }
        s
    }
}

#[derive(Clone, Copy)]
pub struct Crate(char);

impl Crate {
    pub fn new(c: char) -> Self {
        Self(c)
    }
}

impl Debug for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
