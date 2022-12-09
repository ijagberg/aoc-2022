pub struct Rucksack {
    items: Vec<Item>,
}

impl Rucksack {
    pub fn new(items: String) -> Rucksack {
        let items: Vec<Item> = items.trim().chars().map(Item::new).collect();

        if items.len() % 2 != 0 {
            panic!()
        }

        Self { items }
    }

    pub fn compartments(&self) -> (&[Item], &[Item]) {
        let midpoint = self.items.len() / 2;
        (&self.items[..midpoint], &self.items[midpoint..])
    }

    pub fn items(&self) -> &[Item] {
        self.items.as_ref()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Item {
    c: char,
}

impl Item {
    pub fn new(c: char) -> Self {
        if !c.is_ascii_alphabetic() {
            panic!()
        }
        Self { c }
    }

    pub fn priority(&self) -> u32 {
        if self.c.is_lowercase() {
            self.c as u32 - 'a' as u32 + 1
        } else {
            self.c as u32 - 'A' as u32 + 27
        }
    }
}

impl core::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.c)
    }
}

mod tests {
    use super::*;

    #[test]
    fn priority_test() {
        let item = Item::new('a');
        assert_eq!(item.priority(), 1);
        let item = Item::new('z');
        assert_eq!(item.priority(), 26);

        let item = Item::new('A');
        assert_eq!(item.priority(), 27);
        let item = Item::new('Z');
        assert_eq!(item.priority(), 52);
    }
}
