use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::{cmp::Ordering, str::FromStr};

use crate::utils::AocError::*;

const PART2: bool = true;

#[derive(PartialEq, PartialOrd, Debug, Eq, Ord)]
pub enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Debug, Clone, Eq)]
pub struct Hand {
    cards: Vec<char>,
    bid: u32,
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.split(' ');

        let cards = split
            .next()
            .ok_or(GenericError)
            .context("Could not parse cards")?
            .chars()
            .collect::<Vec<_>>();
        if cards.len() != 5 {
            return Err(GenericError).context("Not exactly five cards");
        }

        let bid = split
            .next()
            .ok_or(GenericError)
            .context("Could not parse bid")?
            .parse::<u32>()?;
        Ok(Hand { cards, bid })
    }
}

impl Hand {
    pub fn score_card(&self, c: char) -> Result<u8> {
        Ok(match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => c.to_digit(10).ok_or(GenericError)? as u8,
        })
    }

    pub fn score_card2(&self, c: char) -> Result<u8> {
        Ok(match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _ => c.to_digit(10).ok_or(GenericError)? as u8,
        })
    }

    pub fn partition(&self, hand: &Vec<u8>) -> Vec<u8> {
        let mut result = vec![1];
        let mut pos = 0;

        for i in 0..4 {
            if hand[i] == hand[i + 1] {
                result[pos] += 1;
            } else {
                result.push(1);
                pos += 1;
            }
        }

        result
    }

    pub fn score(&self) -> Result<Kind> {
        let mut scores = self
            .cards
            .iter()
            .map(|c| self.score_card(*c))
            .collect::<Result<Vec<_>>>()?;
        scores.sort();

        let mut partition = self.partition(&scores);
        partition.sort();

        if partition.len() == 1 && partition[0] == 5 {
            return Ok(Kind::Five);
        }

        if partition.len() == 2 && partition[0] == 1 && partition[1] == 4 {
            return Ok(Kind::Four);
        }

        if partition.len() == 2 && partition[0] == 2 && partition[1] == 3 {
            return Ok(Kind::FullHouse);
        }

        if partition.len() == 3 && partition[0] == 1 && partition[1] == 1 && partition[2] == 3 {
            return Ok(Kind::Three);
        }

        if partition.len() == 3 && partition[0] == 1 && partition[1] == 2 && partition[2] == 2 {
            return Ok(Kind::TwoPair);
        }

        if partition.len() == 4 {
            return Ok(Kind::OnePair);
        }

        Ok(Kind::HighCard)
    }

    pub fn score2(&self) -> Result<Kind> {
        let mut scores = self
            .cards
            .iter()
            .map(|c| self.score_card2(*c))
            .collect::<Result<Vec<_>>>()?;
        scores.sort();

        let mut partition = self.partition(&scores);
        partition.sort();

        if partition.len() == 1 && partition[0] == 5 {
            return Ok(Kind::Five);
        }

        if partition.len() == 2 && partition[0] == 1 && partition[1] == 4 {
            if scores[0] == 1 {
                return Ok(Kind::Five);
            }

            return Ok(Kind::Four);
        }

        if partition.len() == 2 && partition[0] == 2 && partition[1] == 3 {
            if scores[0] == 1 {
                return Ok(Kind::Five);
            }
            return Ok(Kind::FullHouse);
        }

        if partition.len() == 3 && partition[0] == 1 && partition[1] == 1 && partition[2] == 3 {
            if scores[0] == 1 {
                return Ok(Kind::Four);
            }
            return Ok(Kind::Three);
        }

        if partition.len() == 3 && partition[0] == 1 && partition[1] == 2 && partition[2] == 2 {
            if scores[1] == 1 {
                return Ok(Kind::Four);
            }

            if scores[0] == 1 {
                return Ok(Kind::FullHouse);
            }
            return Ok(Kind::TwoPair);
        }

        if partition.len() == 4 {
            if scores[0] == 1 {
                return Ok(Kind::Three);
            }
            return Ok(Kind::OnePair);
        }

        if scores[0] == 1 {
            return Ok(Kind::OnePair);
        }

        Ok(Kind::HighCard)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if PART2 {
            self.score2().unwrap() == other.score2().unwrap()
        } else {
            self.score().unwrap() == other.score().unwrap()
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if PART2 {
            let self_kind = self.score2().unwrap();
            let other_kind = other.score2().unwrap();
            let kind_match = self_kind == other_kind;

            if !kind_match {
                self_kind.cmp(&other_kind)
            } else {
                for i in 0..5 {
                    if self.cards[i] != other.cards[i] {
                        return self
                            .score_card2(self.cards[i])
                            .unwrap()
                            .cmp(&other.score_card2(other.cards[i]).unwrap());
                    }
                }

                Ordering::Equal
            }
        } else {
            let self_kind = self.score().unwrap();
            let other_kind = other.score().unwrap();
            let kind_match = self_kind == other_kind;

            if !kind_match {
                self_kind.cmp(&other_kind)
            } else {
                for i in 0..5 {
                    if self.cards[i] != other.cards[i] {
                        return self
                            .score_card(self.cards[i])
                            .unwrap()
                            .cmp(&other.score_card(other.cards[i]).unwrap());
                    }
                }

                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Less))
    }

    fn le(&self, other: &Self) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(core::cmp::Ordering::Less | core::cmp::Ordering::Equal)
        )
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(Ordering::Greater | Ordering::Equal)
        )
    }
}

#[aoc_generator(day07)]
pub fn input_generator(input: &str) -> Result<Vec<Hand>> {
//     let input = "32T3K 765
// T55J5 684
// KK677 28
// KTJJT 220
// QQQJA 483";
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Hand::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

#[aoc(day07, part1)]
pub fn solve_part1(input: &[Hand]) -> Result<usize> {
    let mut hands = input.to_vec();
    hands.sort();

    let result = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * (h.bid as usize))
        .sum::<usize>();

    println!("{:?}", hands);
    Ok(result)
}

#[aoc(day07, part2)]
pub fn solve_part2(input: &[Hand]) -> Result<usize> {
    let mut hands = input.to_vec();
    hands.sort();

    let result = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * (h.bid as usize))
        .sum::<usize>();

    println!("{:?}", hands);
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<Vec<Hand>> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(0, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(0, solve_part2(&data)?))
    }
}
