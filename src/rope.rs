use std::{collections::HashSet, str::FromStr};

pub struct RopeSim {
    head_pos: Idx,
    tail_pos: Idx,
}

impl RopeSim {
    pub fn new() -> Self {
        Self {
            head_pos: Idx::new(0, 0),
            tail_pos: Idx::new(0, 0),
        }
    }

    pub fn tail_visits(&mut self, motions: &[Motion]) -> usize {
        let mut visited = HashSet::new();
        visited.insert(self.tail_pos);

        for motion in motions {
            let step_diff = motion.direction.step_diff();
            for _ in 0..motion.steps {
                let cur_head_pos = self.head_pos;
                let cur_tail_pos = self.tail_pos;

                let new_head_pos = cur_head_pos.apply_step_diff(step_diff);

                let new_tail_pos = if !Idx::are_touching(new_head_pos, cur_tail_pos) {
                    cur_head_pos
                } else {
                    cur_tail_pos
                };

                self.head_pos = new_head_pos;
                self.tail_pos = new_tail_pos;
                visited.insert(self.tail_pos);
            }
        }
        visited.len()
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Idx {
    column: isize,
    row: isize,
}

impl Idx {
    fn new(column: isize, row: isize) -> Self {
        Self { column, row }
    }

    fn are_touching(a: Self, b: Self) -> bool {
        let col_diff = a.column.abs_diff(b.column);
        let row_diff = a.row.abs_diff(b.row);
        // diagonal special case
        if col_diff == 1 && row_diff == 1 {
            true
        } else if col_diff == 1 && row_diff == 0 {
            true
        } else if col_diff == 0 && row_diff == 1 {
            true
        } else if col_diff == 0 && row_diff == 0 {
            true
        } else {
            false
        }
    }

    fn apply_step_diff(&self, diff: (isize, isize)) -> Self {
        Self::new(self.column + diff.0, self.row + diff.1)
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
