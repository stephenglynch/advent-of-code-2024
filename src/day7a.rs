use heapless::Vec;
use nom::bytes::complete::tag;
use nom::character::complete::{u32, newline};
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
    Mult
}

type Operands = Vec<u32, OPERANDS_MAX_LEN>;
type Equation = (u32, Operands);
type EquationList = Vec<Equation, EQUATIONS_LIST_MAX_LEN>;
type Operators = Vec<Op, OPERATORS_MAX_LEN>;

fn parse(input: &str) -> IResult<&str, EquationList> {
    let mut it = iterator(input, terminated(parse_equation, newline));
    let equations: EquationList = it.collect();
    let (input, _) = it.finish()?;
    Ok((input, equations))
}

fn parse_operands(input: &str) -> IResult<&str, Operands> {
    let mut it = iterator(input, terminated(u32, tag(" ")));
    let mut operands: Operands = it.collect();
    let (input, _) = it.finish()?;
    let (input, last) = u32(input)?;
    let _ = operands.push(last);
    Ok((input, operands))
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let (input, result) = terminated(u32, tag(": "))(input)?;
    let (input, operands) = parse_operands(input)?;
    Ok((input, (result, operands)))
}

// fn evaluate_equation(eq: Equation, ops: Operators) -> bool {
//     let (expected, operands) = eq;
//     let mut i = 0;
//     let result = operands.into_iter().reduce(|acc, x| {
//         let val = match ops[i] {
//             Op::Add => acc + x,
//             Op::Mult => acc * x,
//         };
//         i += 1;
//         val
//     }).unwrap();

//     expected == result
// }

fn cycle_op(op: Op) -> (Op, bool) {
    match op {
        Op::Add => (Op::Mult, false),
        Op::Mult => (Op::Add, true)
    }
}

fn evaluate_op(op: Op, a: u32, b: u32) -> u32 {
    match op {
        Op::Add => a + b,
        Op::Mult => a * b
    }
}

fn check_equation(eq: Equation) -> Option<u32> {
    let (expected, operands) = eq;
    let num_operators = operands.len() - 1;
    let mut operators: Operators = Vec::new();
    let mut carry = false;
    let _ = operators.resize(num_operators, Op::Add);
    for _ in 0..2_u32.pow(num_operators as u32) {
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

        // Cycle to next permutation
        (operators[0], carry) = cycle_op(operators[0]);
        for op in operators.iter_mut() {
            if carry {
                (*op, carry) = cycle_op(*op);
            } else {
                break;
            }
        }
    }

    None
}

pub fn answer() -> u32 {
    let (_, equations) = parse(INPUT_CONTENT).unwrap();
    let mut answer = 0;
    for eq in equations {
        if let Some(val) = check_equation(eq) {
            answer += val;
        }
    }
    return answer;
}
