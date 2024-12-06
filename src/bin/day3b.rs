#![no_std]
#![no_main]

use core::str;

#[allow(unused_imports)]
use embassy_rp;

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::Instant;

use {defmt_rtt as _, panic_probe as _};

use nom::branch::alt;
use nom::bytes::complete::{is_a, tag, take};
use nom::combinator::{iterator, map, value};
use nom::IResult;

const INPUT_CONTENT: &str = include_str!("../../data/day3/input.txt");

#[derive(Clone)]
enum Instruction {
    Do,
    Dont,
    Mult(u32, u32)
}

fn parse_number(input: &[u8]) -> IResult<&[u8], u32> {
    let (input, s) = is_a("0123456789".as_bytes())(input)?;
    Ok((input, str::from_utf8(s).unwrap().parse().unwrap()))
}

fn parse_mult(input: &[u8]) -> IResult<&[u8], Instruction> {
    let (input, _) = tag("mul(".as_bytes())(input)?;
    let (input, a) = parse_number(input)?;
    let (input, _) = tag(",".as_bytes())(input)?;
    let (input, b) = parse_number(input)?;
    let (input, _) = tag(")".as_bytes())(input)?;
    Ok((input, Instruction::Mult(a, b)))
}

fn parse_do(input: &[u8]) -> IResult<&[u8], Instruction> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, Instruction::Do))
}

fn parse_dont(input: &[u8]) -> IResult<&[u8], Instruction> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Instruction::Dont))
}

fn parse_instruction(input: &[u8]) -> IResult<&[u8], Instruction> {
    alt((
        parse_do,
        parse_dont,
        parse_mult
    ))(input)
}

fn calculate_answer(input: &[u8]) -> u32 {
    let (_, answer) = iterator(input, alt((
        map(parse_instruction, |x| Some(x)),
        value(None, take(1usize))
    )))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .fold((1, 0), |(enabled, acc), instr| {
            match instr {
                Instruction::Do => (1, acc),
                Instruction::Dont => (0, acc),
                Instruction::Mult(a, b ) => (enabled, acc + enabled * a * b)
            }
        });

    answer
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let start = Instant::now();
    let answer = calculate_answer(INPUT_CONTENT.as_bytes());
    let duration = Instant::now() - start;
    info!("answer = {} (took {} ms)", answer, duration.as_millis());
}
