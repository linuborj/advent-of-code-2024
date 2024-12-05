use nom::bytes::complete::tag;
use nom::character::complete::satisfy;
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::IResult;
use std::process::exit;

fn parse_cell(input: &str) -> IResult<&str, char> {
    satisfy(char::is_uppercase)(input)
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(tag("\n"), many1(parse_cell))(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Table {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Vec<char>>
}

impl Table {
    pub fn indices(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        (0..self.height).flat_map(move |y| (0..self.width).map(move |x| (x, y))).into_iter()
    }

    fn x_mas_range(&self, (x, y): (i32, i32)) -> Vec<Vec<(i32, i32)>> {
        vec![
            (0..4).map(move |d| (x + d, y)).collect(),
            (0..4).map(move |d| (x - d, y)).collect(),
            (0..4).map(move |d| (x, y + d)).collect(),
            (0..4).map(move |d| (x, y - d)).collect(),
            (0..4).map(move |d| (x + d, y + d)).collect(),
            (0..4).map(move |d| (x - d, y + d)).collect(),
            (0..4).map(move |d| (x + d, y - d)).collect(),
            (0..4).map(move |d| (x - d, y - d)).collect(),
        ]
    }

    fn get(&self, (x, y): (i32, i32)) -> char {
        if x < 0 || x >= self.width {
            return '-';
        }
        if y < 0 || y >= self.height {
            return '-';
        }
        self.cells[y as usize][x as usize]
    }
}

fn parse_input(input: &str) -> IResult<&str, Table> {
    map(parse_lines, |lines| Table { width: lines[0].len() as i32, height: lines.len() as i32, cells: lines })(input)
}

fn part1(table: &Table) {
    let mut xmas_count: i32 = 0;
    for (x, y) in table.indices() {
        if table.get((x, y)) == 'X' {
            for range in table.x_mas_range((x, y)) {
                let letters: Vec<char> = range.iter().map(|&(x, y)| { table.get((x, y)) }).collect();
                if letters  == vec!['X', 'M', 'A', 'S'] {
                    xmas_count += 1
                }
            }
        }
    }
    println!("Part 1: {:?}", xmas_count);
}

fn part2(table: &Table) {
    let mut cross_mas_count: i32 = 0;
    for (x, y) in table.indices() {
        if table.get((x, y)) == 'A' {
            if ((table.get((x - 1, y - 1)) == 'M' && table.get((x + 1, y + 1)) == 'S') || (table.get((x - 1, y - 1)) == 'S' && table.get((x + 1, y + 1)) == 'M')) &&
                ((table.get((x - 1, y + 1)) == 'M' && table.get((x + 1, y - 1)) == 'S') || (table.get((x - 1, y + 1)) == 'S' && table.get((x + 1, y - 1)) == 'M')) {
                cross_mas_count += 1;
            }
        }
    }
    println!("Part 2: {:?}", cross_mas_count);
}

fn main() {
    let bytes = include_bytes!("../input");
    let string = String::from_utf8_lossy(bytes);
    let parse_result = parse_input(string.as_ref());
    if parse_result.is_err() {
        println!("Failed to parse input: {:?}", parse_result);
        exit(1)
    }
    let (_, table) = parse_result.unwrap();

    part1(&table);
    part2(&table);
}

#[test]
fn test_parse_input() {
    let bytes = include_bytes!("../example_input");
    let string = String::from_utf8_lossy(bytes);
    let parse_result = parse_input(string.as_ref());
    assert_eq!(parse_result.is_ok(), true)
}
