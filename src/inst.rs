// Copyright (C) 2022 Andrew Archibald
//
// deadfish is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version. You should have received a copy of the GNU Lesser General
// Public License along with deadfish. If not, see http://www.gnu.org/licenses/.

use crate::Builder;

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
    /// Compute the operation on the accumulator.
    #[must_use]
    #[inline]
    pub const fn apply(&self, acc: i32) -> i32 {
        let acc = match self {
            Inst::I => acc.wrapping_add(1),
            Inst::D => acc.wrapping_sub(1),
            Inst::S => acc.wrapping_mul(acc),
            _ => acc,
        };
        if acc == -1 || acc == 256 {
            0
        } else {
            acc
        }
    }

    /// Compute the inverse operation on the accumulator, if possible.
    #[must_use]
    #[inline]
    pub fn apply_inverse(&self, acc: i32) -> Option<i32> {
        let acc = match self {
            Inst::I => acc.wrapping_sub(1),
            Inst::D => acc.wrapping_add(1),
            Inst::S => {
                let sqrt = (acc as u32 as f64).sqrt() as i32;
                if sqrt.wrapping_mul(sqrt) != acc {
                    return None;
                }
                sqrt
            }
            _ => acc,
        };
        if acc == -1 || acc == 256 {
            None
        } else {
            Some(acc)
        }
    }

    #[must_use]
    #[inline]
    pub fn eval(insts: &[Inst]) -> i32 {
        insts.iter().fold(0, |acc, inst| inst.apply(acc))
    }

    #[must_use]
    #[inline]
    pub fn encode(ir: &[Ir]) -> Vec<Inst> {
        let mut b = Builder::new();
        b.append_ir(ir);
        b.into()
    }

    #[must_use]
    #[inline]
    pub fn minimize(insts: &[Inst]) -> Vec<Inst> {
        let (ir, _) = Ir::eval(insts);
        Self::encode(&ir)
    }
}

/// Deadfish intermediate representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ir {
    /// Output a number.
    Number(i32),
    /// Print `">> "` shell prompts.
    Prompts(u32),
    /// Print line feeds.
    Blanks(u32),
}

impl Ir {
    #[must_use]
    pub fn eval(insts: &[Inst]) -> (Vec<Self>, i32) {
        let mut ir = Vec::new();
        let mut acc = 0;
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
                    acc = inst.apply(acc);
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
}
