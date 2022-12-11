// Copyright (C) 2022 Thalia Archibald
//
// deadfish is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version. You should have received a copy of the GNU Lesser General
// Public License along with deadfish. If not, see http://www.gnu.org/licenses/.

use std::collections::{HashSet, VecDeque};

use fxhash::FxBuildHasher;

use crate::{heuristic_encode, Builder, Inst, Value};

#[derive(Clone, Debug)]
pub struct BfsEncoder {
    queue: Vec<Node>,
    index: usize,
    visited: HashSet<Value, FxBuildHasher>,
    max_len: u16,
}

/// `Node` is a linked list element in a search path. It contains the
/// accumulator value of applying the path and, if it's not the first in the
/// path, the instruction it applies and the index of the previous node. `Node`s
/// immutably share prefixes.
#[derive(Copy, Clone, Debug)]
struct Node {
    /// Value of the node.
    v: Value,
    /// Instruction to produce `v` from the previous node or `None`, if the
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

    /// Performs a breadth-first search to encode `to` as Deadfish instructions,
    /// starting at `from`. Returns a path, if one could be constructed, and
    /// whether it's optimal.
    #[must_use]
    pub fn encode(&mut self, from: Value, to: Value) -> (Option<Vec<Inst>>, bool) {
        self.queue.clear();
        self.index = 0;
        self.visited.clear();

        let mut zero_index = None;
        let mut closest_square = None;

        self.queue.push(Node {
            v: from,
            inst: None,
            prev: usize::MAX,
            len: 0,
        });
        while let Some((i, node)) = self.queue_next() {
            if node.v == to {
                return (Some(self.path_from_queue(i)), true);
            }

            // Track the shortest path to 0, because a path from 0 to `to` is
            // usually short
            if node.v == 0 && zero_index == None {
                zero_index = Some(i);
            }

            if node.len < self.max_len {
                for inst in [Inst::I, Inst::D, Inst::S] {
                    let v = node.v.apply(inst);
                    if self.visited.insert(v) {
                        let path_len = node.len + 1;
                        self.queue.push(Node {
                            v,
                            inst: Some(inst),
                            prev: i,
                            len: path_len,
                        });
                        let i = self.queue.len();

                        // Track the square that is closest to `to` by an offset
                        if inst == Inst::S {
                            if let Some(offset) = v.offset_to(to) {
                                let path_len = path_len as usize + offset.len();
                                if !matches!(closest_square, Some((_, _, len)) if len <= path_len) {
                                    closest_square = Some((i, offset, path_len));
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut path = None;
        if let Some(i) = zero_index {
            let mut b = Builder::from_insts(self.path_from_queue(i), Value::new());
            heuristic_encode(&mut b, to);
            path = Some(b.into_insts());
        }
        if let Some((i, offset, _)) = closest_square {
            let mut b = Builder::from_insts(self.path_from_queue(i), self.queue[i].v);
            b.offset(offset);
            let square_path = b.into_insts();
            if !matches!(&path, Some(path) if path.len() <= square_path.len()) {
                path = Some(square_path);
            }
        }
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
}

impl Default for BfsEncoder {
    fn default() -> Self {
        Self::new()
    }
}
