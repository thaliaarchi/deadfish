use std::collections::VecDeque;

use crate::{heuristic_encode, Inst, Offset, Value};

#[derive(Clone, Debug)]
pub struct Builder {
    insts: Vec<Inst>,
    acc: Value,
}

impl Builder {
    #[inline]
    pub fn new(acc: Value) -> Self {
        Builder::from_insts(Vec::new(), acc)
    }

    #[inline]
    pub fn from_insts(insts: Vec<Inst>, acc: Value) -> Self {
        Builder { insts, acc }
    }

    #[inline]
    pub const fn acc(&self) -> Value {
        self.acc
    }

    #[inline]
    pub fn insts(&self) -> &[Inst] {
        &self.insts
    }

    #[inline]
    pub fn into_insts(self) -> Vec<Inst> {
        self.insts
    }

    #[inline]
    pub fn reset(&mut self, acc: Value) {
        self.acc = acc;
        self.insts.clear();
    }

    /// Encodes `n` as Deadfish instructions.
    #[inline]
    pub fn push_number(&mut self, n: Value) {
        heuristic_encode(self, n);
        self.insts.push(Inst::O);
        self.acc = n;
    }

    #[inline]
    pub fn push_numbers<I: Iterator<Item = Value>>(&mut self, numbers: I) {
        for n in numbers {
            self.push_number(n);
        }
    }

    #[inline]
    pub fn push_string(&mut self, s: &str) {
        for n in s.chars() {
            // Encode Ā (256) as its decomposition, since it cannot be
            // represented in Deadfish as-is.
            if n == 'Ā' {
                self.push_number(Value::from_raw('A' as u32));
                self.push_number(Value::from_raw('\u{0304}' as u32));
            } else {
                self.push_number(Value::from_raw(n as u32));
            }
        }
    }

    #[inline]
    pub fn push_bytes(&mut self, b: &[u8]) {
        for &n in b {
            self.push_number(Value::from_raw(n as u32));
        }
    }

    #[inline]
    pub fn append(&mut self, insts: &[Inst]) {
        self.insts.extend_from_slice(insts);
        self.acc = Inst::eval(insts, self.acc);
    }

    #[inline]
    pub fn push(&mut self, inst: Inst) {
        self.insts.push(inst);
        self.acc = self.acc.apply(inst);
    }

    #[inline]
    pub fn offset(&mut self, offset: Offset) {
        if offset.is_negative() {
            self.sub(offset.abs());
        } else {
            self.add(offset.abs());
        }
    }

    pub fn add(&mut self, x: u32) {
        self.push_repeat(Inst::I, x);
        self.acc += x;
    }

    pub fn sub(&mut self, x: u32) {
        self.push_repeat(Inst::D, x);
        self.acc -= x;
    }

    pub fn square(&mut self, count: u32) {
        self.push_repeat(Inst::S, count);
        self.acc = self.acc.square_repeat(count);
    }

    #[inline]
    fn push_repeat(&mut self, inst: Inst, count: u32) {
        self.insts.extend((0..count).map(|_| inst));
    }

    pub(crate) fn offset_squares(&mut self, offsets: &VecDeque<Offset>) {
        if let Some(&first) = offsets.get(0) {
            self.offset(first);
            for &offset in offsets.iter().skip(1) {
                self.push(Inst::S);
                self.offset(offset);
            }
        }
    }
}

impl From<Builder> for Vec<Inst> {
    fn from(b: Builder) -> Self {
        b.insts
    }
}

impl Default for Builder {
    fn default() -> Self {
        Builder::new(Value::new())
    }
}

#[test]
fn decompose_256() {
    let composed = "Ātra beigto zivju kodēšana";
    let decomposed = "A\u{0304}tra beigto zivju kodēšana";
    let mut b = Builder::new(Value::new());
    b.push_string(composed);
    assert_eq!(Inst::eval_string(b.insts()).unwrap(), decomposed);
}
