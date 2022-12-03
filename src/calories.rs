pub struct Elves {
    elves: Vec<Elf>,
}

impl Elves {
    pub fn new(elves: Vec<Elf>) -> Self {
        Self { elves }
    }

    pub fn elves(&self) -> &[Elf] {
        self.elves.as_ref()
    }

    pub fn highest_total_calories(&self) -> u32 {
        self.elves.iter().map(|e| e.total_calories()).max().unwrap()
    }

    pub fn top_three(&self) -> u32 {
        let mut calories: Vec<u32> = self.elves.iter().map(|e| e.total_calories()).collect();
        calories.sort();

        calories[calories.len() - 3..].into_iter().sum()
    }
}

pub struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    pub fn new(calories: Vec<u32>) -> Self {
        Self { calories }
    }

    pub fn total_calories(&self) -> u32 {
        self.calories.iter().sum()
    }
}
