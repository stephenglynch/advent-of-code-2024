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
const RULES_LUT_LEN: usize = RULES_MAX_LEN.next_power_of_two();
const UPDATES_LIST_MAX_LEN: usize = 200;
const UPDATE_MAX_LEN: usize = 23;

type Rule = (u8, u8);
type RulesList = Vec<Rule, RULES_MAX_LEN>;
type RulesLut = FnvIndexSet<Rule, RULES_LUT_LEN>;
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

fn middle_page(update: &Update) -> u8 {
    update[update.len() / 2]
}

fn create_rule_lut(rules: &RulesList) -> RulesLut {
    let mut lut = RulesLut::new();
    for &pair in rules {
        let _ = lut.insert(pair);
    }
    lut
}

fn check_rule(rules: &RulesLut, a: u8, b: u8) -> Ordering {
    if rules.contains(&(a, b)) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let start = Instant::now();
    let (_, (rules, updates)) = parse(INPUT_CONTENT).unwrap();
    let rule_lut = create_rule_lut(&rules);

    let mut answer = 0;
    for update in updates {
        let mut sorted_update = update.clone();
        sorted_update.sort_unstable_by(|&a, &b| {
            check_rule(&rule_lut, a, b)
        });

        if sorted_update == update {
            let middle = middle_page(&update) as u32;
            answer += middle;
        }
    }

    let duration = Instant::now() - start;
    info!("answer = {} (took {} us)", answer, duration.as_micros());
}
