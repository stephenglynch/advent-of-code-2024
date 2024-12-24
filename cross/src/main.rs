#![no_std]
#![no_main]

use advent_of_code_2024_lib::*;

#[allow(unused_imports)]
use embassy_rp;

use {defmt_rtt as _, panic_probe as _};
use defmt::*;
use embassy_executor::Spawner;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("day 1a answer = {}", day1a::answer());
    info!("day 1b answer = {}", day1b::answer());
    info!("day 2a answer = {}", day2a::answer());
    info!("day 2b answer = {}", day2b::answer());
    info!("day 3a answer = {}", day3a::answer());
    info!("day 3b answer = {}", day3b::answer());
    info!("day 4a answer = {}", day4a::answer());
    info!("day 4b answer = {}", day4b::answer());
    info!("day 5a answer = {}", day5a::answer());
    info!("day 5b answer = {}", day5b::answer());
    info!("day 7a answer = {}", day7a::answer());
}
