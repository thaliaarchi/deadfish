// Copyright (C) 2022 Andrew Archibald
//
// deadfish is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version. You should have received a copy of the GNU Lesser General
// Public License along with deadfish. If not, see http://www.gnu.org/licenses/.

use std::collections::HashSet;
use std::mem;

use fxhash::FxBuildHasher;

use crate::{Inst, Ir};

#[derive(Clone, Debug)]
pub struct Encoder {
    acc: i32,
    insts: Vec<Inst>,
    queue: Vec<Node>,
    queue_index: usize,
    queue_capacity: usize,
    visited: HashSet<i32, FxBuildHasher>,
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
    pub const DEFAULT_QUEUE_CAPACITY: usize = 1 << 16;

    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Encoder {
            acc: 0,
            insts: Vec::new(),
            queue: Vec::new(),
            queue_index: 0,
            queue_capacity: Self::DEFAULT_QUEUE_CAPACITY,
            visited: HashSet::default(),
        }
    }

    #[must_use]
    #[inline]
    pub fn with_acc(acc: i32) -> Self {
        let mut enc = Self::new();
        enc.acc = acc;
        enc
    }

    /// Performs a breadth-first search to encode `n` as Deadfish instructions.
    pub fn try_append_number(&mut self, n: i32) -> (Option<&[Inst]>, usize) {
        self.queue.push(Node {
            acc: self.acc,
            inst: None,
            prev: usize::MAX,
        });
        while let Some((i, node)) = self.queue_next() {
            if node.acc == n {
                self.acc = n;
                return (Some(self.path_from_queue(i)), i);
            }
            for inst in [Inst::I, Inst::D, Inst::S] {
                let acc = inst.apply(node.acc);
                if self.visited.insert(acc)
                    && (self.queue.capacity() < self.queue_capacity
                        || self.queue.len() < self.queue.capacity())
                {
                    self.queue.push(Node { acc, inst: Some(inst), prev: i });
                }
            }
        }
        let steps = self.queue.len();
        self.clear();
        (None, steps)
    }

    #[must_use]
    #[inline]
    pub fn try_encode_number(&mut self, n: i32) -> Option<Vec<Inst>> {
        self.acc = 0;
        self.insts.clear();
        if self.try_append_number(n).0.is_some() {
            Some(self.take_insts())
        } else {
            None
        }
    }

    /// Encodes `n` as Deadfish instructions.
    pub fn append_number(&mut self, n: i32) -> &[Inst] {
        let acc = self.acc;
        match self.try_append_number(n) {
            (Some(insts), _) => insts,
            (None, steps) => panic!("Unable to encode {acc} -> {n} within {steps} steps"),
        }
    }

    #[must_use]
    #[inline]
    pub fn encode_number(&mut self, n: i32) -> Vec<Inst> {
        self.encode(|enc| {
            enc.append_number(n);
        })
    }

    pub fn append_ir(&mut self, ir: &[Ir]) -> &[Inst] {
        let start = self.insts.len();
        for &inst in ir {
            if let Ir::Number(n) = inst {
                self.append_number(n);
            }
        }
        &self.insts[start..]
    }

    #[must_use]
    #[inline]
    pub fn encode_ir(&mut self, ir: &[Ir]) -> Vec<Inst> {
        self.encode(|enc| {
            enc.append_ir(ir);
        })
    }

    #[inline]
    pub fn append_numbers<T: Into<i32>, I: Iterator<Item = T>>(&mut self, numbers: I) -> &[Inst] {
        let start = self.insts.len();
        for n in numbers {
            self.append_number(n.into());
        }
        &self.insts[start..]
    }

    #[must_use]
    #[inline]
    pub fn encode_numbers<T: Into<i32>, I: Iterator<Item = T>>(&mut self, numbers: I) -> Vec<Inst> {
        self.encode(|enc| {
            enc.append_numbers(numbers);
        })
    }

    #[inline]
    pub fn append_str(&mut self, s: &str) -> &[Inst] {
        let start = self.insts.len();
        for n in s.chars() {
            self.append_number(n as i32);
        }
        &self.insts[start..]
    }

    #[must_use]
    #[inline]
    pub fn encode_str(&mut self, s: &str) -> Vec<Inst> {
        self.encode(|enc| {
            enc.append_str(s);
        })
    }

    #[inline]
    pub fn append_insts(&mut self, insts: &[Inst]) -> i32 {
        for &inst in insts {
            self.push(inst);
        }
        self.acc
    }

    #[inline]
    pub fn push(&mut self, inst: Inst) -> i32 {
        self.insts.push(inst);
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
    pub fn reset(&mut self, acc: i32) {
        self.acc = acc;
        self.insts.clear();
    }

    #[inline]
    pub fn set_queue_capacity(&mut self, capacity: usize) {
        self.queue_capacity = capacity;
    }

    #[inline]
    fn queue_next(&mut self) -> Option<(usize, Node)> {
        let i = self.queue_index;
        if let Some(node) = self.queue.get(i) {
            self.queue_index += 1;
            Some((i, *node))
        } else {
            None
        }
    }

    fn path_from_queue(&mut self, tail: usize) -> &[Inst] {
        let start = self.insts.len();
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
        self.clear();
        self.insts[start..].reverse();
        self.insts.push(Inst::O);
        &self.insts[start..]
    }

    #[inline]
    fn clear(&mut self) {
        self.queue.clear();
        self.queue_index = 0;
        self.visited.clear();
    }

    #[inline]
    fn encode<F: FnOnce(&mut Self) -> R, R>(&mut self, f: F) -> Vec<Inst> {
        self.acc = 0;
        self.insts.clear();
        f(self);
        self.take_insts()
    }
}

impl From<Encoder> for Vec<Inst> {
    fn from(enc: Encoder) -> Self {
        enc.insts
    }
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}
