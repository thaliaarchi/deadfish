// Copyright (C) 2022 Andrew Archibald
//
// deadfish is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version. You should have received a copy of the GNU Lesser General
// Public License along with deadfish. If not, see http://www.gnu.org/licenses/.

use std::collections::HashSet;
use std::mem;

use crate::Inst;

#[derive(Clone, Debug, Default)]
pub struct Encoder {
    acc: i32,
    insts: Vec<Inst>,
    queue: Vec<Node>,
    queue_index: usize,
    visited: HashSet<i32>,
}

/// `Node` is a linked list element in a search path. It contains the
/// accumulator value of applying the path and, if it's not the first in the
/// path, the instruction it applies and the index of the previous node. `Node`s
/// immutably share prefixes.
#[derive(Copy, Clone, Debug)]
struct Node {
    /// Value of the node.
    acc: i32,
    /// Instruction to produce `n` from the previous node or `None`, if the
    /// first node in the path.
    inst: Option<Inst>,
    /// Index in `queue` of the previous node. To avoid extra space for
    /// alignment, it's not also within the `Option`, but is treated as such.
    prev: usize,
}

impl Encoder {
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Performs a breadth-first search to encode `n` as Deadfish instructions.
    pub fn append_number(&mut self, n: i32) -> &[Inst] {
        self.queue.push(Node {
            acc: self.acc,
            inst: None,
            prev: usize::MAX,
        });
        while let Some(node) = self.queue_next() {
            let i = self.queue_index - 1;
            if node.acc == n {
                self.visited.clear();
                self.acc = n;
                return self.path_from_queue(i);
            }
            for inst in [Inst::I, Inst::D, Inst::S] {
                let acc = inst.apply(node.acc);
                if !self.visited.contains(&acc) {
                    self.visited.insert(acc);
                    self.queue.push(Node { acc, inst: Some(inst), prev: i });
                }
            }
        }
        panic!("BUG! Encoding not found")
    }

    #[inline]
    pub fn encode_number(&mut self, acc: i32, n: i32) -> Vec<Inst> {
        self.acc = acc;
        self.insts.clear();
        self.append_number(n);
        self.take_insts()
    }

    #[inline]
    pub fn push(&mut self, inst: Inst) -> i32 {
        self.acc = inst.apply(self.acc);
        self.acc
    }

    #[must_use]
    #[inline]
    pub fn insts(&self) -> &[Inst] {
        &self.insts
    }

    #[must_use]
    #[inline]
    pub fn take_insts(&mut self) -> Vec<Inst> {
        mem::take(&mut self.insts)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.acc = 0;
        self.insts.clear();
    }

    #[inline]
    fn queue_next(&mut self) -> Option<Node> {
        if let Some(node) = self.queue.get(self.queue_index) {
            self.queue_index += 1;
            Some(*node)
        } else {
            None
        }
    }

    fn path_from_queue(&mut self, tail: usize) -> &[Inst] {
        let path_start = self.insts.len();
        let mut index = tail;
        loop {
            let node = self.queue[index];
            match node.inst {
                Some(inst) => {
                    self.insts.push(inst);
                    index = node.prev;
                }
                None => break,
            }
        }
        self.queue.clear();
        self.queue_index = 0;
        self.insts[path_start..].reverse();
        self.insts.push(Inst::O);
        &self.insts[path_start..]
    }
}

impl From<Encoder> for Vec<Inst> {
    fn from(enc: Encoder) -> Self {
        enc.insts
    }
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
        let mut enc = Encoder::new();
        assert_eq!(insts![o], enc.encode_number(0, 0));
        assert_eq!(insts![i o], enc.encode_number(0, 1));
        assert_eq!(insts![i i o], enc.encode_number(0, 2));
        assert_eq!(insts![i i i o], enc.encode_number(0, 3));
        assert_eq!(insts![i i s o], enc.encode_number(0, 4));
        assert_eq!(insts![i i s i o], enc.encode_number(0, 5));
        assert_eq!(insts![i i s i i o], enc.encode_number(0, 6));
        assert_eq!(insts![i i i s d d o], enc.encode_number(0, 7));
        assert_eq!(insts![i i i s d o], enc.encode_number(0, 8));
        assert_eq!(insts![i i i s o], enc.encode_number(0, 9));
        assert_eq!(insts![i i i s i o], enc.encode_number(0, 10));
    }
}