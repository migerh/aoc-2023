use anyhow::{Context, Error, Result};
use rayon::{slice::ParallelSlice, iter::ParallelIterator};
use std::str::FromStr;

use crate::utils::AocError::*;

#[derive(Debug)]
pub struct Almanac {
    seed_map: Vec<SeedMap>,
    seeds: Vec<i128>,
}

impl FromStr for Almanac {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let seed_map = s
            .split("\n\n")
            .filter(|s| !s.is_empty())
            .skip(1)
            .map(SeedMap::from_str)
            .collect::<Result<Vec<_>>>()
            .context("Error while parsing input")?;

        let seeds = s.lines().next()
            .ok_or(GenericError).context("Could not find line with seeds")?
            .split(": ")
            .nth(1)
            .ok_or(GenericError).context("Could not find seeds")?
            .split(' ')
            .map(|v| Ok(v.trim().parse::<i128>()?))
            .collect::<Result<Vec<_>>>()?;

        Ok( Almanac { seed_map, seeds })
    }
}

impl Almanac {
    pub fn map_seed(&self, seed: i128) -> i128 {
        let mut seed = seed;

        for map in &self.seed_map {
            if let Some(v) = map.map_seed(seed) {
                seed = v;
            }
        }

        seed
    }
}

#[derive(Debug)]
pub struct SeedMap {
    ranges: Vec<Range>,
}

impl FromStr for SeedMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let ranges = s
            .lines()
            .skip(1)
            .map(Range::from_str)
            .collect::<Result<Vec<_>>>()?;
        Ok(SeedMap { ranges })
    }
}

impl SeedMap {
    pub fn map_seed(&self, seed: i128) -> Option<i128> {
        self.ranges.iter().filter_map(|r| r.map(seed)).next()
    }
}

#[derive(Debug)]
pub struct Range {
    destination_start: i128,
    source_start: i128,
    length: i128,
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let values = s.split(' ')
            .map(|v| Ok(v.trim().parse::<i128>()?))
            .collect::<Result<Vec<_>>>()?;

        if values.len() != 3 {
            return Err(GenericError).context("Could not parse range")?
        }

        Ok(Range { destination_start: values[0], source_start: values[1], length: values[2] })
    }
}

impl Range {
    pub fn map(&self, seed: i128) -> Option<i128> {
        if self.source_start <= seed && seed < self.source_start + self.length {
            Some(self.destination_start + seed - self.source_start)
        } else {
            None
        }
    }
}

#[aoc_generator(day05)]
pub fn input_generator(input: &str) -> Result<Almanac> {
    Almanac::from_str(input)
}

#[aoc(day05, part1)]
pub fn solve_part1(input: &Almanac) -> Result<i128> {
    let result = input.seeds.iter().map(|s| input.map_seed(*s)).collect::<Vec<_>>();
    Ok(*result.iter().min().ok_or(GenericError).context("Could not find lowest location")?)
}

#[aoc(day05, part2)]
pub fn solve_part2(input: &Almanac) -> Result<i128> {
    let min = input.seeds.par_chunks_exact(2).filter_map(|r| {
        let start = r[0];
        let end = r[0] + r[1];
        (start..end).map(|s| input.map_seed(s)).min()
    }).min().ok_or(GenericError).context("Foo")?;
    Ok(min)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
    }

    fn input() -> Result<Almanac> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(35, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(46, solve_part2(&data)?))
    }
}
