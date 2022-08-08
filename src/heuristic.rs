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
//!
//! # Optimization
//!
//! Prefer shorter paths with fewer squares.

use std::collections::VecDeque;

use crate::{normalize, Inst};

pub fn encode(insts: &mut Vec<Inst>, acc: i32, n: i32) {
    let acc = normalize(acc);
    let n = normalize(n);

    let (offset_to, squares_to) = encode_to_zero(acc);
    let (offsets_from, len_from) = encode_from_zero(n);
    let len = offset_to.unsigned_abs() as usize + squares_to as usize + len_from;

    if len < n.abs_diff(acc) as usize {
        push_offset(insts, offset_to);
        push_repeat(insts, Inst::S, squares_to);
        if let Some(&first) = offsets_from.get(0) {
            push_offset(insts, first);
            for &offset in offsets_from.iter().skip(1) {
                insts.push(Inst::S);
                push_offset(insts, offset);
            }
        }
    } else {
        push_offset(insts, n - acc);
    }
    insts.push(Inst::O);
}

fn push_offset(insts: &mut Vec<Inst>, offset: i32) {
    let (direction, count) = if offset >= 0 {
        (Inst::I, offset as u32)
    } else {
        (Inst::D, -offset as u32)
    };
    push_repeat(insts, direction, count);
}

fn push_repeat(insts: &mut Vec<Inst>, inst: Inst, count: u32) {
    insts.extend((0..count).map(|_| inst));
}

#[must_use]
fn encode_from_zero(n: i32) -> (VecDeque<i32>, usize) {
    let mut n = n as u32;
    let mut offsets = VecDeque::new();
    let mut len = 0;
    while n >= 4 {
        let (sqrt, offset) = nearest_sqrt(n);
        offsets.push_front(offset);
        len += offset.unsigned_abs() as usize + 1;
        n = sqrt;
    }
    if n != 0 {
        offsets.push_front(n as i32);
        len += n as usize;
    }
    (offsets, len)
}

#[inline]
fn nearest_sqrt(n: u32) -> (u32, i32) {
    let sqrt = (n as f64).sqrt();
    let floor = sqrt.floor() as u32;
    let ceil = sqrt.ceil() as u32;
    let floor_diff = n - floor * floor;
    let ceil_diff = ceil * ceil - n;
    // TODO: Avoid crossing over 256 with offset or squaring to it
    // Avoid squaring to 1 << 32
    if ceil == 65536 || floor_diff < ceil_diff {
        (floor, floor_diff as i32)
    } else {
        (ceil, -(ceil_diff as i32))
    }
}

/// Finds the shortest path from `acc` to 0, preferring fewer squares as a tie
/// breaker.
#[inline]
const fn encode_to_zero(acc: i32) -> (i32, u32) {
    let (offset1, squares1) = encode_to_zero_no_overflow(acc);
    let (offset2, squares2) = encode_to_zero_overflow(acc);
    let len1 = offset1.unsigned_abs() + squares1;
    let len2 = offset2.unsigned_abs() + squares2;
    if len1 < len2 || len1 == len2 && squares1 <= squares2 {
        (offset1, squares1)
    } else {
        (offset2, squares2)
    }
}

#[inline]
const fn encode_to_zero_no_overflow(acc: i32) -> (i32, u32) {
    const LOW_16: u32 = (4 + 16) / 2;
    const LOW_256: u32 = (16 + 256) / 2;
    const LOW_NEG: u32 = u32::MAX / 2 + 256 / 2;
    let (target, squares): (i32, _) = match acc as u32 {
        // Offset to 0
        0..4 => (0, 0),
        // Offset and square to 256
        4..LOW_16 => (4, 2),
        LOW_16..LOW_256 => (16, 1),
        LOW_256..LOW_NEG => (256, 0),
        // Offset to -1
        LOW_NEG.. => (-1, 0),
        // Cases for squaring to `x << 32` are not necessary here, because each
        // of those roots have at least 16 trailing zeros and are covered by
        // `encode_to_zero_overflow`.
    };
    (target.wrapping_sub(acc), squares)
}

#[inline]
const fn encode_to_zero_overflow(acc: i32) -> (i32, u32) {
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
