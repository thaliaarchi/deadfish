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
    pub const fn apply(&self, n: i32) -> i32 {
        let n = match self {
            Inst::I => n.wrapping_add(1),
            Inst::D => n.wrapping_sub(1),
            Inst::S => n.wrapping_mul(n),
            _ => n,
        };
        if n == -1 || n == 256 {
            0
        } else {
            n
        }
    }

    #[must_use]
    #[inline]
    pub fn apply_inverse(&self, n: i32) -> Option<i32> {
        let n = match self {
            Inst::I => n.wrapping_sub(1),
            Inst::D => n.wrapping_add(1),
            Inst::S => {
                let sqrt = (n as f64).sqrt() as i32;
                if sqrt.wrapping_mul(sqrt) != n {
                    return None;
                }
                sqrt
            }
            _ => n,
        };
        if n == -1 || n == 256 {
            None
        } else {
            Some(n)
        }
    }
}
