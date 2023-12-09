use anyhow::{Context, Result};

#[aoc_generator(day09)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<i64>>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| {
            l.split(' ')
                .map(|v| Ok(v.parse::<i64>()?))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

fn next_value(row: &Vec<i64>) -> Option<i64> {
    let mut diffs = vec![row.clone()];
    loop {
        let next = diffs
            .iter()
            .last()?
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>();

        if next.iter().all(|v| *v == 0) {
            diffs.push(next);
            let lasts = diffs
                .iter()
                .map(|v| v.iter().last())
                .collect::<Option<Vec<_>>>()?;

            return Some(lasts.iter().map(|v| **v).sum::<i64>());
        } else {
            diffs.push(next);
        }
    }
}

#[aoc(day09, part1)]
pub fn solve_part1(input: &[Vec<i64>]) -> Result<i64> {
    let result = input.iter().filter_map(next_value).sum::<i64>();
    Ok(result)
}

fn previous_value(row: &Vec<i64>) -> Option<i64> {
    let mut diffs = vec![row.clone()];
    loop {
        let next = diffs
            .iter()
            .last()?
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>();
        if next.iter().all(|v| *v == 0) {
            diffs.push(next);
            let lasts = diffs
                .iter()
                .map(|v| v.iter().rev().last())
                .collect::<Option<Vec<_>>>()?;

            return Some(lasts.iter().rev().map(|v| **v).fold(0, |acc, el| el - acc));
        } else {
            diffs.push(next);
        }
    }
}

#[aoc(day09, part2)]
pub fn solve_part2(input: &[Vec<i64>]) -> Result<i64> {
    let result = input.iter().filter_map(previous_value).sum::<i64>();
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
    }

    fn input() -> Result<Vec<Vec<i64>>> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(114, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(2, solve_part2(&data)?))
    }
}
