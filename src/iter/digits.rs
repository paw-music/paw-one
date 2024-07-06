use num::Integer;
use num::{One, Zero};

#[derive(Clone, Copy, defmt::Format, Debug)]
pub enum Edge {
    Rising,
    Falling,
}

impl From<debouncr::Edge> for Edge {
    fn from(value: debouncr::Edge) -> Self {
        match value {
            debouncr::Edge::Rising => Self::Rising,
            debouncr::Edge::Falling => Self::Falling,
        }
    }
}

/// Structure to iterate through digits of arbitrary number with defined base.
/// IMPORTANT: Numbers are enumerated from right to left, so in 123, the first digit is 3, not 1.
pub struct DigitsIter<N: Integer> {
    n: N,
    base: N,
    // div: N,
}

impl<N: Integer + core::ops::Div + core::ops::MulAssign + Copy> DigitsIter<N> {
    pub fn new(n: N, base: N) -> Self {
        Self { n, base }
    }
}

impl<N: Integer + Copy + core::ops::RemAssign + core::ops::DivAssign> Iterator for DigitsIter<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == N::zero() {
            None
        } else {
            let digit = self.n % self.base;
            self.n /= self.base;
            Some(digit)
        }
    }

    // fn next(&mut self) -> Option<Self::Item> {
    //     if self.n == N::zero() {
    //         None
    //     } else {
    //         let digit = self.n / self.div;
    //         self.n %= self.div;
    //         self.div /= self.base;
    //         Some(digit)
    //     }
    // }
}

pub trait Digits: Sized {
    type Int: Integer;

    fn digits(self, base: Self::Int) -> DigitsIter<Self::Int>;

    fn edges(self, other: Self, base: Self::Int) -> impl Iterator<Item = Option<Edge>>
    where
        Self::Int: Integer + Copy + core::ops::RemAssign + core::ops::DivAssign,
    {
        let bits = core::mem::size_of::<Self::Int>() * 8;
        let zero = Self::Int::zero();
        let one = Self::Int::one();

        self.digits(base)
            .chain(core::iter::repeat(zero))
            .zip(other.digits(base).chain(core::iter::repeat(zero)))
            .take(bits)
            .map(move |edge| {
                if edge == (zero, one) {
                    Some(Edge::Rising)
                } else if edge == (one, zero) {
                    Some(Edge::Falling)
                } else {
                    None
                }
            })
    }
}

impl<N: Integer + core::ops::Mul + core::ops::MulAssign + Copy> Digits for N {
    type Int = N;

    fn digits(self, base: N) -> DigitsIter<N> {
        DigitsIter::new(self, base)
    }
}
