// Copyright (C) 2022 Andrew Archibald
//
// deadfish is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version. You should have received a copy of the GNU Lesser General
// Public License along with deadfish. If not, see http://www.gnu.org/licenses/.

use crate::{Acc, Builder};

/// Deadfish instructions.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Inst {
    /// `i` — Increment
    I,
    /// `d` — Decrement
    D,
    /// `s` — Square
    S,
    /// `o` — Output
    O,
    /// other — Print a line feed
    Blank,
}

impl Inst {
    #[must_use]
    #[inline]
    pub fn eval(insts: &[Inst], acc: Acc) -> Acc {
        insts.iter().fold(acc, |acc, &inst| acc.apply(inst))
    }

    #[must_use]
    #[inline]
    pub fn encode_number(acc: Acc, n: Acc) -> Vec<Inst> {
        let mut b = Builder::new(acc);
        b.push_number(n);
        b.into()
    }

    #[must_use]
    #[inline]
    pub fn encode(ir: &[Ir]) -> Vec<Inst> {
        let mut b = Builder::new(Acc::new());
        b.push_ir(ir);
        b.into()
    }

    #[must_use]
    #[inline]
    pub fn minimize(insts: &[Inst]) -> Vec<Inst> {
        let (ir, _) = Ir::eval(insts);
        Self::encode(&ir)
    }

    #[must_use]
    pub fn parse<B: AsRef<[u8]>>(src: B) -> Vec<Inst> {
        let src = src.as_ref();
        let mut insts = Vec::with_capacity(src.len());
        for c in src {
            insts.push(match c {
                b'i' => Inst::I,
                b'd' => Inst::D,
                b's' => Inst::S,
                b'o' => Inst::O,
                _ => Inst::Blank,
            });
        }
        insts
    }

    #[must_use]
    pub fn eval_string(insts: &[Inst]) -> Option<String> {
        let mut s = String::new();
        let mut acc = Acc::new();
        for &inst in insts {
            match inst {
                Inst::O => s.push(char::from_u32(acc.value())?),
                _ => acc = acc.apply(inst),
            }
        }
        Some(s)
    }
}

/// Deadfish intermediate representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ir {
    /// Output a number.
    Number(Acc),
    /// Print `">> "` shell prompts.
    Prompts(u32),
    /// Print line feeds.
    Blanks(u32),
}

impl Ir {
    #[must_use]
    pub fn eval(insts: &[Inst]) -> (Vec<Self>, Acc) {
        let mut ir = Vec::new();
        let mut acc = Acc::new();
        // Counting prompts or blanks
        let mut counting_prompts = true;
        let mut count = 0;

        for &inst in insts {
            match inst {
                Inst::I | Inst::D | Inst::S => {
                    // Flush any blanks and switch to counting prompts
                    if !counting_prompts && count != 0 {
                        ir.push(Ir::Blanks(count));
                        count = 0;
                    }
                    counting_prompts = true;
                    count += 1;

                    // Apply `i`, `d`, or `s` to the accumulator
                    acc = acc.apply(inst);
                }
                Inst::O => {
                    // Flush any prompts and blanks (including a prompt for `o`)
                    if !counting_prompts && count != 0 {
                        ir.push(Ir::Blanks(count));
                        ir.push(Ir::Prompts(1));
                    } else {
                        ir.push(Ir::Prompts(count + 1));
                    }
                    count = 0;

                    // Push `o` with the evaluated current accumulator
                    ir.push(Ir::Number(acc));
                }
                Inst::Blank => {
                    // Flush any prompts and switch to counting blanks
                    if counting_prompts && count != 0 {
                        ir.push(Ir::Prompts(count));
                        count = 0;
                    }
                    counting_prompts = false;
                    count += 1;
                }
            }
        }

        // Flush remaining prompts and blanks
        if count != 0 {
            if counting_prompts {
                ir.push(Ir::Prompts(count));
            } else {
                ir.push(Ir::Blanks(count));
            }
        }

        (ir, acc)
    }

    #[must_use]
    pub fn eval_string(ir: &[Ir]) -> Option<String> {
        let mut s = String::new();
        for &inst in ir {
            if let Ir::Number(n) = inst {
                s.push(char::from_u32(n.value())?);
            }
        }
        Some(s)
    }
}
