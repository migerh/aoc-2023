use std::{str::FromStr, cmp::Ordering};

use anyhow::{Error, Result, Context};

use crate::utils::AocError::*;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(s: char) -> Result<Self> {
        use Card::*;

        Ok(match s {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'J' => Jack,
            'T' => Ten,
            '9' => Nine,
            '8' => Eight,
            '7' => Seven,
            '6' => Six,
            '5' => Five,
            '4' => Four,
            '3' => Three,
            '2' => Two,
            'X' => Joker,
            _ => Err(GenericError).context("Could not parse card")?
        })
    }
}

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hand {
    pub cards: Vec<Card>,
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
            .map(Card::from_char)
            .collect::<Result<Vec<_>>>()?;

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
    pub fn partition(hand: &[Card]) -> Vec<u8> {
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

    pub fn to_joker(&self) -> Self {
        use Card::*;
        let cards = self.cards.iter().map(|c| if *c == Jack { Joker } else { c.clone() }).collect::<Vec<_>>();
        let bid = self.bid;
        Hand { cards, bid }
    }

    pub fn kind(&self) -> Kind {
        use Card::*;
        let mut scores = self.cards.clone();
        scores.sort();

        let mut partition = Self::partition(&scores);
        partition.sort();

        if partition.len() == 1 && partition[0] == 5 {
            return Kind::Five;
        }

        if partition.len() == 2 && partition[0] == 1 && partition[1] == 4 {
            if scores[0] == Joker {
                return Kind::Five;
            }

            return Kind::Four;
        }

        if partition.len() == 2 && partition[0] == 2 && partition[1] == 3 {
            if scores[0] == Joker {
                return Kind::Five;
            }
            return Kind::FullHouse;
        }

        if partition.len() == 3 && partition[0] == 1 && partition[1] == 1 && partition[2] == 3 {
            if scores[0] == Joker {
                return Kind::Four;
            }
            return Kind::Three;
        }

        if partition.len() == 3 && partition[0] == 1 && partition[1] == 2 && partition[2] == 2 {
            if scores[1] == Joker {
                return Kind::Four;
            }

            if scores[0] == Joker {
                return Kind::FullHouse;
            }
            return Kind::TwoPair;
        }

        if partition.len() == 4 {
            if scores[0] == Joker {
                return Kind::Three;
            }
            return Kind::OnePair;
        }

        if scores[0] == Joker {
            return Kind::OnePair;
        }

        Kind::HighCard
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
            let self_kind = self.kind();
            let other_kind = other.kind();
            let kind_match = self_kind == other_kind;

            if !kind_match {
                self_kind.cmp(&other_kind)
            } else {
                for i in 0..5 {
                    if self.cards[i] != other.cards[i] {
                        return self.cards[i].cmp(&other.cards[i]);
                    }
                }

                Ordering::Equal
            }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

