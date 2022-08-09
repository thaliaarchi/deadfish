// Copyright (C) 2022 Andrew Archibald
//
// deadfish is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version. You should have received a copy of the GNU Lesser General
// Public License along with deadfish. If not, see http://www.gnu.org/licenses/.

use std::collections::{HashSet, VecDeque};

use fxhash::FxBuildHasher;

use crate::{heuristic_encode, Builder, Inst};

#[derive(Clone, Debug)]
pub struct BfsEncoder {
    queue: Vec<Node>,
    index: usize,
    visited: HashSet<i32, FxBuildHasher>,
    max_len: u16,
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
    /// Path length.
    len: u16,
}

impl BfsEncoder {
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self::with_bound(usize::MAX)
    }

    #[must_use]
    #[inline]
    pub fn with_bound(max_len: usize) -> Self {
        BfsEncoder {
            queue: Vec::new(),
            index: 0,
            visited: HashSet::default(),
            max_len: max_len.try_into().unwrap_or(u16::MAX),
        }
    }

    #[inline]
    pub fn set_bound(&mut self, max_len: usize) {
        self.max_len = max_len.try_into().unwrap_or(u16::MAX);
    }

    /// Performs a breadth-first search to encode `n` as Deadfish instructions.
    /// Returns a path, if one could be constructed, and whether it's optimal.
    #[must_use]
    pub fn encode(&mut self, acc: i32, n: i32) -> (Option<Vec<Inst>>, bool) {
        let mut first_zero = None;
        self.queue.push(Node {
            acc,
            inst: None,
            prev: usize::MAX,
            len: 0,
        });
        while let Some((i, node)) = self.queue_next() {
            if node.acc == n {
                let mut path = self.path_from_queue(i);
                path.push(Inst::O);
                self.clear();
                return (Some(path), true);
            }
            if node.acc == 0 && first_zero == None {
                first_zero = Some(i);
            }
            if node.len < self.max_len {
                for inst in [Inst::I, Inst::D, Inst::S] {
                    let acc = inst.apply(node.acc);
                    if self.visited.insert(acc) {
                        self.queue.push(Node {
                            acc,
                            inst: Some(inst),
                            prev: i,
                            len: node.len + 1,
                        });
                    }
                }
            }
        }
        let path = first_zero.map(|i| {
            let mut b = Builder::from_insts(self.path_from_queue(i), 0);
            heuristic_encode(&mut b, n);
            b.push(Inst::O);
            b.into()
        });
        self.clear();
        (path, false)
    }

    #[inline]
    fn queue_next(&mut self) -> Option<(usize, Node)> {
        let i = self.index;
        if let Some(node) = self.queue.get(i) {
            self.index += 1;
            Some((i, *node))
        } else {
            None
        }
    }

    fn path_from_queue(&mut self, tail: usize) -> Vec<Inst> {
        let mut path = VecDeque::new();
        let mut index = tail;
        loop {
            let node = self.queue[index];
            match node.inst {
                Some(inst) => {
                    path.push_front(inst);
                    index = node.prev;
                }
                None => break,
            }
        }
        path.into()
    }

    #[inline]
    fn clear(&mut self) {
        self.queue.clear();
        self.index = 0;
        self.visited.clear();
    }
}

impl Default for BfsEncoder {
    fn default() -> Self {
        Self::new()
    }
}
