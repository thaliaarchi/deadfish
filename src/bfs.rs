// Copyright (C) 2022 Andrew Archibald
//
// deadfish is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version. You should have received a copy of the GNU Lesser General
// Public License along with deadfish. If not, see http://www.gnu.org/licenses/.

use std::collections::HashSet;

use fxhash::FxBuildHasher;

use crate::Inst;

#[derive(Clone, Debug)]
pub struct BfsEncoder {
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

impl BfsEncoder {
    pub const DEFAULT_QUEUE_CAPACITY: usize = 1 << 16;

    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(Self::DEFAULT_QUEUE_CAPACITY)
    }

    #[must_use]
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        BfsEncoder {
            queue: Vec::new(),
            queue_index: 0,
            queue_capacity: capacity,
            visited: HashSet::default(),
        }
    }

    /// Performs a breadth-first search to encode `n` as Deadfish instructions.
    #[must_use]
    pub fn try_encode(&mut self, acc: i32, n: i32) -> Option<Vec<Inst>> {
        self.queue.push(Node {
            acc,
            inst: None,
            prev: usize::MAX,
        });
        while let Some((i, node)) = self.queue_next() {
            if node.acc == n {
                let path = self.path_from_queue(i);
                self.clear();
                return Some(path);
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
        self.clear();
        None
    }

    #[must_use]
    pub fn encode(&mut self, acc: i32, n: i32) -> Vec<Inst> {
        match self.try_encode(acc, n) {
            Some(path) => path,
            None => panic!(
                "Unable to encode {acc} -> {n} within {} steps",
                self.queue.capacity(),
            ),
        }
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

    fn path_from_queue(&mut self, tail: usize) -> Vec<Inst> {
        let mut path = Vec::new();
        let mut index = tail;
        loop {
            let node = self.queue[index];
            match node.inst {
                Some(inst) => {
                    path.push(inst);
                    index = node.prev;
                }
                None => break,
            }
        }
        path.reverse();
        path.push(Inst::O);
        path
    }

    #[inline]
    fn clear(&mut self) {
        self.queue.clear();
        self.queue_index = 0;
        self.visited.clear();
    }
}

impl Default for BfsEncoder {
    fn default() -> Self {
        Self::new()
    }
}