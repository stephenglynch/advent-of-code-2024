use heapless::Vec;
use crate::util::count_lines;

const INPUT_CONTENT: &str = include_str!("../data/day2/input.txt");
const INPUT_NUM_LINES: usize = count_lines(INPUT_CONTENT);
const MAX_LEVELS_PER_REPORT: usize = 8;

type Level = i8;
type Report = Vec<Level, MAX_LEVELS_PER_REPORT>;
type AllReports = Vec<Report, INPUT_NUM_LINES>;

fn parse_report(line: &str) -> Report {
    line.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn parse(text: &str) -> AllReports {
    text.lines().map(|line| parse_report(line)).collect()
}

fn calculate_answer(all_reports: AllReports) -> u16 {
    all_reports.into_iter().fold(0, |acc, r| acc + check_safety(r) as u16)
}

fn check_safety(mut report: Report) -> bool {
    if check_increasing(&report) {
        return true;
    }
    else {
        report.reverse();
        check_increasing(&report)
    }
}

fn check_increasing(report: &[Level]) -> bool {
    let mut prev = report[0]; 
    for n in &report[1..] {
        if *n <= prev || *n > prev + 3 {
            return false;
        }
        prev = *n;
    }
    true
}

pub fn answer() -> u16 {
    let all_reports = parse(INPUT_CONTENT);
    let answer = calculate_answer(all_reports);
    return answer;
}

