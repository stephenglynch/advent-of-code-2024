use core::str;
use nom::branch::alt;
use nom::bytes::complete::{is_a, tag, take};
use nom::combinator::{iterator, map, value};
use nom::IResult;

const INPUT_CONTENT: &str = include_str!("../data/day3/input.txt");

type MultPair = (u32, u32);

fn parse_number(input: &[u8]) -> IResult<&[u8], u32> {
    let (input, s) = is_a("0123456789".as_bytes())(input)?;
    Ok((input, str::from_utf8(s).unwrap().parse().unwrap()))
}

fn parse_mult(input: &[u8]) -> IResult<&[u8], MultPair> {
    let (input, _) = tag("mul(".as_bytes())(input)?;
    let (input, a) = parse_number(input)?;
    let (input, _) = tag(",".as_bytes())(input)?;
    let (input, b) = parse_number(input)?;
    let (input, _) = tag(")".as_bytes())(input)?;
    Ok((input, (a, b)))
}

fn calculate_answer(input: &[u8]) -> u32 {
    iterator(input, alt((
        map(parse_mult, |x| Some(x)),
        value(None, take(1usize))
    )))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .fold(0, |acc, (a, b)| acc + a * b)
}

pub fn answer() -> u32 {
    return calculate_answer(INPUT_CONTENT.as_bytes());
}
