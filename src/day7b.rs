use heapless::Vec;
use nom::bytes::complete::tag;
use nom::character::complete::{u64, newline};
use nom::sequence::terminated;
use nom::combinator::iterator;
use nom::IResult;

const INPUT_CONTENT: &str = include_str!("../data/day7/example.txt");

// Determined by inspecting input.txt
const EQUATIONS_LIST_MAX_LEN: usize = 850;
const OPERANDS_MAX_LEN: usize = 12;
const OPERATORS_MAX_LEN: usize = OPERANDS_MAX_LEN - 1;

#[derive(Copy, Clone, Debug)]
enum Op {
    Add,
    Mult,
    Concat
}

type Operands = Vec<u64, OPERANDS_MAX_LEN>;
type Equation = (u64, Operands);
type EquationList = Vec<Equation, EQUATIONS_LIST_MAX_LEN>;
type Operators = Vec<Op, OPERATORS_MAX_LEN>;

fn parse(input: &str) -> IResult<&str, EquationList> {
    let mut it = iterator(input, terminated(parse_equation, newline));
    let equations: EquationList = it.collect();
    let (input, _) = it.finish()?;
    Ok((input, equations))
}

fn parse_operands(input: &str) -> IResult<&str, Operands> {
    let mut it = iterator(input, terminated(u64, tag(" ")));
    let mut operands: Operands = it.collect();
    let (input, _) = it.finish()?;
    let (input, last) = u64(input)?;
    let _ = operands.push(last);
    Ok((input, operands))
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let (input, result) = terminated(u64, tag(": "))(input)?;
    let (input, operands) = parse_operands(input)?;
    Ok((input, (result, operands)))
}

fn concat_ints(mut a: u64, b: u64) -> u64 {
    let mut temp = b;
    while temp > 0 {
        temp /= 10;
        a *= 10;
    }
    a + b
}

fn evaluate_op(op: Op, a: u64, b: u64) -> u64 {
    match op {
        Op::Add => a + b,
        Op::Mult => a * b,
        Op::Concat => concat_ints(a, b)
    }
}

struct OpsPermutations {
    current: Option<Operators>,
    state: Operators,
}

impl OpsPermutations {
    fn new(length: usize) -> Self {
        let mut current = Vec::new();
        let _ = current.resize(length, Op::Add);
        OpsPermutations {
            state: current.clone(),
            current: Some(current)
        }
    }
}

impl Iterator for OpsPermutations{
    type Item = Operators;

    fn next(&mut self) -> Option<Self::Item> {
        let mut bump_next = true;
        let next = self.current.clone();
        for op in self.state.iter_mut() {
            if bump_next {
                *op = match *op {
                    Op::Add => {
                        bump_next = false;
                        Op::Mult
                    }
                    Op::Mult => {
                        bump_next = false;
                        Op::Concat
                    }
                    Op::Concat => {
                        bump_next = true;
                        Op::Add
                    }
                };
            } else {
                break;
            }
        }
        if !bump_next {
            self.current = Some(self.state.clone());
        } else {
            self.current = None;
        }
        next
    }
}

fn check_equation(eq: Equation) -> Option<u64> {
    let (expected, operands) = eq;
    let iter = OpsPermutations::new(operands.len() - 1);

    for operators in iter {
        // Compute current permutation
        let mut i = 0;
        let result  = operands.iter().copied().reduce(|acc, x| {
            let val = evaluate_op(operators[i], acc, x);
            i += 1;
            val
        }).unwrap();

        // Check result
        if result == expected {
            return Some(result);
        }
    }

    None
}

pub fn answer() -> u64 {
    let (_, equations) = parse(INPUT_CONTENT).unwrap();
    let mut answer = 0;
    for eq in equations {
        if let Some(val) = check_equation(eq) {
            answer += val;
        }
    }
    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_CONTENT: &str = include_str!("../data/day7/example.txt");
    const INPUT_CONTENT: &str = include_str!("../data/day7/input.txt");

    #[test]
    fn test_iter() {
        println!("blah blah");
        let iter = OpsPermutations::new(2);
        for ops in iter {
            println!("{:?}", ops);
        }
    }

    #[test]
    fn test_answer() {
        let (_, equations) = parse(INPUT_CONTENT).unwrap();
        let mut answer = 0;
        for eq in equations {
            if let Some(val) = check_equation(eq) {
                answer += val;
            }
        }
        println!("answer = {}", answer);
    }

    #[test]
    fn test_example_answer() {
        assert_eq!(answer(), 3749);
    }
}