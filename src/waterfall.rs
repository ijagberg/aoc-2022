use std::collections::HashMap;

pub struct Waterfall {
    grid: HashMap<Coord, Tile>,
    bottom: isize,
    top: isize,
    left: isize,
    right: isize,
    has_floor: bool,
}

impl Waterfall {
    pub fn new(has_floor: bool) -> Self {
        Self {
            grid: HashMap::new(),
            bottom: isize::MIN,
            top: isize::MAX,
            left: isize::MAX,
            right: isize::MIN,
            has_floor,
        }
    }

    pub fn add_rock_line(&mut self, from: Coord, to: Coord) {
        if from.x() == to.x() {
            // vertical
            let (from_y, to_y) = if from.y() <= to.y() {
                (from.y(), to.y())
            } else {
                (to.y(), from.y())
            };
            for y in from_y..=to_y {
                self.grid.insert(Coord(from.x(), y), Tile::Rock);
                if y > self.bottom {
                    self.bottom = y;
                }
                if y < self.top {
                    self.top = y;
                }
                if from.x() < self.left {
                    self.left = from.x();
                }
                if from.x() > self.right {
                    self.right = from.x();
                }
            }
        } else if from.y() == to.y() {
            // horizontal
            let (from_x, to_x) = if from.x() <= to.x() {
                (from.x(), to.x())
            } else {
                (to.x(), from.x())
            };
            for x in from_x..=to_x {
                self.grid.insert(Coord(x, from.y()), Tile::Rock);
                if from.y() > self.bottom {
                    self.bottom = from.y();
                }
                if from.y() < self.top {
                    self.top = from.y();
                }
                if x < self.left {
                    self.left = x;
                }
                if x > self.right {
                    self.right = x;
                }
            }
        } else {
            panic!("{:?} to {:?} is an invalid line", from, to);
        }
    }

    pub fn simulate_sand(&mut self, mut coord: Coord) -> Option<SimulationResult> {
        match self.get_tile(coord) {
            Tile::Rock | Tile::Sand => return None,
            _ => (),
        }
        loop {
            if !self.has_floor && coord.y() > self.bottom {
                // falling into abyss
                return Some(SimulationResult::Abyss(coord.x()));
            }
            // check below
            let down = Coord(coord.x(), coord.y() + 1);
            match self.get_tile(down) {
                Tile::Rock | Tile::Sand => {
                    // check down_left
                    let down_left = Coord(down.x() - 1, down.y());
                    match self.get_tile(down_left) {
                        Tile::Rock | Tile::Sand => {
                            // check down_right
                            let down_right = Coord(down.x() + 1, down.y());
                            match self.get_tile(down_right) {
                                Tile::Rock | Tile::Sand => {
                                    // come to rest here
                                    self.grid.insert(coord, Tile::Sand);
                                    if coord.y() < self.top {
                                        self.top = coord.y() - 1;
                                    }
                                    if coord.x() < self.left {
                                        self.left = coord.x() - 1;
                                    }
                                    if coord.x() > self.right {
                                        self.right = coord.x() + 1;
                                    }
                                    return Some(SimulationResult::Resting(coord));
                                }
                                Tile::Air => {
                                    coord = down_right;
                                    continue;
                                }
                            }
                        }
                        Tile::Air => {
                            coord = down_left;
                            continue;
                        }
                    }
                }
                Tile::Air => {
                    coord = down;
                    continue;
                }
            }
        }
    }

    pub fn nice_string(&self) -> String {
        let mut buf = String::new();
        for y in self.top - 5..=self.bottom + 5 {
            for x in self.left - 5..=self.right + 5 {
                match self.get_tile(Coord::new(x, y)) {
                    Tile::Rock => buf.push('#'),
                    Tile::Sand => buf.push('O'),
                    Tile::Air => buf.push('.'),
                }
            }
            buf.push('\n');
        }
        buf
    }

    fn get_tile(&self, coord: Coord) -> Tile {
        if self.has_floor && coord.y() == self.bottom + 2 {
            Tile::Rock
        } else {
            self.grid.get(&coord).copied().unwrap_or(Tile::Air)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Rock,
    Sand,
    Air,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct Coord(isize, isize);

impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> isize {
        self.0
    }

    pub fn y(&self) -> isize {
        self.1
    }
}

pub enum SimulationResult {
    Resting(Coord),
    Abyss(isize),
}
