#![no_std]
#![no_main]

use core::cmp::Ordering;

#[allow(unused_imports)]
use embassy_rp;

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Instant;
use heapless::{Vec, FnvIndexSet};
use {defmt_rtt as _, panic_probe as _};

use nom::bytes::complete::tag;
use nom::character::complete::{u8, newline};
use nom::sequence::{separated_pair, terminated};
use nom::combinator::iterator;
use nom::IResult;

const INPUT_CONTENT: &str = include_str!("../../data/day5/input.txt");

// Determined by inspecting input.txt
const RULES_MAX_LEN: usize = 1200;
const UPDATES_LIST_MAX_LEN: usize = 200;
const UPDATE_MAX_LEN: usize = 23;
const MAX_PAGES: usize = 50;

type Rule = (u8, u8);
type RulesList = Vec<Rule, RULES_MAX_LEN>;
type Update = Vec<u8, UPDATE_MAX_LEN>;
type UpdatesList = Vec<Update, UPDATES_LIST_MAX_LEN>;

fn parse(input: &str) -> IResult<&str, (RulesList, UpdatesList)> {
    let (input, rules) = parse_rules(input)?;
    let (input, _) = newline(input)?;
    let (_, updates) = parse_all_updates(input)?;
    Ok((input, (rules, updates)))
}

fn parse_pair(input: &str) -> IResult<&str, Rule> {
    separated_pair(u8, tag("|"), u8)(input)
}

fn parse_rules(input: &str) -> IResult<&str, RulesList> {
    let mut it = iterator(input, terminated(parse_pair, newline));
    let rules_list = it.collect();
    let (input, _) = it.finish()?;
    Ok((input, rules_list))
}

fn parse_update(input: &str) -> IResult<&str, Update> {
    let parser = terminated(u8, tag(","));
    let mut it = iterator(input, parser);
    let mut update_list: Update = it.collect();
    let (input, _) = it.finish()?;
    let (input, last) = u8(input)?;
    let _ = update_list.push(last);
    Ok((input, update_list))
}

fn parse_all_updates(input: &str) -> IResult<&str, UpdatesList> {
    let mut it = iterator(input, terminated(parse_update, newline));
    let updates_list = it.collect();
    let (input, _) = it.finish()?;
    Ok((input, updates_list))
}

fn unique_pages(rules: &RulesList) -> Vec<u8, MAX_PAGES> {
    let mut pages = FnvIndexSet::<_, 64>::new();
    for (left, right) in rules.iter().copied() {
        let _ = pages.insert(left);
        let _ = pages.insert(right);
    }
    pages.into_iter().copied().collect()
}

fn middle_page(update: &Update) -> u8 {
    update[update.len() / 2]
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let (_, (rules, updates)) = parse(INPUT_CONTENT).unwrap();
    let mut pages = unique_pages(&rules);
    info!("rules.len = {} updates.len = {}", rules.len(), updates.len());

    // Sort pages by the ordering rules
    pages.sort_unstable_by(|a, b| {
        if rules.iter().find(|e| **e == (*a, *b)).is_some() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    info!("pages = {=[?]} ({} items)", pages, pages.len());

    // let indexed_pages: Vec<_, MAX_PAGES> = pages.into_iter().enumerate().collect();

    let mut answer = 0;
    let mut update_num = 0;
    for update in updates {
        // info!("update = {=[?]}", update);
        let mut sorted_update = update.clone();
        sorted_update.sort_unstable_by_key(|&x| {
            let (i, _) = pages.iter().enumerate().find(|&(_, &y)| y == x).unwrap();
            i
        });

        if sorted_update == update {
            let middle = middle_page(&update) as u32;
            answer += middle;
            // info!("update {} {=[?]} middle page {}", update_num, update, middle);
        } else {
            error!("update{} = {=[?]} sorted = {=[?]}", update_num, update, sorted_update);
        }
        update_num += 1;
    }

    info!("answer = {}", answer);
}
