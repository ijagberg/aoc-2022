use std::collections::{HashSet, VecDeque};

use simple_grid::{Grid, GridIndex};

pub struct Hills {
    grid: Grid<Height>,
}

impl Hills {
    pub fn new(width: usize, height: usize, data: Vec<char>) -> Self {
        Self {
            grid: Grid::new(width, height, data.into_iter().map(|h| Height(h)).collect()),
        }
    }

    pub fn find_shortest_path(&self) -> Option<u32> {
        let start = self.find_starting_pos()?;

        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();
        to_visit.push_back((0, start));
        visited.insert(start);

        while let Some((steps, current_idx)) = to_visit.pop_front() {
            visited.insert(current_idx);
            let current = &self.grid[current_idx];
            println!("visiting {} at {}", current.0, current_idx);
            if current.is_target() {
                return Some(steps);
            }

            for n in self.neighbors(current_idx) {
                let has_visited = visited.contains(&n);
                if !has_visited {
                    visited.insert(n);
                    to_visit.push_back((steps + 1, n));
                }
            }
        }

        None
    }

    pub fn find_shortest_hike(&self) -> Option<u32> {
        let start = self.find_target_pos()?;
        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();
        to_visit.push_back((0, start));
        visited.insert(start);

        while let Some((steps, current_idx)) = to_visit.pop_front() {
            visited.insert(current_idx);
            let current = &self.grid[current_idx];
            if current.is_hike_start() {
                return Some(steps);
            }

            for n in self.inverse_neighbors(current_idx) {
                let has_visited = visited.contains(&n);
                if !has_visited {
                    visited.insert(n);
                    to_visit.push_back((steps + 1, n));
                }
            }
        }

        None
    }

    fn neighbors(&self, idx: GridIndex) -> Vec<GridIndex> {
        let mut neighbors = Vec::new();
        // up
        if idx.row() > 0 {
            let up_idx = GridIndex::new(idx.column(), idx.row() - 1);
            if self.grid[idx].can_walk_to(self.grid[up_idx]) {
                neighbors.push(up_idx);
            }
        }
        // down
        if idx.row() < self.grid.height() - 1 {
            let down_idx = GridIndex::new(idx.column(), idx.row() + 1);
            if self.grid[idx].can_walk_to(self.grid[down_idx]) {
                neighbors.push(down_idx);
            }
        }
        // left
        if idx.column() > 0 {
            let left_idx = GridIndex::new(idx.column() - 1, idx.row());
            if self.grid[idx].can_walk_to(self.grid[left_idx]) {
                neighbors.push(left_idx);
            }
        }
        // right
        if idx.column() < self.grid.width() - 1 {
            let right_idx = GridIndex::new(idx.column() + 1, idx.row());
            if self.grid[idx].can_walk_to(self.grid[right_idx]) {
                neighbors.push(right_idx);
            }
        }

        neighbors
    }

    fn inverse_neighbors(&self, idx: GridIndex) -> Vec<GridIndex> {
        let mut neighbors = Vec::new();
        // up
        if idx.row() > 0 {
            let up_idx = GridIndex::new(idx.column(), idx.row() - 1);
            if self.grid[up_idx].can_walk_to(self.grid[idx]) {
                neighbors.push(up_idx);
            }
        }
        // down
        if idx.row() < self.grid.height() - 1 {
            let down_idx = GridIndex::new(idx.column(), idx.row() + 1);
            if self.grid[down_idx].can_walk_to(self.grid[idx]) {
                neighbors.push(down_idx);
            }
        }
        // left
        if idx.column() > 0 {
            let left_idx = GridIndex::new(idx.column() - 1, idx.row());
            if self.grid[left_idx].can_walk_to(self.grid[idx]) {
                neighbors.push(left_idx);
            }
        }
        // right
        if idx.column() < self.grid.width() - 1 {
            let right_idx = GridIndex::new(idx.column() + 1, idx.row());
            if self.grid[right_idx].can_walk_to(self.grid[idx]) {
                neighbors.push(right_idx);
            }
        }

        neighbors
    }

    fn find_starting_pos(&self) -> Option<GridIndex> {
        for idx in self.grid.indices() {
            let height = &self.grid[idx];
            if height.is_start() {
                return Some(idx);
            }
        }
        None
    }

    fn find_target_pos(&self) -> Option<GridIndex> {
        for idx in self.grid.indices() {
            let height = &self.grid[idx];
            if height.is_target() {
                return Some(idx);
            }
        }
        None
    }
}

#[derive(Clone, Copy)]
struct Height(char);

impl Height {
    pub fn is_start(&self) -> bool {
        self.0 == 'S'
    }

    pub fn is_hike_start(&self) -> bool {
        self.value() == ('a' as u8 - 'a' as u8)
    }

    pub fn is_target(&self) -> bool {
        self.0 == 'E'
    }

    pub fn value(&self) -> u8 {
        if self.is_target() {
            'z' as u8 - 'a' as u8
        } else if self.is_start() {
            'a' as u8 - 'a' as u8
        } else {
            self.0 as u8 - 'a' as u8
        }
    }

    pub fn can_walk_to(&self, other: Self) -> bool {
        self.value() + 1 >= other.value()
    }
}
