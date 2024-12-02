#![no_std]
#![no_main]

#[allow(unused_imports)]
use embassy_rp;

use defmt::*;
use embassy_executor::Spawner;
use heapless::Vec;
use {defmt_rtt as _, panic_probe as _};
use advent_of_code_2024::util::count_lines;


const INPUT_CONTENT: &str = include_str!("../../data/day1/input.txt");
const INPUT_NUM_LINES: usize = count_lines(INPUT_CONTENT);

type Id = u32;
type IdList = Vec<Id, INPUT_NUM_LINES>;

fn parse_lists(context: &str) -> (IdList, IdList) {
    let mut list_a = IdList::new();
    let mut list_b = IdList::new();
    for line in context.lines() {
        let mut iter = line.split_whitespace();
        let _ = list_a.push(iter.next().unwrap().parse().unwrap());
        let _ = list_b.push(iter.next().unwrap().parse().unwrap());
    }

    (list_a, list_b)
}

fn calculate_answer(list_a: IdList, list_b: IdList) -> Id
{
    let mut total = 0;
    for (i, a) in list_a.into_iter().enumerate() {
        let a = a as i32;
        let b = list_b[i] as i32;
        total += (b - a).abs() as Id;
    }
    return total;
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let (mut lista, mut listb) = parse_lists(INPUT_CONTENT);
    lista.sort_unstable();
    listb.sort_unstable();
    let answer = calculate_answer(lista, listb);

    info!("answer = {}", answer);
}
