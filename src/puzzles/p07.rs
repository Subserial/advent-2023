#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Hand {
    High(String),
    OnePair(String),
    TwoPair(String),
    ThreeKind(String),
    FullHouse(String),
    FourKind(String),
    FiveKind(String),
}

impl Hand {
    fn as_count(&self) -> usize {
        match self {
            Hand::High(_) => 5,
            Hand::OnePair(_) => 7,
            Hand::TwoPair(_) => 9,
            Hand::ThreeKind(_) => 11,
            Hand::FullHouse(_) => 13,
            Hand::FourKind(_) => 17,
            Hand::FiveKind(_) => 25,
        }
    }

    fn from(card: String, val: usize) -> Self {
        match val {
            25 => Hand::FiveKind(card),
            17 => Hand::FourKind(card),
            13 => Hand::FullHouse(card),
            11 => Hand::ThreeKind(card),
            9 => Hand::TwoPair(card),
            7 => Hand::OnePair(card),
            5 => Hand::High(card),
            _ => panic!(),
        }
    }

    fn count(card: &str) -> usize {
        card.chars().map(|c| card.matches(c).count()).sum::<usize>()
    }

    fn from_one(s: &str) -> Self {
        let card = s
            .replace("A", "Z")
            .replace("K", "Y")
            .replace("Q", "X")
            .replace("J", "W")
            .replace("T", "V");
        let count = Hand::count(&card);
        Hand::from(card, count)
    }

    fn from_two(s: &str) -> Self {
        let card = s
            .replace("A", "Z")
            .replace("K", "Y")
            .replace("Q", "X")
            .replace("J", "0")
            .replace("T", "V");
        let best = "ZYXV987654321"
            .chars()
            .map(|c| Hand::count(&card.replace('0', &String::from(c))))
            .max()
            .unwrap();
        Hand::from(card, best)
    }
}

pub fn run_one(data: &str) -> String {
    let mut hands: Vec<(Hand, u32)> = data
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" ").unwrap();
            (Hand::from_one(left), right.parse::<u32>().unwrap())
        })
        .collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, val))| (i as u32 + 1) * *val)
        .sum::<u32>()
        .to_string()
}

pub fn run_two(data: &str) -> String {
    let mut hands: Vec<(Hand, u32)> = data
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" ").unwrap();
            (Hand::from_two(left), right.parse::<u32>().unwrap())
        })
        .collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, val))| (i as u32 + 1) * *val)
        .sum::<u32>()
        .to_string()
}
