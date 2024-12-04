#![no_std]
#![no_main]

use core::str;

#[allow(unused_imports)]
use embassy_rp;

use defmt::info;
use embassy_executor::Spawner;
use heapless::Vec;

use {defmt_rtt as _, panic_probe as _};

use nom::{
    bytes::complete::{is_a, tag },
    error::{Error, ErrorKind},
    multi::fold_many0,
    Err, IResult};

const INPUT_CONTENT: &str = include_str!("../../data/day3/input.txt");
const PAIRS_VEC_LEN: usize = 1000;

type MultPair = (u32, u32);
type MultPairVec = Vec<MultPair, PAIRS_VEC_LEN>;

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

fn search_until_mult(mut input: &[u8]) -> IResult<&[u8], MultPair> {
    loop {
        match parse_mult(input) {
            Ok((i, pair)) => return Ok((i, pair)),
            Err(_) => ()
        }
        input = match &input.get(1..) {
            Some(i) => i,
            None => return Err(Err::Error(Error::new(input, ErrorKind::Fail)))
        };
    }
}

fn parse_all(input: &[u8]) -> IResult<&[u8], MultPairVec> {
    let (input, x) = fold_many0(
        search_until_mult,
        MultPairVec::new,
        |mut acc, item| {
            let _ = acc.push(item);
            acc
        }
    )(input)?;
    Ok((input, x))
}


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let (_, mults) = parse_all(INPUT_CONTENT.as_bytes()).unwrap();
    info!("mults.len = {}", mults.len());
    let answer = mults.into_iter().fold(0, |acc, (a, b)| acc + a * b);
    info!("answer = {}", answer);
}
