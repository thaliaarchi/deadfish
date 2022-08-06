// Copyright (C) 2022 Andrew Archibald
//
// deadfish is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version. You should have received a copy of the GNU Lesser General
// Public License along with deadfish. If not, see http://www.gnu.org/licenses/.

use std::collections::VecDeque;

use crate::Inst;

#[must_use]
pub fn sqrt_encode(n: i32) -> Vec<Inst> {
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
