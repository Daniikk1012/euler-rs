use std::{error::Error, fmt::Display, fs, str::FromStr, time::SystemTime};

#[derive(Debug)]
enum CardParseError {
    Rank,
    Suit,
    Format,
}

impl Display for CardParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardParseError::Rank => write!(f, "Invalid rank"),
            CardParseError::Suit => write!(f, "Invalid card"),
            CardParseError::Format => write!(f, "Invalid format"),
        }
    }
}

impl Error for CardParseError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    One,
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

impl FromStr for Rank {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Rank::One),
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            "9" => Ok(Rank::Nine),
            "T" => Ok(Rank::Ten),
            "J" => Ok(Rank::Jack),
            "Q" => Ok(Rank::Queen),
            "K" => Ok(Rank::King),
            "A" => Ok(Rank::Ace),
            _ => Err(CardParseError::Rank),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl FromStr for Suit {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Suit::Clubs),
            "D" => Ok(Suit::Diamonds),
            "H" => Ok(Suit::Hearts),
            "S" => Ok(Suit::Spades),
            _ => Err(CardParseError::Suit),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl FromStr for Card {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 2 {
            Ok(Card { rank: s[..1].parse()?, suit: s[1..].parse()? })
        } else {
            Err(CardParseError::Format)
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard([Rank; 5]),
    OnePair([Rank; 4]),
    TwoPair([Rank; 3]),
    ThreeSame([Rank; 3]),
    Straight(Rank),
    Flush([Rank; 5]),
    FullHouse([Rank; 2]),
    FourSame([Rank; 2]),
    StraightFlush(Rank),
    RoyalFlush,
}

impl Hand {
    fn new(mut cards: [Card; 5]) -> Self {
        cards.sort_unstable_by_key(|&Card { rank, .. }| rank);

        if cards.iter().all(|&Card { suit, .. }| suit == cards[0].suit)
            && cards[0].rank == Rank::Ten
            && cards[1].rank == Rank::Jack
            && cards[2].rank == Rank::Queen
            && cards[3].rank == Rank::King
            && cards[4].rank == Rank::Ace
        {
            Hand::RoyalFlush
        } else if cards.iter().all(|&Card { suit, .. }| suit == cards[0].suit)
            && cards
                .windows(2)
                .all(|window| window[0].rank as u8 + 1 == window[1].rank as u8)
        {
            Hand::StraightFlush(cards[0].rank)
        } else if cards[..4]
            .iter()
            .all(|&Card { rank, .. }| rank == cards[0].rank)
        {
            Hand::FourSame([cards[0].rank, cards[4].rank])
        } else if cards[1..]
            .iter()
            .all(|&Card { rank, .. }| rank == cards[1].rank)
        {
            Hand::FourSame([cards[1].rank, cards[0].rank])
        } else if cards[0].rank == cards[1].rank
            && cards[2].rank == cards[3].rank
            && cards[2].rank == cards[4].rank
        {
            Hand::FullHouse([cards[2].rank, cards[0].rank])
        } else if cards[0].rank == cards[1].rank
            && cards[0].rank == cards[2].rank
            && cards[3].rank == cards[4].rank
        {
            Hand::FullHouse([cards[0].rank, cards[3].rank])
        } else if cards.iter().all(|&Card { suit, .. }| suit == cards[0].suit) {
            Hand::Flush([
                cards[4].rank,
                cards[3].rank,
                cards[2].rank,
                cards[1].rank,
                cards[0].rank,
            ])
        } else if cards
            .windows(2)
            .all(|window| window[0].rank as u8 + 1 == window[1].rank as u8)
        {
            Hand::Straight(cards[0].rank)
        } else if cards[0].rank == cards[1].rank
            && cards[0].rank == cards[2].rank
        {
            Hand::ThreeSame([cards[0].rank, cards[4].rank, cards[3].rank])
        } else if cards[1].rank == cards[2].rank
            && cards[1].rank == cards[3].rank
        {
            Hand::ThreeSame([cards[1].rank, cards[4].rank, cards[0].rank])
        } else if cards[2].rank == cards[3].rank
            && cards[2].rank == cards[4].rank
        {
            Hand::ThreeSame([cards[2].rank, cards[1].rank, cards[0].rank])
        } else if cards[0].rank == cards[1].rank
            && cards[2].rank == cards[3].rank
        {
            Hand::TwoPair([cards[2].rank, cards[0].rank, cards[4].rank])
        } else if cards[0].rank == cards[1].rank
            && cards[3].rank == cards[4].rank
        {
            Hand::TwoPair([cards[3].rank, cards[0].rank, cards[2].rank])
        } else if cards[1].rank == cards[2].rank
            && cards[3].rank == cards[4].rank
        {
            Hand::TwoPair([cards[3].rank, cards[1].rank, cards[0].rank])
        } else if cards[0].rank == cards[1].rank {
            Hand::OnePair([
                cards[0].rank,
                cards[4].rank,
                cards[3].rank,
                cards[2].rank,
            ])
        } else if cards[1].rank == cards[2].rank {
            Hand::OnePair([
                cards[1].rank,
                cards[4].rank,
                cards[3].rank,
                cards[0].rank,
            ])
        } else if cards[2].rank == cards[3].rank {
            Hand::OnePair([
                cards[2].rank,
                cards[4].rank,
                cards[1].rank,
                cards[0].rank,
            ])
        } else if cards[3].rank == cards[4].rank {
            Hand::OnePair([
                cards[3].rank,
                cards[2].rank,
                cards[1].rank,
                cards[0].rank,
            ])
        } else {
            Hand::HighCard([
                cards[4].rank,
                cards[3].rank,
                cards[2].rank,
                cards[1].rank,
                cards[0].rank,
            ])
        }
    }
}

fn main() {
    let time = SystemTime::now();

    let result = fs::read_to_string("inputs/54.txt")
        .unwrap()
        .lines()
        .filter(|&line| {
            let vec: Vec<_> =
                line.split(' ').map(|s| s.parse().unwrap()).collect();
            Hand::new(vec[..5].to_vec().try_into().unwrap())
                > Hand::new(vec[5..].to_vec().try_into().unwrap())
        })
        .count();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
