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

use crate::{Builder, Offset, Value};

pub(crate) fn heuristic_encode(b: &mut Builder, v: Value) {
    let acc = b.acc();

    let simple_offset = acc.offset_to(v);

    let (offset_to_0, squares_to_0) = encode_to_0(acc);
    let (offsets_from_0, len_from_0) = encode_from_0(v);
    let len_via_0 = offset_to_0.abs() + squares_to_0 + len_from_0;

    let start = b.insts().len();
    if simple_offset.is_some_and(|offset| offset.abs() <= len_via_0) {
        b.offset(simple_offset.unwrap());
    } else {
        b.offset(offset_to_0);
        b.square(squares_to_0);
        b.offset_squares(&offsets_from_0);
    }
    debug_assert_eq!(b.acc(), v, "acc={acc} {:?}", &b.insts()[start..]);
}

fn encode_from_0(v: Value) -> (VecDeque<Offset>, u32) {
    let mut v = v;
    let mut offsets = VecDeque::new();
    let mut len = 0;
    while v >= 4 {
        let (sqrt, offset) = v.nearest_sqrt();
        offsets.push_front(offset);
        len += offset.abs() + 1;
        v = sqrt;
    }
    offsets.push_front(Offset(v.value() as i64));
    len += v.value();
    (offsets, len)
}

/// Finds the shortest path from `v` to 0, preferring fewer squares as a
/// tiebreaker.
#[inline]
fn encode_to_0(v: Value) -> (Offset, u32) {
    let (offset1, squares1) = encode_to_0_no_overflow(v);
    let (offset2, squares2) = encode_to_0_overflow(v);
    let len1 = offset1.abs() + squares1;
    let len2 = offset2.abs() + squares2;
    if len1 < len2 || len1 == len2 && squares1 <= squares2 {
        (offset1, squares1)
    } else {
        (offset2, squares2)
    }
}

#[inline]
const fn encode_to_0_no_overflow(v: Value) -> (Offset, u32) {
    // Cases for squaring to `x << 32` are not necessary here, because each of
    // those roots have at least 16 trailing zeros and are covered by
    // `encode_to_0_overflow`.
    const N16: u32 = -16i32 as u32;
    const N4: u32 = -4i32 as u32;
    const LOW_16: u32 = (4 + 16) / 2;
    const LOW_256: u32 = (16 + 256) / 2;
    const LOW_N16: u32 = 256 / 2 + N16 / 2;
    const LOW_N4: u32 = N16 / 2 + N4 / 2 + 1;
    const LOW_N1: u32 = N4 / 2 + u32::MAX / 2;
    let (target, squares) = match v.value() {
        // Offset to 0
        0..4 => (0, 0),
        // Square to 256
        4..LOW_16 => (4, 2),
        LOW_16..LOW_256 => (16, 1),
        LOW_256..LOW_N16 => (256, 0),
        // Square to 256 via negative roots
        LOW_N16..LOW_N4 => (N16, 1),
        LOW_N4..LOW_N1 => (N4, 2),
        // Offset to -1
        LOW_N1.. => (u32::MAX, 0),
    };
    (Offset(target as i64 - v.value() as i64), squares)
}

#[inline]
fn encode_to_0_overflow(v: Value) -> (Offset, u32) {
    let mut n = v.value();
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
        n = (v + offset).value();
        tz = n.trailing_zeros();
    }
    // Check if `v` is a modular root of 256
    let square = n.wrapping_mul(n);
    let square2 = square.wrapping_mul(square);
    let square3 = square2.wrapping_mul(square2);
    let squares = if square == 256 {
        1
    } else if square2 == 256 {
        2
    } else if square3 == 256 {
        3
    } else {
        // Square until there are 32 trailing zeros; equivalent to
        // log2(32) - floor(log2(tz))
        tz.leading_zeros() - 32u32.leading_zeros()
    };
    (offset, squares)
}

#[test]
fn sqrts_of_256() {
    let sqrts_of_256 = [
        16u32, 134217712, 134217744, 268435440, 268435472, 402653168, 402653200, 536870896,
        536870928, 671088624, 671088656, 805306352, 805306384, 939524080, 939524112, 1073741808,
        1073741840, 1207959536, 1207959568, 1342177264, 1342177296, 1476394992, 1476395024,
        1610612720, 1610612752, 1744830448, 1744830480, 1879048176, 1879048208, 2013265904,
        2013265936, 2147483632, 2147483664, 2281701360, 2281701392, 2415919088, 2415919120,
        2550136816, 2550136848, 2684354544, 2684354576, 2818572272, 2818572304, 2952790000,
        2952790032, 3087007728, 3087007760, 3221225456, 3221225488, 3355443184, 3355443216,
        3489660912, 3489660944, 3623878640, 3623878672, 3758096368, 3758096400, 3892314096,
        3892314128, 4026531824, 4026531856, 4160749552, 4160749584, 4294967280,
    ];
    for n in sqrts_of_256 {
        assert_eq!(encode_to_0(Value::from(n)), (Offset(0), 1));
    }
}
