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

use crate::{Acc, Builder, Offset};

pub(crate) fn heuristic_encode(b: &mut Builder, n: Acc) {
    let acc = b.acc();

    let simple_offset = acc.offset_to(n);

    let (offset_to_0, squares_to_0) = encode_to_0(acc);
    let (offsets_from_0, len_from_0) = encode_from_0(n);
    let len_via_0 = offset_to_0.len() + squares_to_0 as usize + len_from_0;

    let start = b.insts().len();
    if simple_offset.is_some_and(|&offset| offset.len() <= len_via_0) {
        b.offset(simple_offset.unwrap());
    } else {
        b.offset(offset_to_0);
        b.square(squares_to_0);
        b.offset_squares(&offsets_from_0);
    }
    debug_assert_eq!(n, b.acc(), "acc={acc} {:?}", &b.insts()[start..]);
}

#[must_use]
pub(crate) fn encode_from_0(n: Acc) -> (VecDeque<Offset>, usize) {
    let mut n = n;
    let mut offsets = VecDeque::new();
    let mut len = 0;
    while n >= 4 {
        let (sqrt, offset) = n.nearest_sqrt();
        offsets.push_front(offset);
        len += offset.len() + 1;
        n = sqrt;
    }
    offsets.push_front(Offset(n.value() as i64));
    len += n.value() as usize;
    (offsets, len)
}

/// Finds the shortest path from `acc` to 0, preferring fewer squares as a tie
/// breaker.
#[inline]
const fn encode_to_0(n: Acc) -> (Offset, u32) {
    let (offset1, squares1) = encode_to_zero_no_overflow(n);
    let (offset2, squares2) = encode_to_zero_overflow(n);
    let len1 = offset1.abs() + squares1;
    let len2 = offset2.abs() + squares2;
    if len1 < len2 || len1 == len2 && squares1 <= squares2 {
        (offset1, squares1)
    } else {
        (offset2, squares2)
    }
}

#[inline]
const fn encode_to_zero_no_overflow(n: Acc) -> (Offset, u32) {
    const LOW_16: u32 = (4 + 16) / 2;
    const LOW_256: u32 = (16 + 256) / 2;
    const LOW_NEG: u32 = u32::MAX / 2 + 256 / 2;
    let (target, squares) = match n.value() {
        // Offset to 0
        0..4 => (0, 0),
        // Offset and square to 256
        4..LOW_16 => (4, 2),
        LOW_16..LOW_256 => (16, 1),
        LOW_256..LOW_NEG => (256, 0),
        // Offset to -1
        LOW_NEG.. => (u32::MAX, 0),
        // Cases for squaring to `x << 32` are not necessary here, because each
        // of those roots have at least 16 trailing zeros and are covered by
        // `encode_to_zero_overflow`.
    };
    (Offset(target as i64 - n.value() as i64), squares)
}

#[inline]
const fn encode_to_zero_overflow(n: Acc) -> (Offset, u32) {
    let mut n = n.value();
    let mut tz = n.trailing_zeros();
    let mut offset = Offset(0);
    if tz < 2 {
        offset = if tz == 1 {
            match n & 0b1111 {
                // Offset to have 4+ trailing zeros
                0b1110 => Offset(2),
                0b0010 => Offset(-2),
                // Use the 1 trailing zero
                _ => Offset(0),
            }
        } else {
            match n & 0b1111_1111 {
                // Offset to have 8+ trailing zeros
                0b1111_1101 => Offset(3),
                0b0000_0011 => Offset(-3),
                // Offset to have 2+ trailing zeros (0b11 => 1, 0b01 => -1)
                _ => Offset((n & 0b11) as i64 - 2),
            }
        };
        n = (Acc::from_raw(n) + offset).value();
        tz = n.trailing_zeros();
    }
    let squares = if n.wrapping_mul(n) == 256 {
        // Square once, if n is a modular square root of 256
        1
    } else {
        // Square until there are 32 trailing zeros; equivalent to
        // log2(32) - floor(log2(tz))
        tz.leading_zeros() - 32u32.leading_zeros()
    };
    (offset, squares)
}
