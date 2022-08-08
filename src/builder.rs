// Copyright (C) 2022 Andrew Archibald
//
// deadfish is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version. You should have received a copy of the GNU Lesser General
// Public License along with deadfish. If not, see http://www.gnu.org/licenses/.

use crate::{heuristic_encode, normalize, Inst, Ir};

#[derive(Clone, Debug)]
pub struct Builder {
    insts: Vec<Inst>,
    acc: i32,
}

impl Builder {
    #[must_use]
    #[inline]
    pub fn new(acc: i32) -> Self {
        Builder {
            insts: Vec::new(),
            acc: normalize(acc),
        }
    }

    #[must_use]
    #[inline]
    pub const fn acc(&self) -> i32 {
        self.acc
    }

    #[must_use]
    #[inline]
    pub fn insts(&self) -> &[Inst] {
        &self.insts
    }

    #[inline]
    pub fn reset(&mut self, acc: i32) {
        self.acc = normalize(acc);
        self.insts.clear();
    }

    /// Encodes `n` as Deadfish instructions.
    #[inline]
    pub fn push_number(&mut self, n: i32) {
        heuristic_encode(self, n);
        self.acc = normalize(n);
    }

    #[inline]
    pub fn push_ir(&mut self, ir: &[Ir]) {
        for &inst in ir {
            if let Ir::Number(n) = inst {
                self.push_number(n);
            }
        }
    }

    #[inline]
    pub fn push_numbers<T: Into<i32>, I: Iterator<Item = T>>(&mut self, numbers: I) {
        for n in numbers {
            self.push_number(n.into());
        }
    }

    #[inline]
    pub fn push_str(&mut self, s: &str) {
        for n in s.chars() {
            // TODO: Represent 256
            self.push_number(n as i32);
        }
    }

    #[inline]
    pub fn append(&mut self, insts: &[Inst]) -> i32 {
        self.insts.extend_from_slice(insts);
        self.acc = Inst::eval(insts, self.acc);
        self.acc
    }

    #[inline]
    pub fn push(&mut self, inst: Inst) -> i32 {
        self.insts.push(inst);
        self.acc = inst.apply(self.acc);
        self.acc
    }

    #[inline]
    pub fn offset(&mut self, offset: i32) -> i32 {
        if offset > 0 {
            self.add(offset as u32)
        } else if offset < 0 {
            self.sub(offset.unsigned_abs())
        } else {
            self.acc
        }
    }

    pub fn add(&mut self, x: u32) -> i32 {
        self.push_repeat(Inst::I, x);
        let acc = self.acc as u32;
        self.acc = if acc < 256 && acc.saturating_add(x) >= 256 || acc.saturating_add(x) == u32::MAX
        {
            0
        } else {
            acc.wrapping_add(x) as i32
        };
        self.acc
    }

    pub fn sub(&mut self, x: u32) -> i32 {
        self.push_repeat(Inst::D, x);
        let acc = self.acc as u32;
        self.acc = if acc > 256 && acc.saturating_sub(x) <= 256 || acc.saturating_sub(x) == 0 {
            0
        } else {
            acc.wrapping_sub(x) as i32
        };
        self.acc
    }

    pub fn square(&mut self, count: u32) -> i32 {
        self.push_repeat(Inst::S, count);
        for _ in 0..count {
            self.acc = self.acc.wrapping_mul(self.acc);
            if self.acc == 256 || self.acc == -1 {
                self.acc = 0;
                break;
            }
        }
        self.acc
    }

    #[inline]
    fn push_repeat(&mut self, inst: Inst, count: u32) {
        self.insts.extend((0..count).map(|_| inst));
    }
}

impl From<Builder> for Vec<Inst> {
    fn from(b: Builder) -> Self {
        b.insts
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new(0)
    }
}
