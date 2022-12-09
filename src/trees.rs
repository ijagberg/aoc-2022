use simple_grid::{Grid, GridIndex};
use std::{collections::HashSet, fmt::Display};

pub struct Trees {
    grid: Grid<Tree>,
}

impl Trees {
    pub fn new(grid: Grid<Tree>) -> Self {
        Self { grid }
    }

    pub fn best_scenic_score(&self) -> u32 {
        let mut best = None;
        for idx in self.grid.indices() {
            let score = self.scenic_score(idx.column(), idx.row());
            if best
                .map(|(_idx, best_score)| score > best_score)
                .unwrap_or(true)
            {
                best = Some((idx, score));
            }
        }

        let (best_idx, best_score) = best.unwrap();

        best_score
    }

    fn scenic_score(&self, tree_column: usize, tree_row: usize) -> u32 {
        let tree_height = self.grid[(tree_column, tree_row)].height();

        // look up
        let mut up_score = 0;
        for row in (0..tree_row).rev() {
            let idx = GridIndex::new(tree_column, row);
            let height_at = self.grid[idx].height();
            up_score += 1;
            if height_at >= tree_height {
                break;
            }
        }

        // look down
        let mut down_score = 0;
        for row in (tree_row + 1..self.grid.height()) {
            let idx = GridIndex::new(tree_column, row);
            let height_at = self.grid[idx].height();
            down_score += 1;
            if height_at >= tree_height {
                break;
            }
        }

        // look left
        let mut left_score = 0;
        for column in (0..tree_column).rev() {
            let idx = GridIndex::new(column, tree_row);
            let height_at = self.grid[idx].height();
            left_score += 1;
            if height_at >= tree_height {
                break;
            }
        }

        // look right
        let mut right_score = 0;
        for column in (tree_column + 1..self.grid.width()) {
            let idx = GridIndex::new(column, tree_row);
            let height_at = self.grid[idx].height();
            right_score += 1;
            if height_at >= tree_height {
                break;
            }
        }

        let score = up_score * down_score * left_score * right_score;

        score
    }

    pub fn count_visible(&self) -> usize {
        let mut visible = HashSet::new();

        for row in self.grid.rows() {
            // left to right
            let mut tallest_so_far = None;
            for column in self.grid.columns() {
                let idx = GridIndex::new(column, row);
                let tree = &self.grid[idx];
                if tallest_so_far
                    .map(|tallest| tree.height() > tallest)
                    .unwrap_or(true)
                {
                    visible.insert(idx);
                    tallest_so_far = Some(tree.height());
                }
            }
            // right to left
            let mut tallest_so_far = None;
            for column in self.grid.columns().rev() {
                let idx = GridIndex::new(column, row);
                let tree = &self.grid[idx];
                if tallest_so_far
                    .map(|tallest| tree.height() > tallest)
                    .unwrap_or(true)
                {
                    visible.insert(idx);
                    tallest_so_far = Some(tree.height());
                }
            }
        }

        for column in self.grid.columns() {
            // up to down
            let mut tallest_so_far = None;
            for row in self.grid.rows() {
                let idx = GridIndex::new(column, row);
                let tree = &self.grid[idx];
                if tallest_so_far
                    .map(|tallest| tree.height() > tallest)
                    .unwrap_or(true)
                {
                    visible.insert(idx);
                    tallest_so_far = Some(tree.height());
                }
            }
            // down to up
            let mut tallest_so_far = None;
            for row in self.grid.rows().rev() {
                let idx = GridIndex::new(column, row);
                let tree = &self.grid[idx];
                if tallest_so_far
                    .map(|tallest| tree.height() > tallest)
                    .unwrap_or(true)
                {
                    visible.insert(idx);
                    tallest_so_far = Some(tree.height());
                }
            }
        }
        visible.len()
    }
}

pub struct Tree(u32);

impl Tree {
    pub fn new(height: u32) -> Self {
        Self(height)
    }

    fn height(&self) -> u32 {
        self.0
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.height())
    }
}
