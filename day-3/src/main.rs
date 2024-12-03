use std::process::exit;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::anychar;
use nom::combinator::{map, map_res};
use nom::{IResult, Parser};
use std::str::FromStr;
use nom::multi::fold_many1;

fn parse_arg(input: &str) -> IResult<&str, i32> {
    map_res(take_while_m_n(1, 3, |c: char| c.is_digit(10)), |s: &str| i32::from_str(s)).parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    Do,
    Dont,
    Mul(i32, i32)
}

fn parse_mul(input: &str) -> IResult<&str, Op> {
    let (input, _) = tag("mul(")(input)?;
    let (input, arg0) = parse_arg(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, arg1) = parse_arg(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Op::Mul(arg0, arg1)))
}

fn parse_op_or_skip_character(input: &str) -> IResult<&str, Option<Op>> {
    alt((
        map(tag("do()"), |_| Option::Some(Op::Do)),
        map(tag("don't()"), |_| Option::Some(Op::Dont)),
        map(parse_mul, |m| Option::Some(m)),
        map(anychar, |_| Option::None),
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Op>> {
    fold_many1(parse_op_or_skip_character, Vec::new, |mut acc: Vec<Op>, item| {
        if item.is_some() {
            acc.push(item.unwrap());
        }
        acc
    })(input)
}

fn part1(ops: &Vec<Op>) {
    let result: i32 = ops.iter().map(|op| match op {
        Op::Do => 0,
        Op::Dont => 0,
        Op::Mul(x, y) => x * y
    }).sum();
    println!("Part 1: {:?}", result);
}

enum State {
    Active(i32),
    Inactive(i32)
}

impl State {
    pub fn activate(self) -> State {
        match self {
            State::Active(sum) => State::Active(sum),
            State::Inactive(sum) => State::Active(sum),
        }
    }

    pub fn deactivate(self) -> State {
        match self {
            State::Active(sum) => State::Inactive(sum),
            State::Inactive(sum) => State::Inactive(sum),
        }
    }

    pub fn add_if_active(self, value: i32) -> State {
        match self {
            State::Active(sum) => State::Active(value + sum),
            State::Inactive(sum) => State::Inactive(sum),
        }
    }

    pub fn get_sum(self) -> i32 {
        match self {
            State::Active(sum) => sum,
            State::Inactive(sum) => sum,
        }
    }
}

fn part2(ops: &Vec<Op>) {
    let result = ops.iter().fold(State::Active(0), |state, op | match op {
        Op::Do => state.activate(),
        Op::Dont => state.deactivate(),
        Op::Mul(x, y) => state.add_if_active(x * y)
    }).get_sum();
    println!("Part 2: {:?}", result);
}

fn main() {
    let bytes = include_bytes!("../input");
    let string = String::from_utf8_lossy(bytes);
    let parse_result = parse_input(string.as_ref());
    if parse_result.is_err() {
        println!("Failed to parse input: {:?}", parse_result);
        exit(1)
    }
    let (_, ops) = parse_result.unwrap();
    part1(&ops);
    part2(&ops);
}

#[test]
fn test_parse_input() {
    let bytes = include_bytes!("../example_input");
    let string = String::from_utf8_lossy(bytes);
    let parse_result = parse_input(string.as_ref());
    assert_eq!(
        parse_result,
        Ok((
            "",
            vec![
                Op::Mul(2, 4),
                Op::Dont,
                Op::Mul(5, 5) ,
                Op::Mul(11, 8) ,
                Op::Do,
                Op::Mul(8, 5)
            ]
        ))
    );
}
