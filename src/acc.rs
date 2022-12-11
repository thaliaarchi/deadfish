// Copyright (C) 2022 Thalia Archibald
//
// deadfish is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version. You should have received a copy of the GNU Lesser General
// Public License along with deadfish. If not, see http://www.gnu.org/licenses/.

use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use crate::Inst;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]

pub struct Acc(u32);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Offset(pub i64);

impl Acc {
    /// Create a new accumulator at zero.
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Acc(0)
    }

    #[must_use]
    #[inline]
    pub const fn from_checked(n: u32) -> Option<Self> {
        if n == normalize(n) {
            Some(Acc(n))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) const fn from_raw(n: u32) -> Self {
        debug_assert!(n == normalize(n));
        Acc(n)
    }

    #[must_use]
    #[inline]
    pub const fn value(self) -> u32 {
        self.0
    }

    /// Compute the operation on the accumulator.
    #[must_use]
    #[inline]
    pub const fn apply(self, inst: Inst) -> Self {
        match inst {
            Inst::I => self.increment(),
            Inst::D => self.decrement(),
            Inst::S => self.square(),
            _ => self,
        }
    }

    /// Compute the inverse operation on the accumulator, if possible.
    #[must_use]
    #[inline]
    pub fn apply_inverse(self, inst: Inst) -> Option<Self> {
        let n = match inst {
            Inst::I => self.0.wrapping_sub(1),
            Inst::D => self.0.wrapping_add(1),
            Inst::S => {
                let sqrt = (self.0 as f64).sqrt();
                if sqrt.floor() != sqrt.ceil() {
                    return None;
                }
                sqrt as u32
            }
            _ => return Some(self),
        };
        if n == normalize(n) {
            Some(Acc(n))
        } else {
            None
        }
    }

    #[must_use]
    pub const fn increment(self) -> Self {
        Acc(normalize(self.0.wrapping_add(1)))
    }

    #[must_use]
    pub const fn decrement(self) -> Self {
        Acc(normalize(self.0.wrapping_sub(1)))
    }

    #[must_use]
    pub const fn square(self) -> Self {
        Acc(normalize(self.0.wrapping_mul(self.0)))
    }

    #[must_use]
    #[inline]
    pub const fn saturating_add(self, rhs: u32) -> Self {
        let add = self.0.saturating_add(rhs);
        if self.0 < 256 && add >= 256 {
            Acc(255)
        } else if add == u32::MAX {
            Acc(u32::MAX - 1)
        } else {
            Acc(add)
        }
    }

    #[must_use]
    #[inline]
    pub const fn saturating_sub(self, rhs: u32) -> Self {
        let sub = self.0.saturating_sub(rhs);
        if self.0 > 256 && sub <= 256 {
            Acc(257)
        } else if sub == 0 && self.0 != 0 {
            Acc(1)
        } else {
            Acc(sub)
        }
    }

    #[must_use]
    #[inline]
    pub fn square_repeat(self, count: u32) -> Self {
        let mut n = self.0;
        for _ in 0..count {
            n = normalize(n.wrapping_mul(n));
            if n == 0 {
                break;
            }
        }
        Acc(n)
    }

    #[must_use]
    #[inline]
    pub fn nearest_sqrt(&self) -> (Acc, Offset) {
        let sqrt = (self.0 as f64).sqrt();
        let floor = sqrt.floor() as u32;
        let ceil = sqrt.ceil() as u32;
        let floor_diff = self.0 - floor * floor;
        let ceil_diff = ceil * ceil - self.0;
        // Choose the closer square root and avoid squaring to 256 or 1 << 32
        if floor_diff < ceil_diff && floor != 16 || ceil == 16 || ceil == 65536 {
            (Acc::from_raw(floor), Offset(floor_diff as i64))
        } else {
            (Acc::from_raw(ceil), Offset(-(ceil_diff as i64)))
        }
    }

    #[must_use]
    #[inline]
    pub const fn offset_to(self, other: Acc) -> Option<Offset> {
        if (self.0 < 256) == (other.0 < 256) {
            Some(Offset(other.0 as i64 - self.0 as i64))
        } else {
            None
        }
    }
}

impl Offset {
    #[must_use]
    #[inline]
    pub const fn new(offset: u32, is_negative: bool) -> Self {
        if is_negative {
            Offset(-(offset as i64))
        } else {
            Offset(offset as i64)
        }
    }

    #[must_use]
    #[inline]
    pub const fn abs(&self) -> u32 {
        self.0.unsigned_abs().try_into().unwrap_or(u32::MAX)
    }

    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        self.abs() as usize
    }

    #[must_use]
    #[inline]
    pub const fn is_negative(&self) -> bool {
        self.0 < 0
    }
}

impl const Add<u32> for Acc {
    type Output = Acc;

    #[inline]
    fn add(self, rhs: u32) -> Self::Output {
        let add = self.0.saturating_add(rhs);
        if self.0 < 256 && add >= 256 || add == u32::MAX {
            Acc(0)
        } else {
            Acc(add)
        }
    }
}

impl const AddAssign<u32> for Acc {
    fn add_assign(&mut self, rhs: u32) {
        *self = *self + rhs;
    }
}

impl const Sub<u32> for Acc {
    type Output = Acc;

    #[inline]
    fn sub(self, rhs: u32) -> Self::Output {
        let sub = self.0.saturating_sub(rhs);
        if self.0 > 256 && sub <= 256 {
            Acc(0)
        } else {
            Acc(sub)
        }
    }
}

impl const SubAssign<u32> for Acc {
    fn sub_assign(&mut self, rhs: u32) {
        *self = *self - rhs;
    }
}

impl const Add<Offset> for Acc {
    type Output = Acc;

    #[inline]
    fn add(self, rhs: Offset) -> Self::Output {
        if rhs.0 >= 0 {
            self + rhs.abs()
        } else {
            self - rhs.abs()
        }
    }
}

impl const Sub<Offset> for Acc {
    type Output = Acc;

    #[inline]
    fn sub(self, rhs: Offset) -> Self::Output {
        self + -rhs
    }
}

impl const Neg for Offset {
    type Output = Offset;

    #[inline]
    fn neg(self) -> Self::Output {
        Offset(self.0.saturating_neg())
    }
}

impl const PartialEq<u32> for Acc {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl const PartialOrd<u32> for Acc {
    #[inline]
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        if self.0 == *other {
            Some(Ordering::Equal)
        } else if self.0 == 0 && normalize(*other) == 0 {
            None
        } else if self.0 < *other {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl const PartialOrd for Offset {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl const Ord for Offset {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 == other.0 {
            Ordering::Equal
        } else {
            let x = self.0.unsigned_abs();
            let y = other.0.unsigned_abs();
            if x < y || x == y && !self.is_negative() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}

impl const Default for Acc {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl const From<u32> for Acc {
    #[inline]
    fn from(n: u32) -> Self {
        Acc(normalize(n))
    }
}

impl const From<i32> for Acc {
    #[inline]
    fn from(n: i32) -> Self {
        Acc(normalize(n as u32))
    }
}

impl const From<Acc> for u32 {
    #[inline]
    fn from(acc: Acc) -> Self {
        acc.0
    }
}

impl const From<Acc> for i32 {
    #[inline]
    fn from(acc: Acc) -> Self {
        acc.0 as i32
    }
}

impl const From<i64> for Offset {
    #[inline]
    fn from(offset: i64) -> Self {
        Offset(offset)
    }
}

impl Display for Acc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0 as i32)
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[inline]
const fn normalize(n: u32) -> u32 {
    if n == 256 || n == u32::MAX {
        0
    } else {
        n
    }
}
