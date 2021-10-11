#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Suit {
    CLUBS = 0,
    HEARTS = 1,
    SPADES = 2,
    DIAMONDS = 3,
}
impl Suit {
    fn from_num(num: usize) -> Option<Self> {
        match num {
            0 => Some(Self::CLUBS),
            1 => Some(Self::HEARTS),
            2 => Some(Self::SPADES),
            3 => Some(Self::DIAMONDS),
            _ => None,
        }
    }
}
impl PartialOrd for Suit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (*self as usize).partial_cmp(&(*other as usize))
    }
}
impl std::iter::Step for Suit {
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
