// Copyright (C) 2022 Andrew Archibald
//
// deadfish is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version. You should have received a copy of the GNU Lesser General
// Public License along with deadfish. If not, see http://www.gnu.org/licenses/.

/// Deadfish instructions.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Inst {
    /// Increment
    I,
    /// Decrement
    D,
    /// Square
    S,
    /// Output
    O,
    /// Print a line feed
    Blank,
}

impl Inst {
    #[must_use]
    #[inline]
    pub const fn apply(&self, acc: i32) -> i32 {
        let acc = match self {
            Inst::I => acc.wrapping_add(1),
            Inst::D => acc.wrapping_sub(1),
            Inst::S => acc.wrapping_mul(acc),
            _ => acc,
        };
        if acc == -1 || acc == 256 {
            0
        } else {
            acc
        }
    }

    #[must_use]
    #[inline]
    pub fn apply_inverse(&self, acc: i32) -> Option<i32> {
        let acc = match self {
            Inst::I => acc.wrapping_sub(1),
            Inst::D => acc.wrapping_add(1),
            Inst::S => {
                let sqrt = (acc as f64).sqrt() as i32;
                if sqrt.wrapping_mul(sqrt) != acc {
                    return None;
                }
                sqrt
            }
            _ => acc,
        };
        if acc == -1 || acc == 256 {
            None
        } else {
            Some(acc)
        }
    }
}
