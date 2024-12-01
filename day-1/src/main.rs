use std::collections::{HashMap, HashSet};
use std::process::exit;
use nom::character::complete::{digit1, multispace0, newline, space0};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{combinator::map_res, IResult};
use std::str::FromStr;

fn parse_tuple(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        map_res(digit1, |s: &str| i32::from_str(s)),
        space0,
        map_res(digit1, |s: &str| i32::from_str(s)),
    )(input)
}

fn parse_tuples(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    separated_list1(newline, parse_tuple)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    let (input, _) = multispace0(input)?;
    let (input, tuples) = parse_tuples(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, tuples))
}

fn part1(tuples: Vec<(i32, i32)>) {
    let (mut left_values, mut right_values): (Vec<i32>, Vec<i32>) = tuples.iter().cloned().unzip();
    left_values.sort();
    right_values.sort();
    let distance: u32 = left_values.iter().zip(right_values).map(|(a, b)| a.abs_diff(b)).sum();

    println!("Part 1: {:?}", distance);
}

fn part2(tuples: Vec<(i32, i32)>) {
    let (lefts, rights): (Vec<i32>, Vec<i32>) = tuples.iter().cloned().unzip();
    let lefts: HashSet<&i32> = lefts.iter().collect();
    let counts: HashMap<i32, i32> = rights.iter()
        .filter(|value| lefts.contains(value))
        .fold(HashMap::new(), |mut current_count, value| {
            current_count.entry(*value).and_modify(|count| *count += 1).or_insert(1);
            current_count
        });
    let similarity: i32 = counts.iter().map(|(key, value)| *key * value).sum();
    println!("Part 2: {:?}", similarity);
}

fn main() {
    let bytes = include_bytes!("../input");
    let string = String::from_utf8_lossy(bytes);
    let parse_result = parse_input(string.as_ref());
    if parse_result.is_err() {
        println!("Failed to parse input: {:?}", parse_result);
        exit(1)
    }
    let (_, tuples) = parse_result.unwrap();
    part1(tuples.clone());
    part2(tuples.clone());
}

#[test]
fn test_parse_tuples() {
    let bytes = include_bytes!("../example_input");
    let string = String::from_utf8_lossy(bytes);
    let parse_result = parse_input(string.as_ref());
    assert_eq!(
        parse_result,
        Ok((
            "",
            [
                (3, 4),
                (4, 3),
                (2, 5),
                (1, 3),
                (3, 9),
                (3, 4),
            ].to_vec()
        ))
    );
}