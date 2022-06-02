use num::traits::Unsigned;
use std::{cmp::PartialOrd, convert::TryFrom, marker::Copy, ops::{Add, Sub, AddAssign, SubAssign}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SignedInt<T: Unsigned> {
    Pos(T),
    Neg(T),
}

impl<T: Unsigned + Copy> Copy for SignedInt<T> {}

impl<T: Unsigned + PartialOrd> Add<T> for SignedInt<T> {
    type Output = Self;

    fn add(self, other: T) -> Self {
        match self {
            Self::Pos(num) => Self::Pos(num + other),
            Self::Neg(num) => {
                if num < other {
                    Self::Pos(other - num)
                } else {
                    Self::Neg(num - other)
                }
            },
        }
    }
}

impl<T: Unsigned + PartialOrd> Sub<T> for SignedInt<T> {
    type Output = Self;

    fn sub(self, other: T) -> Self {
        match self {
            Self::Neg(num) => Self::Neg(num + other),
            Self::Pos(num) => {
                if num < other {
                    Self::Neg(other - num)
                } else {
                    Self::Pos(num - other)
                }
            },
        }
    }
}

impl<T: Unsigned + PartialOrd + Copy> AddAssign<T> for SignedInt<T> {
    fn add_assign(&mut self, other: T) {
        *self = self.add(other);
    }
}

impl<T: Unsigned + PartialOrd + Copy> SubAssign<T> for SignedInt<T> {
    fn sub_assign(&mut self, other: T) {
        *self = self.sub(other);
    }
}

impl<T: Unsigned + TryFrom<isize>> TryFrom<isize> for SignedInt<T> {
    type Error = <T as TryFrom<isize>>::Error;

    fn try_from(num: isize) -> Result<Self, <T as TryFrom<isize>>::Error> {
        if num.is_positive() {
            match T::try_from(num) {
                Ok(num) => {
                    Ok(Self::Pos(num))
                },
                Err(err) => Err(err),
            }
        } else {
            match T::try_from(num.abs()) {
                Ok(num) => Ok(Self::Neg(num)),
                Err(err) => Err(err),
            }
        }
    }
}
