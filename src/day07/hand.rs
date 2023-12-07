use std::{str::FromStr, cmp::Ordering};

use anyhow::{Error, Result, Context};

use crate::utils::AocError::*;

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

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<char>,
    pub bid: u32,
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
    pub fn score_card(c: char) -> Result<u8> {
        Ok(match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            v => v
                .to_digit(10)
                .ok_or(GenericError)
                .with_context(|| format!("Invalid card: {v}"))? as u8,
        })
    }

    pub fn score_card_with_joker(c: char) -> Result<u8> {
        Ok(match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _ => c.to_digit(10).ok_or(GenericError)? as u8,
        })
    }

    pub fn partition(hand: &[u8]) -> Vec<u8> {
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

    pub fn kind(&self) -> Result<Kind> {
        let mut scores = self
            .cards
            .iter()
            .map(|c| Self::score_card(*c))
            .collect::<Result<Vec<_>>>()
            .context("Could not determine score")?;
        scores.sort();

        let mut partition = Self::partition(&scores);
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

    pub fn kind_with_joker(&self) -> Result<Kind> {
        let mut scores = self
            .cards
            .iter()
            .map(|c| Self::score_card_with_joker(*c))
            .collect::<Result<Vec<_>>>()
            .context("Could not determine scores (with joker)")?;
        scores.sort();

        let mut partition = Self::partition(&scores);
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

    pub fn compare_with(&self, other: &Self) -> Ordering {
            let self_kind = self.kind().unwrap();
            let other_kind = other.kind().unwrap();
            let kind_match = self_kind == other_kind;

            if !kind_match {
                self_kind.cmp(&other_kind)
            } else {
                for i in 0..5 {
                    if self.cards[i] != other.cards[i] {
                        return Self
                            ::score_card(self.cards[i])
                            .unwrap()
                            .cmp(&Self::score_card(other.cards[i]).unwrap());
                    }
                }

                Ordering::Equal
            }
    }

    pub fn compare_with_joker_with(&self, other: &Self) -> Ordering {
            let self_kind = self.kind_with_joker().unwrap();
            let other_kind = other.kind_with_joker().unwrap();
            let kind_match = self_kind == other_kind;

            if !kind_match {
                self_kind.cmp(&other_kind)
            } else {
                for i in 0..5 {
                    if self.cards[i] != other.cards[i] {
                        return Self::score_card_with_joker(self.cards[i])
                            .unwrap()
                            .cmp(&Self::score_card_with_joker(other.cards[i]).unwrap());
                    }
                }

                Ordering::Equal
            }
    }
}

