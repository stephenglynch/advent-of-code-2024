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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North, East, South, West
}

type Point = (usize, usize);
type UniquePositions = FnvIndexSet<Point, UNIQUE_POSITIONS_SIZE>;
type UniquePositionDirections = FnvIndexSet<(Point, Direction), UNIQUE_POSITIONS_SIZE>;

struct Grid {
    grid: Vec<GridObject, GRID_SIZE>,
    grid_x_max: usize,
    grid_y_max: usize,
    guard_direction: Direction,
    guard_init_x: usize,
    guard_init_y: usize,
    guard_x: usize,
    guard_y: usize,
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

        let grid_x_max = count_lines(text);
        let grid_y_max = count_line_len(text);

        // Find guard start position
        let i = grid.iter().position(|&x| x == GridObject::Guard).unwrap();
        let y = i / grid_x_max;
        let x = i % grid_y_max;

        Grid {
            grid: grid,
            grid_x_max: grid_x_max,
            grid_y_max: grid_y_max,
            guard_direction: Direction::North,
            guard_init_x: x,
            guard_init_y: y,
            guard_x: x,
            guard_y: y,
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
        if self.guard_x + 1 < self.grid_x_max {
            Some((self.guard_x + 1, self.guard_y))
        } else {
            None
        }
    }

    fn step_south(&self) -> Option<Point> {
        if self.guard_y + 1 < self.grid_y_max {
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
        self.guard_direction = match self.guard_direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        }
    }

    fn step(&mut self) -> bool {
        if let Some(p) = match self.guard_direction {
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
                    (self.guard_x, self.guard_y) = p;
                }
            }
            false
        } else {
            true
        }
    }

    fn get_point(&self, p: Point) -> GridObject {
        let (x, y) = p;
        self.grid[x + y * self.grid_x_max]
    }

    fn set_point(&mut self, p: Point, o: GridObject) {
        let (x, y) = p;
        self.grid[x + y * self.grid_x_max] = o;
    }

    fn reset_guard(&mut self) {
        self.guard_x = self.guard_init_x;
        self.guard_y = self.guard_init_y;
        self.guard_direction = Direction::North;
    }

}

pub fn answer(text: &str) -> usize {
    let mut grid = Grid::new(text);

    // Complete patrol first
    let mut initial_patrol = UniquePositions::new();
    while !grid.step() {
        let _ = initial_patrol.insert((grid.guard_x, grid.guard_y));
    };

    // Find all possible obstacles and determine which ones result in a loop
    let mut answer = 0;
    for p in initial_patrol.iter().copied() {
        // Skip guard's position as this is invalid
        if p == (grid.guard_init_x, grid.guard_init_y) {
            continue;
        }
        let mut new_patrol = UniquePositionDirections::new();
        grid.reset_guard();
        grid.set_point(p, GridObject::Obstruction);
        while !grid.step() {
            if !new_patrol.insert(((grid.guard_x, grid.guard_y), grid.guard_direction)).unwrap() {
                answer += 1;
                break;
            } else {
            }
        }
        grid.set_point(p, GridObject::Empty);
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_CONTENT: &str = include_str!("../data/day6/input.txt");

    #[test]
    fn test_example_answer() {
        assert_eq!(6, answer(EXAMPLE_CONTENT));
    }

    #[test]
    fn test_answer() {
        println!("answer = {}", answer(INPUT_CONTENT));
    }
}