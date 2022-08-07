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

#[must_use]
pub fn encode_to_zero(acc: i32) -> Vec<Inst> {
    let mut path = Vec::with_capacity(6);
    let mut acc = acc;
    let mut tz = acc.trailing_zeros();
    if tz < 2 {
        let offset: i32 = if acc & 0b1111_1111 == 0b1111_1101 {
            3
        } else if acc & 0b1111_1111 == 0b0000_0011 {
            -3
        } else if acc & 0b1111 == 0b1110 {
            2
        } else if acc & 0b1111 == 0b0010 {
            -2
        } else if acc & 0b11 == 0b11 {
            1
        } else if acc & 0b11 == 0b01 {
            -1
        } else {
            0
        };
        if offset != 0 {
            if offset > 0 {
                repeat_vec(&mut path, Inst::I, offset as u32);
            } else {
                repeat_vec(&mut path, Inst::D, -offset as u32);
            }
            acc += offset;
            tz = acc.trailing_zeros();
        }
    }
    // log2(32) - floor(log2(tz))
    let squares = tz.leading_zeros() - 32u32.leading_zeros();
    repeat_vec(&mut path, Inst::S, squares);
    path.push(Inst::O);
    path
}

#[inline]
fn repeat_vec(path: &mut Vec<Inst>, inst: Inst, count: u32) {
    path.reserve(count as usize);
    for _ in 0..count {
        path.push(inst);
    }
}
