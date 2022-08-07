// Copyright (C) 2022 Andrew Archibald
//
// deadfish is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version. You should have received a copy of the GNU Lesser General
// Public License along with deadfish. If not, see http://www.gnu.org/licenses/.

use crate::{BfsEncoder, Inst, Ir};

#[derive(Clone, Debug)]
pub struct Builder {
    insts: Vec<Inst>,
    acc: i32,
    encoder: BfsEncoder,
}

impl Builder {
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self::with_acc(0)
    }

    #[must_use]
    #[inline]
    pub fn with_acc(acc: i32) -> Self {
        Builder {
            acc,
            insts: Vec::new(),
            encoder: BfsEncoder::new(),
        }
    }

    #[must_use]
    #[inline]
    pub const fn acc(&self) -> i32 {
        self.acc
    }

    #[inline]
    pub fn reset(&mut self, acc: i32) {
        self.acc = acc;
        self.insts.clear();
    }

    /// Encodes `n` as Deadfish instructions.
    #[inline]
    pub fn append_number(&mut self, n: i32) {
        let path = self.encoder.encode(self.acc, n);
        let n1 = self.append_insts(&path);
        debug_assert_eq!(n, n1);
    }

    #[inline]
    pub fn append_ir(&mut self, ir: &[Ir]) {
        for &inst in ir {
            if let Ir::Number(n) = inst {
                self.append_number(n);
            }
        }
    }

    #[inline]
    pub fn append_numbers<T: Into<i32>, I: Iterator<Item = T>>(&mut self, numbers: I) {
        for n in numbers {
            self.append_number(n.into());
        }
    }

    #[inline]
    pub fn append_str(&mut self, s: &str) {
        for n in s.chars() {
            self.append_number(n as i32);
        }
    }

    #[inline]
    pub fn append_insts(&mut self, insts: &[Inst]) -> i32 {
        self.insts.reserve(insts.len());
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
    pub fn insts(&self) -> (&[Inst], i32) {
        (&self.insts, self.acc)
    }

    #[must_use]
    #[inline]
    pub fn done(self) -> (Vec<Inst>, i32) {
        (self.insts, self.acc)
    }
}

impl From<Builder> for Vec<Inst> {
    fn from(b: Builder) -> Self {
        b.insts
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}