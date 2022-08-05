// Copyright (C) 2022 Andrew Archibald
//
// deadfish is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version. You should have received a copy of the GNU Lesser General
// Public License along with deadfish. If not, see http://www.gnu.org/licenses/.

use std::collections::{HashSet, VecDeque};

use crate::Inst;

/// Performs a breadth-first search to encode `n` as Deadfish instructions,
/// where the accumulator starts at `acc`.
#[must_use]
pub fn encode_number(acc: i32, n: i32) -> Vec<Inst> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((acc, Vec::new()));
    while let Some((v, mut path)) = queue.pop_front() {
        if v == n {
            path.push(Inst::O);
            return path.into_iter().collect();
        }
        for inst in [Inst::I, Inst::D, Inst::S] {
            let v1 = inst.apply(v);
            if !visited.contains(&v1) {
                visited.insert(v1);
                let mut path1 = path.clone();
                path1.push(inst);
                queue.push_back((v1, path1));
            }
        }
    }
    panic!("encoding not found")
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! insts[
        (@inst i) => { Inst::I };
        (@inst d) => { Inst::D };
        (@inst s) => { Inst::S };
        (@inst o) => { Inst::O };
        (@inst _) => { Inst::Blank };
        ($($inst:tt)*) => { &[$(insts!(@inst $inst)),+][..] };
    ];

    #[test]
    fn encode() {
        assert_eq!(insts![o], encode_number(0, 0));
        assert_eq!(insts![i o], encode_number(0, 1));
        assert_eq!(insts![i i o], encode_number(0, 2));
        assert_eq!(insts![i i i o], encode_number(0, 3));
        assert_eq!(insts![i i s o], encode_number(0, 4));
        assert_eq!(insts![i i s i o], encode_number(0, 5));
        assert_eq!(insts![i i s i i o], encode_number(0, 6));
        assert_eq!(insts![i i i s d d o], encode_number(0, 7));
        assert_eq!(insts![i i i s d o], encode_number(0, 8));
        assert_eq!(insts![i i i s o], encode_number(0, 9));
        assert_eq!(insts![i i i s i o], encode_number(0, 10));
    }
}
