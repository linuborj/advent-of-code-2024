use std::process::exit;
use nom::character::complete::{digit1, multispace0, newline, space1};
use nom::multi::{separated_list0, separated_list1};
use nom::{combinator::map_res, IResult};
use std::str::FromStr;

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(
        space1,
        map_res(digit1, |s: &str| i32::from_str(s)),
    )(input)
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list0(newline, parse_line)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, _) = multispace0(input)?;
    let (input, lines) = parse_lines(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, lines))
}

fn is_safe(list: &Vec<i32>) -> bool {
    let deltas: Vec<i32> = list.iter().zip(list.iter().skip(1)).map(|(a, b)| a - b).collect();
    (deltas.iter().all(|x| *x > 0) || deltas.iter().all(|x| *x < 0)) && deltas.iter().all(|x| (*x).abs() <= 3)
}

fn part1(lists: &Vec<Vec<i32>>) {
    let safe_lists: Vec<&Vec<i32>> = lists.iter().filter(|x| is_safe(x)).collect();
    println!("Part 1: {:?}", safe_lists.iter().count());
}

fn is_almost_safe(list: &Vec<i32>) -> bool {
    // We know from part 1 that more than half of the lists are completely safe, that is about ~500.
    // The loop below will accept them on its first iteration.
    // The remaining ~500 lists may take up to 8 iterations to determine that they are
    // unsafe. So we're talking ~500 + (~8 * ~500) = ~4500 iterations in total.
    // That's nothing, no need to optimize.
    for (index, _) in list.iter().enumerate() {
        let list_with_an_excluded_item: Vec<i32> = list.iter().enumerate().filter(|(i, _) | *i != index).map(|(_, x)| x.clone()).collect();
        if is_safe(&list_with_an_excluded_item) {
            return true;
        }
    }
    false
}

fn part2(lists: &Vec<Vec<i32>>) {
    let almost_safe_lists: Vec<&Vec<i32>> = lists.iter().filter(|x| is_almost_safe(x)).collect();
    println!("Part 2: {:?}", almost_safe_lists.iter().count());
}

fn main() {
    let bytes = include_bytes!("../input");
    let string = String::from_utf8_lossy(bytes);
    let parse_result = parse_input(string.as_ref());
    if parse_result.is_err() {
        println!("Failed to parse input: {:?}", parse_result);
        exit(1)
    }
    let (_, lists) = parse_result.unwrap();
    part1(&lists);
    part2(&lists);
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
            [
                [7, 6, 4, 2, 1].to_vec(),
                [1, 2, 7, 8, 9].to_vec(),
                [9, 7, 6, 2, 1].to_vec(),
                [1, 3, 2, 4, 5].to_vec(),
                [8, 6, 4, 4, 1].to_vec(),
                [1, 3, 6, 7, 9].to_vec(),
            ].to_vec()
        ))
    );
}