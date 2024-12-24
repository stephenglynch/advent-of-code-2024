use heapless::Vec;
use crate::util::{count_lines, count_line_len};

const INPUT_CONTENT: &str = include_str!("../data/day4/input.txt");
const INPUT_NUM_LINES: usize = count_lines(INPUT_CONTENT);
const INPUT_LINE_LEN: usize = count_line_len(INPUT_CONTENT);
const GRID_SIZE: usize = INPUT_NUM_LINES * INPUT_LINE_LEN;

type Grid = Vec<char, GRID_SIZE>;

fn parse_grid(text: &str) -> Grid {
    let mut grid = Grid::new();
    text.lines().for_each(|line| line.chars().for_each(|c| {
        let _ = grid.push(c);
    }));
    grid
}

fn grid_get(grid: &Grid, x: usize, y: usize) -> Option<char> {
    if y >= INPUT_LINE_LEN || x >= INPUT_NUM_LINES {
        None
    } else {
        grid.get(x * INPUT_LINE_LEN + y).copied()
    }
}

fn check_xmass(grid: &Grid, x_start: usize, y_start: usize) -> bool {
    // Check all possible arrangements
    let xmass = [
        [(0, 0, 'M'), (0, 2, 'M'), (1, 1, 'A'), (2, 0, 'S'), (2, 2, 'S')],
        [(0, 0, 'S'), (0, 2, 'S'), (1, 1, 'A'), (2, 0, 'M'), (2, 2, 'M')],
        [(0, 0, 'M'), (0, 2, 'S'), (1, 1, 'A'), (2, 0, 'M'), (2, 2, 'S')],
        [(0, 0, 'S'), (0, 2, 'M'), (1, 1, 'A'), (2, 0, 'S'), (2, 2, 'M')],
    ];

    for xmas in xmass {
        if xmas.into_iter().fold(true, |acc, (x, y, c_xmas)| {
            acc && match grid_get(grid, x_start + x, y_start + y) {
                Some(c) => c == c_xmas,
                None => false
            }
        }) {
            return true
        }
    }
    false
}


fn grid_find_xmas(grid: &Grid) -> u32 {
    let mut total = 0;
    for x in 0..(INPUT_LINE_LEN - 2) {
        for y in 0..(INPUT_NUM_LINES - 2) {
            total += check_xmass(grid, x, y) as u32;
        }
    }
    total
}

pub fn answer() -> u32 {
    let grid = parse_grid(INPUT_CONTENT);
    let answer = grid_find_xmas(&grid);
    return answer;
}
