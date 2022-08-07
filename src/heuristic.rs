// Copyright (C) 2022 Andrew Archibald
//
// deadfish is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version. You should have received a copy of the GNU Lesser General
// Public License along with deadfish. If not, see http://www.gnu.org/licenses/.

//! # Heuristic
//!
//! Squaring a number doubles the number of trailing zeros. That is, for all
//! values `n: u32`,
//! `n.wrapping_mul(n).trailing_zeros() == (2 * n.trailing_zeros()).min(32)`.

use std::collections::VecDeque;

use crate::Inst;

#[must_use]
pub fn encode_from_zero(n: i32) -> Vec<Inst> {
    let mut n = n as u64;
    let mut path = VecDeque::new();
    path.push_front(Inst::O);
    loop {
        if n < 4 {
            repeat(&mut path, Inst::I, n);
            return path.into();
        }
        let (sqrt, offset, direction) = nearest_sqrt(n);
        repeat(&mut path, direction, offset);
        path.push_front(Inst::S);
        n = sqrt;
    }
}

#[inline]
fn nearest_sqrt(n: u64) -> (u64, u64, Inst) {
    let sqrt = (n as f64).sqrt();
    let floor = sqrt.floor() as u64;
    let ceil = sqrt.ceil() as u64;
    if n - floor * floor < ceil * ceil - n {
        (floor, n - floor * floor, Inst::I)
    } else {
        (ceil, ceil * ceil - n, Inst::D)
    }
}

#[inline]
fn repeat(path: &mut VecDeque<Inst>, inst: Inst, count: u64) {
    path.reserve(count as usize);
    for _ in 0..count {
        path.push_front(inst);
    }
}

pub fn append_overflow_to_zero(path: &mut Vec<Inst>, acc: i32) {
    let (offset, squares) = count_overflow_to_zero(acc);
    let (offset, direction) = if offset >= 0 {
        (offset as u32, Inst::I)
    } else {
        (-offset as u32, Inst::D)
    };
    path.reserve(offset as usize + squares as usize + 1);
    repeat_vec(path, direction, offset);
    repeat_vec(path, Inst::S, squares);
    path.push(Inst::O);
}

#[must_use]
#[inline]
pub const fn count_overflow_to_zero(acc: i32) -> (i32, u32) {
    let mut acc = acc;
    let mut tz = acc.trailing_zeros();
    let mut offset = 0;
    if tz < 2 {
        offset = if tz == 1 {
            match acc & 0b1111 {
                // Offset to have 4+ trailing zeros
                0b1110 => 2,
                0b0010 => -2,
                // Use the 1 trailing zero
                _ => 0,
            }
        } else {
            match acc & 0b1111_1111 {
                // Offset to have 8+ trailing zeros
                0b1111_1101 => 3,
                0b0000_0011 => -3,
                // Offset to have 2+ trailing zeros (0b11 => 1, 0b01 => -1)
                _ => (acc & 0b11) - 2,
            }
        };
        acc = acc.wrapping_add(offset);
        tz = acc.trailing_zeros();
    }
    // log2(32) - floor(log2(tz))
    let squares = tz.leading_zeros() - 32u32.leading_zeros();
    (offset, squares)
}

#[inline]
fn repeat_vec(path: &mut Vec<Inst>, inst: Inst, count: u32) {
    for _ in 0..count {
        path.push(inst);
    }
}
