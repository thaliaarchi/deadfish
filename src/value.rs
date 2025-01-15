use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

use crate::Inst;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]

pub struct Value(pub(crate) u32);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Offset(pub(crate) i64);

impl Value {
    /// Create a new Deadfish value of zero.
    #[inline]
    pub const fn new() -> Self {
        Value(0)
    }

    #[inline]
    pub fn from_checked(n: u32) -> Option<Self> {
        if n == normalize(n) {
            Some(Value(n))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn from_raw(n: u32) -> Self {
        debug_assert!(n == normalize(n));
        Value(n)
    }

    #[inline]
    pub const fn value(self) -> u32 {
        self.0
    }

    /// Compute the operation on the value.
    #[inline]
    pub fn apply(self, inst: Inst) -> Self {
        match inst {
            Inst::I => self.increment(),
            Inst::D => self.decrement(),
            Inst::S => self.square(),
            _ => self,
        }
    }

    /// Compute the inverse operation on the value, if possible.
    #[inline]
    pub fn apply_inverse(self, inst: Inst) -> Option<Self> {
        Value::from_checked(match inst {
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
        })
    }

    pub fn increment(self) -> Self {
        Value::from(self.0.wrapping_add(1))
    }

    pub fn decrement(self) -> Self {
        Value::from(self.0.wrapping_sub(1))
    }

    pub fn square(self) -> Self {
        Value::from(self.0.wrapping_mul(self.0))
    }

    #[inline]
    pub const fn saturating_add(self, rhs: u32) -> Self {
        let add = self.0.saturating_add(rhs);
        if self.0 < 256 && add >= 256 {
            Value(255)
        } else if add == u32::MAX {
            Value(u32::MAX - 1)
        } else {
            Value(add)
        }
    }

    #[inline]
    pub const fn saturating_sub(self, rhs: u32) -> Self {
        let sub = self.0.saturating_sub(rhs);
        if self.0 > 256 && sub <= 256 {
            Value(257)
        } else if sub == 0 && self.0 != 0 {
            Value(1)
        } else {
            Value(sub)
        }
    }

    #[inline]
    pub fn square_repeat(self, count: u32) -> Self {
        let mut n = self.0;
        for _ in 0..count {
            n = normalize(n.wrapping_mul(n));
            if n == 0 {
                break;
            }
        }
        Value(n)
    }

    #[inline]
    pub fn nearest_sqrt(&self) -> (Value, Offset) {
        let sqrt = (self.0 as f64).sqrt();
        let floor = sqrt.floor() as u32;
        let ceil = sqrt.ceil() as u32;
        let floor_diff = self.0 - floor * floor;
        let ceil_diff = ceil * ceil - self.0;
        // Choose the closer square root and avoid squaring to 256 or 1 << 32
        if floor_diff < ceil_diff && floor != 16 || ceil == 16 || ceil == 65536 {
            (Value(floor), Offset(floor_diff as i64))
        } else {
            (Value(ceil), Offset(-(ceil_diff as i64)))
        }
    }

    #[inline]
    pub const fn offset_to(self, other: Value) -> Option<Offset> {
        if (self.0 < 256) == (other.0 < 256) {
            Some(Offset(other.0 as i64 - self.0 as i64))
        } else {
            None
        }
    }
}

impl Offset {
    #[inline]
    pub const fn new(offset: u32, is_negative: bool) -> Self {
        if is_negative {
            Offset(-(offset as i64))
        } else {
            Offset(offset as i64)
        }
    }

    #[inline]
    pub fn abs(&self) -> u32 {
        self.0.unsigned_abs().try_into().unwrap_or(u32::MAX)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.abs() as usize
    }

    #[inline]
    pub const fn is_negative(&self) -> bool {
        self.0 < 0
    }
}

impl Add<u32> for Value {
    type Output = Value;

    #[inline]
    fn add(self, rhs: u32) -> Self::Output {
        let add = self.0.saturating_add(rhs);
        if self.0 < 256 && add >= 256 || add == u32::MAX {
            Value(0)
        } else {
            Value(add)
        }
    }
}

impl AddAssign<u32> for Value {
    fn add_assign(&mut self, rhs: u32) {
        *self = *self + rhs;
    }
}

impl Sub<u32> for Value {
    type Output = Value;

    #[inline]
    fn sub(self, rhs: u32) -> Self::Output {
        let sub = self.0.saturating_sub(rhs);
        if self.0 > 256 && sub <= 256 {
            Value(0)
        } else {
            Value(sub)
        }
    }
}

impl SubAssign<u32> for Value {
    fn sub_assign(&mut self, rhs: u32) {
        *self = *self - rhs;
    }
}

impl Add<Offset> for Value {
    type Output = Value;

    #[inline]
    fn add(self, rhs: Offset) -> Self::Output {
        if rhs.0 >= 0 {
            self + rhs.abs()
        } else {
            self - rhs.abs()
        }
    }
}

impl AddAssign<Offset> for Value {
    fn add_assign(&mut self, rhs: Offset) {
        *self = *self + rhs;
    }
}

impl Sub<Offset> for Value {
    type Output = Value;

    #[inline]
    fn sub(self, rhs: Offset) -> Self::Output {
        self + -rhs
    }
}

impl SubAssign<Offset> for Value {
    fn sub_assign(&mut self, rhs: Offset) {
        *self = *self - rhs;
    }
}

impl Neg for Offset {
    type Output = Offset;

    #[inline]
    fn neg(self) -> Self::Output {
        Offset(self.0.saturating_neg())
    }
}

impl PartialEq<u32> for Value {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u32> for Value {
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

impl PartialOrd for Offset {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Offset {
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

impl Default for Value {
    #[inline]
    fn default() -> Self {
        Value::new()
    }
}

impl Default for Offset {
    #[inline]
    fn default() -> Self {
        Offset(0)
    }
}

impl From<u32> for Value {
    #[inline]
    fn from(n: u32) -> Self {
        Value(normalize(n))
    }
}

impl From<i32> for Value {
    #[inline]
    fn from(n: i32) -> Self {
        Value(normalize(n as u32))
    }
}

impl From<Value> for u32 {
    #[inline]
    fn from(v: Value) -> Self {
        v.0
    }
}

impl From<Value> for i32 {
    #[inline]
    fn from(v: Value) -> Self {
        v.0 as i32
    }
}

impl From<i64> for Offset {
    #[inline]
    fn from(offset: i64) -> Self {
        Offset(offset)
    }
}

impl Display for Value {
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
fn normalize(n: u32) -> u32 {
    if n == 256 || n == u32::MAX {
        0
    } else {
        n
    }
}
