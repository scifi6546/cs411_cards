#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Rank {
    A = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
    NINE = 9,
    TEN = 10,
    JACK = 11,
    QUEEN = 12,
    KING = 13,
}
impl Rank {
    fn from_num(num: usize) -> Option<Self> {
        match num {
            1 => Some(Self::A),
            2 => Some(Self::TWO),
            3 => Some(Self::THREE),
            4 => Some(Self::FOUR),
            5 => Some(Self::FIVE),
            6 => Some(Self::SIX),
            7 => Some(Self::SEVEN),
            8 => Some(Self::EIGHT),
            9 => Some(Self::NINE),
            10 => Some(Self::TEN),
            11 => Some(Self::JACK),
            12 => Some(Self::QUEEN),
            13 => Some(Self::KING),
            _ => None,
        }
    }
}
impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (*self as usize).partial_cmp(&(*other as usize))
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
