#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Rank {
    A = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}
impl Rank {
    fn from_num(num: usize) -> Option<Self> {
        match num {
            1 => Some(Self::A),
            2 => Some(Self::Two),
            3 => Some(Self::Three),
            4 => Some(Self::Four),
            5 => Some(Self::Five),
            6 => Some(Self::Six),
            7 => Some(Self::Seven),
            8 => Some(Self::Eight),
            9 => Some(Self::Nine),
            10 => Some(Self::Ten),
            11 => Some(Self::Jack),
            12 => Some(Self::Queen),
            13 => Some(Self::King),
            _ => None,
        }
    }
}
impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (*self as usize).partial_cmp(&(*other as usize))
    }
}
impl Ord for Rank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}
impl std::iter::Step for Rank {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        Some(*end as usize - *start as usize)
    }
    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        let num = (start as usize) + count;
        Self::from_num(num)
    }
    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        let start_num = start as usize;
        if start_num < count {
            None
        } else {
            Self::from_num(start_num - count)
        }
    }
}
