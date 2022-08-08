// Copyright (C) 2022 Andrew Archibald
//
// deadfish is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version. You should have received a copy of the GNU Lesser General
// Public License along with deadfish. If not, see http://www.gnu.org/licenses/.

use crate::*;

macro_rules! insts[
    (@inst i) => { Inst::I };
    (@inst d) => { Inst::D };
    (@inst s) => { Inst::S };
    (@inst o) => { Inst::O };
    (@inst _) => { Inst::Blank };
    ($($inst:tt)*) => { &[$(insts!(@inst $inst)),+][..] };
];

#[test]
fn eval() {
    // Example programs from https://esolangs.org/wiki/Deadfish#Example_programs
    assert_eq!(
        (vec![Ir::Prompts(6), Ir::Number(0)], 0),
        Ir::eval(insts![i i s s s o])
    );
    assert_eq!(
        (vec![Ir::Prompts(9), Ir::Number(288)], 288),
        Ir::eval(insts![d i i s s i s d o])
    );
    assert_eq!(
        (vec![Ir::Prompts(40), Ir::Number(0)], 0),
        Ir::eval(insts![
            i i s s i s d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d o
        ])
    );
    // "Hello world"
    assert_eq!(
        (
            vec![
                Ir::Prompts(17),
                Ir::Number(72),
                Ir::Prompts(30),
                Ir::Number(101),
                Ir::Prompts(8),
                Ir::Number(108),
                Ir::Prompts(1),
                Ir::Number(108),
                Ir::Prompts(4),
                Ir::Number(111),
                Ir::Blanks(1),
                Ir::Prompts(80),
                Ir::Number(32),
                Ir::Blanks(1),
                Ir::Prompts(25),
                Ir::Number(119),
                Ir::Prompts(9),
                Ir::Number(111),
                Ir::Prompts(4),
                Ir::Number(114),
                Ir::Prompts(7),
                Ir::Number(108),
                Ir::Prompts(9),
                Ir::Number(100),
                Ir::Blanks(1),
            ],
            100
        ),
        Ir::eval(insts![
            i i s i i i i s i i i i i i i i o i i i i i i i i i i i i i i i i i i i i i i i i i i i
            i i o i i i i i i i o o i i i o _ d d d d d d d d d d d d d d d d d d d d d d d d d d d
            d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d
            d d d d d d d d o _ d d d d d d d d d d d d d d d d d d d d d s d d o d d d d d d d d o
            i i i o d d d d d d o d d d d d d d d o _
        ])
    );
}

#[test]
fn bfs_encode() {
    let mut enc = BfsEncoder::new();
    assert_eq!(insts![o], enc.encode(0, 0));
    assert_eq!(insts![i o], enc.encode(0, 1));
    assert_eq!(insts![i i o], enc.encode(0, 2));
    assert_eq!(insts![i i i o], enc.encode(0, 3));
    assert_eq!(insts![i i s o], enc.encode(0, 4));
    assert_eq!(insts![i i s i o], enc.encode(0, 5));
    assert_eq!(insts![i i s i i o], enc.encode(0, 6));
    assert_eq!(insts![i i i s d d o], enc.encode(0, 7));
    assert_eq!(insts![i i i s d o], enc.encode(0, 8));
    assert_eq!(insts![i i i s o], enc.encode(0, 9));
    assert_eq!(insts![i i i s i o], enc.encode(0, 10));
}

#[ignore]
#[test]
fn slow_encode() {
    // "Wo" in, e.g., "Hello, World!"
    let acc = 87;
    let n = 111;

    let mut heuristic_path = Vec::new();
    encode(&mut heuristic_path, acc, n);

    let mut enc = BfsEncoder::new();
    let path = enc.encode(acc, n);

    assert_eq!(heuristic_path, path);
}
