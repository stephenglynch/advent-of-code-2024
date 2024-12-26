use heapless::{Vec, FnvIndexSet};
use crate::util::{count_lines, count_line_len};

const INPUT_CONTENT: &str = include_str!("../data/day6/input.txt");
const GRID_Y_MAX: usize = count_lines(INPUT_CONTENT);
const GRID_X_MAX: usize = count_line_len(INPUT_CONTENT);
const GRID_SIZE: usize = GRID_Y_MAX * GRID_X_MAX;
const UNIQUE_POSITIONS_SIZE: usize = 8192;

#[derive(Copy, Clone, Debug, PartialEq)]
enum GridObject {
    Empty, Obstruction, Guard
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    North, East, South, West
}

type Point = (usize, usize);
type UniquePositions = FnvIndexSet<Point, UNIQUE_POSITIONS_SIZE>;

struct Grid {
    grid: Vec<GridObject, GRID_SIZE>,
    direction: Direction,
    guard_x: usize,
    guard_y: usize,
    unique_positions: UniquePositions
}

impl Grid {
    fn new(text: &str) -> Self {
        // Parse grid
        let mut grid = Vec::new();
        for l in text.lines() {
            for c in l.bytes() {
                let p = match c {
                    b'#' => GridObject::Obstruction,
                    b'^' => GridObject::Guard,
                    _ => GridObject::Empty
                };
                let _ = grid.push(p);
            }
        }

        // Find guard start position
        let i = grid.iter().position(|&x| x == GridObject::Guard).unwrap();
        let y = i / GRID_X_MAX;
        let x = i % GRID_X_MAX;

        Grid {
            grid: grid,
            direction: Direction::North,
            guard_x: x,
            guard_y: y,
            unique_positions: FnvIndexSet::new()
        }
    }

    fn step_north(&self) -> Option<Point> {
        if let Some(y) = self.guard_y.checked_sub(1) {
            Some((self.guard_x, y))
        } else {
            None
        }
    }

    fn step_east(&self) -> Option<Point> {
        if self.guard_x + 1 < GRID_X_MAX {
            Some((self.guard_x + 1, self.guard_y))
        } else {
            None
        }
    }

    fn step_south(&self) -> Option<Point> {
        if self.guard_y + 1 < GRID_Y_MAX {
            Some((self.guard_x, self.guard_y + 1))
        } else {
            None
        }
    }

    fn step_west(&self) -> Option<Point> {
        if let Some(x) = self.guard_x.checked_sub(1) {
            Some((x, self.guard_y))
        } else {
            None
        }
    }

    fn rotate(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        }
    }

    fn step(&mut self) -> bool {
        if let Some(p) = match self.direction {
            Direction::North => self.step_north(),
            Direction::East => self.step_east(),
            Direction::South => self.step_south(),
            Direction::West => self.step_west()
        } {
            match self.get_point(p) {
                GridObject::Obstruction => {
                    self.rotate();
                }
                _ => {
                    let _ = self.unique_positions.insert(p);
                    (self.guard_x, self.guard_y) = p;
                }
            }
            return false;
        } else {
            return true;
        }
    }

    fn get_point(&self, p: Point) -> GridObject {
        let (x, y) = p;
        self.grid[x + y * GRID_X_MAX]
    }
}

pub fn answer() -> usize {
    let mut grid = Grid::new(INPUT_CONTENT);
    while !grid.step() {}
    let answer = grid.unique_positions.len();
    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_CONTENT: &str = include_str!("../data/day6/example.txt");

    #[test]
    fn test_example_answer() {
        let mut grid = Grid::new(EXAMPLE_CONTENT);
        while !grid.step() {}
        let answer = grid.unique_positions.len();
        assert_eq!(answer, 41);
    }

    #[test]
    fn test_answer() {
        println!("answer = {}", answer());
    }
}