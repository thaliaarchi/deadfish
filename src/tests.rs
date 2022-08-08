// Copyright (C) 2022 Andrew Archibald
//
// deadfish is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version. You should have received a copy of the GNU Lesser General
// Public License along with deadfish. If not, see http://www.gnu.org/licenses/.

use crate::*;

macro_rules! insts[
    ($($str:tt)*) => {
        Inst::parse(concat!($(stringify!($str)),*))
    };
];

#[test]
fn eval() {
    // Example programs from https://esolangs.org/wiki/Deadfish#Example_programs
    assert_eq!(
        (vec![Ir::Prompts(6), Ir::Number(0)], 0),
        Ir::eval(&insts![iissso])
    );
    assert_eq!(
        (vec![Ir::Prompts(9), Ir::Number(288)], 288),
        Ir::eval(&insts![diissisdo])
    );
    assert_eq!(
        (vec![Ir::Prompts(40), Ir::Number(0)], 0),
        Ir::eval(&insts![iissisdddddddddddddddddddddddddddddddddo])
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
        Ir::eval(&insts![
            iisiiiisiiiiiiiioiiiiiiiiiiiiiiiiiiiiiiiiiiiiioiiiiiiiooiiio_
            dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddo_
            dddddddddddddddddddddsddoddddddddoiiioddddddoddddddddo_
        ])
    );
}

#[test]
fn bfs_encode() {
    let mut enc = BfsEncoder::new();
    macro_rules! encode(($acc:literal -> $n:literal [$($insts:tt)*]) => {
        assert_eq!(Some(insts![$($insts)*]), enc.encode($acc, $n));
    });
    encode!(0 -> 0 [o]);
    encode!(0 -> 1 [io]);
    encode!(0 -> 2 [iio]);
    encode!(0 -> 3 [iiio]);
    encode!(0 -> 4 [iiso]);
    encode!(0 -> 5 [iisio]);
    encode!(0 -> 6 [iisiio]);
    encode!(0 -> 7 [iiisddo]);
    encode!(0 -> 8 [iiisdo]);
    encode!(0 -> 9 [iiiso]);
    encode!(0 -> 10 [iiisio]);
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
    let bfs_path = enc.encode(acc, n);

    assert_eq!(Some(heuristic_path), bfs_path);
}
