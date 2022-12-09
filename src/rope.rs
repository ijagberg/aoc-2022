use std::{collections::HashSet, fmt::Display, str::FromStr};

pub struct RopeSim {
    positions: Vec<Idx>,
}

impl RopeSim {
    pub fn new(positions: Vec<Idx>) -> Self {
        if positions.is_empty() {
            panic!()
        }
        Self { positions }
    }

    fn tail_pos(&self) -> Idx {
        self.positions[self.positions.len() - 1]
    }

    pub fn tail_visits(&mut self, motions: &[Motion]) -> usize {
        let mut visited = HashSet::new();
        visited.insert(self.tail_pos());

        for motion in motions {
            let step_diff = motion.direction.step_diff();
            for step in 0..motion.steps {
                for i in 0..self.positions.len() {
                    let pos = self.positions[i];
                    if i == 0 {
                        // head
                        self.positions[0] = pos.apply_step_diff(step_diff);
                    } else {
                        let in_front = self.positions[i - 1];
                        let new_pos = pos.move_towards(in_front);
                        self.positions[i] = new_pos;
                    }
                }
                visited.insert(self.tail_pos());
            }
        }
        Self::tail_display(&visited);
        visited.len()
    }

    fn big_display(&self) {
        let mut s = String::new();
        for row in (-15..=15) {
            for col in (-20..=15) {
                let idx = Idx::new(col, row);
                'find: {
                    for (i, pos) in self.positions.iter().enumerate() {
                        if *pos == idx {
                            s.push_str(&i.to_string());
                            break 'find;
                        }
                    }
                    s.push('.');
                }
            }
            s.push('\n');
        }
        println!("{}", s);
    }

    fn tail_display(positions: &HashSet<Idx>) {
        let mut s = String::new();
        for row in (-15..=15) {
            for col in (-15..=15) {
                match positions.contains(&Idx::new(col, row)) {
                    true => s.push('#'),
                    false => s.push('.'),
                }
            }
            s.push('\n');
        }
        println!("{}", s);
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Idx {
    column: isize,
    row: isize,
}

impl Idx {
    pub fn new(column: isize, row: isize) -> Self {
        Self { column, row }
    }

    fn move_towards(&self, target: Self) -> Self {
        if Self::are_touching(*self, target) {
            *self
        } else {
            let col_diff = target.column - self.column;
            let row_diff = target.row - self.row;

            match (col_diff.abs(), row_diff.abs()) {
                (0, 2) => {
                    // move vertically 1 step
                    self.apply_step_diff((0, row_diff.signum()))
                }
                (2, 0) => {
                    // move horizontally 1 step
                    self.apply_step_diff((col_diff.signum(), 0))
                }
                (1, 2) | (2, 1) | (2, 2) => {
                    // move diagonally towards the target
                    self.apply_step_diff((col_diff.signum(), row_diff.signum()))
                }
                invalid => {
                    panic!(
                        "invalid diff: {:?} between {} and {}",
                        invalid, self, target
                    )
                }
            }
        }
    }

    fn are_touching(a: Self, b: Self) -> bool {
        let col_diff = a.column.abs_diff(b.column);
        let row_diff = a.row.abs_diff(b.row);
        // diagonal special case
        (col_diff == 0 || col_diff == 1) && (row_diff == 0 || row_diff == 1)
    }

    fn apply_step_diff(&self, diff: (isize, isize)) -> Self {
        Self::new(self.column + diff.0, self.row + diff.1)
    }
}

impl Display for Idx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.column, self.row)
    }
}

pub struct Motion {
    direction: Direction,
    steps: usize,
}

impl Motion {
    fn new(direction: Direction, steps: usize) -> Self {
        Self { direction, steps }
    }
}

impl FromStr for Motion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();
        assert_eq!(parts.len(), 2);
        let dir = Direction::from_str(parts[0])?;
        let steps = parts[1].parse().map_err(|_| ())?;
        Ok(Self::new(dir, steps))
    }
}

impl Display for Motion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.direction, self.steps)
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn step_diff(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            e => return Err(()),
        })
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => "U",
                Direction::Down => "D",
                Direction::Left => "L",
                Direction::Right => "R",
            }
        )
    }
}
