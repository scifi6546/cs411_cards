#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Suit {
    Clubs = 0,
    Hearts = 1,
    Spades = 2,
    Diamonds = 3,
}
impl Suit {
    fn from_num(num: usize) -> Option<Self> {
        match num {
            0 => Some(Self::Clubs),
            1 => Some(Self::Hearts),
            2 => Some(Self::Spades),
            3 => Some(Self::Diamonds),
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
