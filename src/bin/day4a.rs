#![no_std]
#![no_main]

#[allow(unused_imports)]
use embassy_rp;

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Instant;
use heapless::Vec;
use {defmt_rtt as _, panic_probe as _};
use advent_of_code_2024::util::{count_lines, count_line_len};

const INPUT_CONTENT: &str = include_str!("../../data/day4/input.txt");
const INPUT_NUM_LINES: usize = count_lines(INPUT_CONTENT);
const INPUT_LINE_LEN: usize = count_line_len(INPUT_CONTENT);
const GRID_SIZE: usize = INPUT_NUM_LINES * INPUT_LINE_LEN;

type Grid = Vec<char, GRID_SIZE>;
type Quad = [char; 4];

fn parse_grid(text: &str) -> Grid {
    let mut grid = Grid::new();
    text.lines().for_each(|line| line.chars().for_each(|c| {
        grid.push(c);
    }));
    grid
}

fn grid_get(grid: &Grid, x: usize, y: usize) -> Option<char> {
    if x >= INPUT_LINE_LEN || y >= INPUT_NUM_LINES {
        None
    } else {
        grid.get(y * INPUT_LINE_LEN + x).copied()
    }
}

fn grid_get_4(grid: &Grid, x_start: usize, y_start: usize, dir_x: i8, dir_y: i8) -> Option<Quad> {
    let mut quad: Quad = ['.'; 4];
    for i in 0..4 {
        let x =  x_start + (i * dir_x) as usize;
        let y = y_start + (i * dir_y) as usize;
        match grid_get(grid, x, y) {
            Some(c) => quad[i as usize] = c,
            None => return None
        };
    }
    Some(quad)
}

fn grid_find_xmas(grid: &Grid) -> u32 {
    let xmas: [char; 4] = ['X', 'M', 'A', 'S'];
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, -1), (-1, 1)];
    let mut total = 0;
    for x in 0..INPUT_LINE_LEN {
        for y in 0..INPUT_NUM_LINES {
            for (dir_x, dir_y) in directions {
                    match grid_get_4(grid, x, y, dir_x, dir_y) {
                        Some(quad) => if quad == xmas {total += 1},
                        None => continue
                    }
            }
        }
    }
    total
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let start = Instant::now();
    let grid = parse_grid(INPUT_CONTENT);
    let answer = grid_find_xmas(&grid);
    let duration = Instant::now() - start;
    info!("answer = {} (took {} us)", answer, duration.as_micros());
}
