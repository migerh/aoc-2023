use anyhow::Result;
use std::{
    cmp::{max, min},
    collections::HashMap,
};

type Map = Vec<Vec<char>>;

#[aoc_generator(day03)]
pub fn input_generator(input: &str) -> Result<Map> {
    Ok(input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>())
}

fn find_gears(
    input: &Map,
    is_valid: fn(char) -> bool,
) -> Result<HashMap<(usize, usize), Vec<u32>>> {
    let mut map = HashMap::new();

    for y in 0..input.len() {
        let mut buf = vec![];
        let mut start = 0;
        let mut in_number = false;
        for x in 0..input[y].len() {
            if !input[y][x].is_ascii_digit() || x == input[y].len() - 1 {
                if !in_number {
                    continue;
                }

                if input[y][x].is_ascii_digit() && x == input[y].len() - 1 {
                    buf.push(input[y][x]);
                }

                let num = buf.iter().collect::<String>().parse::<u32>()?;
                for ys in (max(1, y) - 1)..=min(input.len() - 1, y + 1) {
                    for xs in (max(1, start) - 1)..=min(input[y].len() - 1, x) {
                        if is_valid(input[ys][xs]) {
                            map.entry((ys, xs))
                                .and_modify(|v: &mut Vec<u32>| v.push(num))
                                .or_insert(vec![num]);
                        }
                    }
                }

                buf = vec![];
                in_number = false;
            } else {
                if !in_number {
                    start = x;
                }
                in_number = true;
                buf.push(input[y][x]);
            }
        }
    }

    Ok(map)
}

#[aoc(day03, part1)]
pub fn solve_part1(input: &Map) -> Result<u32> {
    let map = find_gears(input, |c| !c.is_ascii_digit() && c != '.')?;
    let result = map.values().map(|v| v.iter().sum::<u32>()).sum::<u32>();

    Ok(result)
}

#[aoc(day03, part2)]
pub fn solve_part2(input: &Map) -> Result<u32> {
    let map = find_gears(input, |c| c == '*')?;
    let result = map
        .values()
        .filter_map(|v| {
            if v.len() == 2 {
                Some(v[0] * v[1])
            } else {
                None
            }
        })
        .sum::<u32>();

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
    }

    fn input() -> Result<Map> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(4361, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(467835, solve_part2(&data)?))
    }
}
