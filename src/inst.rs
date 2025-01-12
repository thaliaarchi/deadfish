use std::io::{self, Write};

use crate::{Builder, Value};

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
    pub fn eval(insts: &[Inst], acc: Value) -> Value {
        insts.iter().fold(acc, |acc, &inst| acc.apply(inst))
    }

    #[must_use]
    #[inline]
    pub fn encode_number(from: Value, to: Value) -> Vec<Inst> {
        let mut b = Builder::new(from);
        b.push_number(to);
        b.into()
    }

    #[must_use]
    #[inline]
    pub fn encode_numbers(ir: &Vec<Value>) -> Vec<Inst> {
        let mut b = Builder::new(Value::new());
        b.push_numbers(ir.iter().copied());
        b.into()
    }

    #[must_use]
    #[inline]
    pub fn minimize(insts: &[Inst]) -> Vec<Inst> {
        let (numbers, _) = Inst::eval_numbers(insts);
        Self::encode_numbers(&numbers)
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
    pub fn eval_numbers(insts: &[Inst]) -> (Vec<Value>, Value) {
        let mut numbers = Vec::new();
        let mut acc = Value::new();
        for &inst in insts {
            match inst {
                Inst::O => numbers.push(acc),
                _ => acc = acc.apply(inst),
            }
        }
        (numbers, acc)
    }

    #[must_use]
    pub fn eval_string(insts: &[Inst]) -> Option<String> {
        let mut s = String::new();
        let mut acc = Value::new();
        for &inst in insts {
            match inst {
                Inst::O => s.push(char::from_u32(acc.value())?),
                _ => acc = acc.apply(inst),
            }
        }
        Some(s)
    }

    pub fn interpret<W: Write>(insts: &[Inst], stdout: &mut W) -> io::Result<()> {
        let mut acc = Value::new();
        for &inst in insts {
            write!(stdout, ">> ")?;
            match inst {
                Inst::I | Inst::D | Inst::S => acc = acc.apply(inst),
                Inst::O => writeln!(stdout, "{acc}")?,
                Inst::Blank => writeln!(stdout)?,
            }
        }
        stdout.flush()
    }
}

/// Deadfish intermediate representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ir {
    /// Output a number.
    Number(Value),
    /// Print `">> "` shell prompts.
    Prompts(u32),
    /// Print line feeds.
    Blanks(u32),
}

impl Ir {
    #[must_use]
    pub fn eval(insts: &[Inst]) -> (Vec<Self>, Value) {
        let mut ir = Vec::new();
        let mut acc = Value::new();
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

    pub fn interpret<W: Write>(ir: &[Ir], stdout: &mut W) -> io::Result<()> {
        for &inst in ir {
            match inst {
                Ir::Number(n) => writeln!(stdout, "{n}")?,
                Ir::Prompts(count) => {
                    for _ in 0..count {
                        write!(stdout, ">> ")?;
                    }
                }
                Ir::Blanks(count) => {
                    for _ in 0..count {
                        writeln!(stdout)?;
                    }
                }
            }
        }
        stdout.flush()
    }
}
