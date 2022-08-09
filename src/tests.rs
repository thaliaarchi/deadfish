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
fn compare_heuristic() {
    compare_encode(box |acc, n| Some(Inst::encode_number(acc, n)))
}

#[test]
fn compare_bfs() {
    let mut enc = BfsEncoder::with_bound(16);
    compare_encode(box move |acc, n| enc.encode(acc, n));
}

fn compare_encode(mut f: Box<dyn FnMut(i32, i32) -> Option<Vec<Inst>>>) {
    fn compare(acc: i32, n: i32, path: Option<Vec<Inst>>, known_paths: &[Vec<Inst>]) {
        if let Some(path) = path {
            for p in known_paths {
                assert_eq!(n, Inst::eval(p, acc), "{:?}", p);
            }
            assert!(
                known_paths.iter().find(|&p| &path == p).is_some(),
                "path {path:?} not in {known_paths:?}",
            );
        } else {
            println!("Unable to encode {acc} -> {n}");
        }
    }
    macro_rules! encode(($acc:literal -> $n:literal [$($insts:tt),+]) => {
        compare($acc, $n, f($acc, $n), &[$(insts![$insts]),+]);
    });

    // The encodings for 0 -> 1..256 are the shortest solutions from Code Golf,
    // that do not output partial values.
    // https://codegolf.stackexchange.com/questions/40124/short-deadfish-numbers
    encode!(0 -> 0 [o]);
    encode!(0 -> 1 [io]);
    encode!(0 -> 2 [iio]);
    encode!(0 -> 3 [iiio]);
    encode!(0 -> 4 [iiso]);
    encode!(0 -> 5 [iisio]);
    encode!(0 -> 6 [iisiio]);
    encode!(0 -> 7 [iiisddo, iisiiio]);
    encode!(0 -> 8 [iiisdo]);
    encode!(0 -> 9 [iiiso]);
    encode!(0 -> 10 [iiisio]);
    encode!(0 -> 11 [iiisiio]);
    encode!(0 -> 12 [iiisiiio]);
    encode!(0 -> 13 [iissdddo]);
    encode!(0 -> 14 [iissddo]);
    encode!(0 -> 15 [iissdo]);
    encode!(0 -> 16 [iisso]);
    encode!(0 -> 17 [iissio]);
    encode!(0 -> 18 [iissiio]);
    encode!(0 -> 19 [iissiiio]);
    encode!(0 -> 20 [iissiiiio]);
    encode!(0 -> 21 [iisisddddo, iissiiiiio]);
    encode!(0 -> 22 [iisisdddo]);
    encode!(0 -> 23 [iisisddo]);
    encode!(0 -> 24 [iisisdo]);
    encode!(0 -> 25 [iisiso]);
    encode!(0 -> 26 [iisisio]);
    encode!(0 -> 27 [iisisiio]);
    encode!(0 -> 28 [iisisiiio]);
    encode!(0 -> 29 [iisisiiiio]);
    encode!(0 -> 30 [iisisiiiiio]);
    encode!(0 -> 31 [iisiisdddddo, iisisiiiiiio]);
    encode!(0 -> 32 [iisiisddddo]);
    encode!(0 -> 33 [iisiisdddo]);
    encode!(0 -> 34 [iisiisddo]);
    encode!(0 -> 35 [iisiisdo]);
    encode!(0 -> 36 [iisiiso]);
    encode!(0 -> 37 [iisiisio]);
    encode!(0 -> 38 [iisiisiio]);
    encode!(0 -> 39 [iisiisiiio]);
    encode!(0 -> 40 [iisiisiiiio]);
    encode!(0 -> 41 [iisiisiiiiio]);
    encode!(0 -> 42 [iisiisiiiiiio]);
    encode!(0 -> 43 [iiisddsddddddo, iisiisiiiiiiio]);
    encode!(0 -> 44 [iiisddsdddddo, iisiiisdddddo]);
    encode!(0 -> 45 [iiisddsddddo, iisiiisddddo]);
    encode!(0 -> 46 [iiisddsdddo, iisiiisdddo]);
    encode!(0 -> 47 [iiisddsddo, iisiiisddo]);
    encode!(0 -> 48 [iiisddsdo, iisiiisdo]);
    encode!(0 -> 49 [iiisddso, iisiiiso]);
    encode!(0 -> 50 [iiisddsio, iisiiisio]);
    encode!(0 -> 51 [iiisddsiio, iisiiisiio]);
    encode!(0 -> 52 [iiisddsiiio, iisiiisiiio]);
    encode!(0 -> 53 [iiisddsiiiio, iisiiisiiiio]);
    encode!(0 -> 54 [iiisddsiiiiio, iisiiisiiiiio]);
    encode!(0 -> 55 [iiisddsiiiiiio, iisiiisiiiiiio]);
    encode!(0 -> 56 [iiisddsiiiiiiio, iisiiisiiiiiiio]);
    encode!(0 -> 57 [iiisdsdddddddo]);
    encode!(0 -> 58 [iiisdsddddddo]);
    encode!(0 -> 59 [iiisdsdddddo]);
    encode!(0 -> 60 [iiisdsddddo]);
    encode!(0 -> 61 [iiisdsdddo]);
    encode!(0 -> 62 [iiisdsddo]);
    encode!(0 -> 63 [iiisdsdo]);
    encode!(0 -> 64 [iiisdso]);
    encode!(0 -> 65 [iiisdsio]);
    encode!(0 -> 66 [iiisdsiio]);
    encode!(0 -> 67 [iiisdsiiio]);
    encode!(0 -> 68 [iiisdsiiiio]);
    encode!(0 -> 69 [iiisdsiiiiio]);
    encode!(0 -> 70 [iiisdsiiiiiio]);
    encode!(0 -> 71 [iiisdsiiiiiiio]);
    encode!(0 -> 72 [iiisdsiiiiiiiio, iiissdddddddddo]);
    encode!(0 -> 73 [iiissddddddddo]);
    encode!(0 -> 74 [iiissdddddddo]);
    encode!(0 -> 75 [iiissddddddo]);
    encode!(0 -> 76 [iiissdddddo]);
    encode!(0 -> 77 [iiissddddo]);
    encode!(0 -> 78 [iiissdddo]);
    encode!(0 -> 79 [iiissddo]);
    encode!(0 -> 80 [iiissdo]);
    encode!(0 -> 81 [iiisso]);
    encode!(0 -> 82 [iiissio]);
    encode!(0 -> 83 [iiissiio]);
    encode!(0 -> 84 [iiissiiio]);
    encode!(0 -> 85 [iiissiiiio]);
    encode!(0 -> 86 [iiissiiiiio]);
    encode!(0 -> 87 [iiissiiiiiio]);
    encode!(0 -> 88 [iiissiiiiiiio]);
    encode!(0 -> 89 [iiissiiiiiiiio]);
    encode!(0 -> 90 [iiissiiiiiiiiio]);
    encode!(0 -> 91 [iiisisdddddddddo, iiissiiiiiiiiiio]);
    encode!(0 -> 92 [iiisisddddddddo]);
    encode!(0 -> 93 [iiisisdddddddo]);
    encode!(0 -> 94 [iiisisddddddo]);
    encode!(0 -> 95 [iiisisdddddo]);
    encode!(0 -> 96 [iiisisddddo]);
    encode!(0 -> 97 [iiisisdddo]);
    encode!(0 -> 98 [iiisisddo]);
    encode!(0 -> 99 [iiisisdo]);
    encode!(0 -> 100 [iiisiso]);
    encode!(0 -> 101 [iiisisio]);
    encode!(0 -> 102 [iiisisiio]);
    encode!(0 -> 103 [iiisisiiio]);
    encode!(0 -> 104 [iiisisiiiio]);
    encode!(0 -> 105 [iiisisiiiiio]);
    encode!(0 -> 106 [iiisisiiiiiio]);
    encode!(0 -> 107 [iiisisiiiiiiio]);
    encode!(0 -> 108 [iiisisiiiiiiiio]);
    encode!(0 -> 109 [iiisisiiiiiiiiio]);
    encode!(0 -> 110 [iiisisiiiiiiiiiio]);
    encode!(0 -> 111 [iiisiisddddddddddo, iiisisiiiiiiiiiiio]);
    encode!(0 -> 112 [iiisiisdddddddddo]);
    encode!(0 -> 113 [iiisiisddddddddo]);
    encode!(0 -> 114 [iiisiisdddddddo]);
    encode!(0 -> 115 [iiisiisddddddo]);
    encode!(0 -> 116 [iiisiisdddddo]);
    encode!(0 -> 117 [iiisiisddddo]);
    encode!(0 -> 118 [iiisiisdddo]);
    encode!(0 -> 119 [iiisiisddo]);
    encode!(0 -> 120 [iiisiisdo]);
    encode!(0 -> 121 [iiisiiso]);
    encode!(0 -> 122 [iiisiisio]);
    encode!(0 -> 123 [iiisiisiio]);
    encode!(0 -> 124 [iiisiisiiio]);
    encode!(0 -> 125 [iiisiisiiiio]);
    encode!(0 -> 126 [iiisiisiiiiio]);
    encode!(0 -> 127 [iiisiisiiiiiio]);
    encode!(0 -> 128 [iiisiisiiiiiiio]);
    encode!(0 -> 129 [iiisiisiiiiiiiio]);
    encode!(0 -> 130 [iiisiisiiiiiiiiio]);
    encode!(0 -> 131 [iiisiisiiiiiiiiiio]);
    encode!(0 -> 132 [iiisiisiiiiiiiiiiio]);
    encode!(0 -> 133 [iiisiiisdddddddddddo, iiisiisiiiiiiiiiiiio]);
    encode!(0 -> 134 [iiisiiisddddddddddo]);
    encode!(0 -> 135 [iiisiiisdddddddddo]);
    encode!(0 -> 136 [iiisiiisddddddddo]);
    encode!(0 -> 137 [iiisiiisdddddddo]);
    encode!(0 -> 138 [iiisiiisddddddo]);
    encode!(0 -> 139 [iiisiiisdddddo]);
    encode!(0 -> 140 [iiisiiisddddo]);
    encode!(0 -> 141 [iiisiiisdddo]);
    encode!(0 -> 142 [iiisiiisddo]);
    encode!(0 -> 143 [iiisiiisdo]);
    encode!(0 -> 144 [iiisiiiso]);
    encode!(0 -> 145 [iiisiiisio]);
    encode!(0 -> 146 [iiisiiisiio]);
    encode!(0 -> 147 [iiisiiisiiio]);
    encode!(0 -> 148 [iiisiiisiiiio]);
    encode!(0 -> 149 [iiisiiisiiiiio]);
    encode!(0 -> 150 [iiisiiisiiiiiio]);
    encode!(0 -> 151 [iiisiiisiiiiiiio]);
    encode!(0 -> 152 [iiisiiisiiiiiiiio]);
    encode!(0 -> 153 [iiisiiisiiiiiiiiio]);
    encode!(0 -> 154 [iiisiiisiiiiiiiiiio]);
    encode!(0 -> 155 [iiisiiisiiiiiiiiiiio]);
    encode!(0 -> 156 [iiisiiisiiiiiiiiiiiio]);
    encode!(0 -> 157 [iissdddsddddddddddddo]);
    encode!(0 -> 158 [iissdddsdddddddddddo]);
    encode!(0 -> 159 [iissdddsddddddddddo]);
    encode!(0 -> 160 [iissdddsdddddddddo]);
    encode!(0 -> 161 [iissdddsddddddddo]);
    encode!(0 -> 162 [iissdddsdddddddo]);
    encode!(0 -> 163 [iissdddsddddddo]);
    encode!(0 -> 164 [iissdddsdddddo]);
    encode!(0 -> 165 [iissdddsddddo]);
    encode!(0 -> 166 [iissdddsdddo]);
    encode!(0 -> 167 [iissdddsddo]);
    encode!(0 -> 168 [iissdddsdo]);
    encode!(0 -> 169 [iissdddso]);
    encode!(0 -> 170 [iissdddsio]);
    encode!(0 -> 171 [iissdddsiio]);
    encode!(0 -> 172 [iissdddsiiio]);
    encode!(0 -> 173 [iissdddsiiiio]);
    encode!(0 -> 174 [iissdddsiiiiio]);
    encode!(0 -> 175 [iissdddsiiiiiio]);
    encode!(0 -> 176 [iissdddsiiiiiiio]);
    encode!(0 -> 177 [iissdddsiiiiiiiio]);
    encode!(0 -> 178 [iissdddsiiiiiiiiio]);
    encode!(0 -> 179 [iissdddsiiiiiiiiiio]);
    encode!(0 -> 180 [iissdddsiiiiiiiiiiio]);
    encode!(0 -> 181 [iissdddsiiiiiiiiiiiio]);
    encode!(0 -> 182 [iissdddsiiiiiiiiiiiiio, iissddsddddddddddddddo]);
    encode!(0 -> 183 [iissddsdddddddddddddo]);
    encode!(0 -> 184 [iissddsddddddddddddo]);
    encode!(0 -> 185 [iissddsdddddddddddo]);
    encode!(0 -> 186 [iissddsddddddddddo]);
    encode!(0 -> 187 [iissddsdddddddddo]);
    encode!(0 -> 188 [iissddsddddddddo]);
    encode!(0 -> 189 [iissddsdddddddo]);
    encode!(0 -> 190 [iissddsddddddo]);
    encode!(0 -> 191 [iissddsdddddo]);
    encode!(0 -> 192 [iissddsddddo]);
    encode!(0 -> 193 [iissddsdddo]);
    encode!(0 -> 194 [iissddsddo]);
    encode!(0 -> 195 [iissddsdo]);
    encode!(0 -> 196 [iissddso]);
    encode!(0 -> 197 [iissddsio]);
    encode!(0 -> 198 [iissddsiio]);
    encode!(0 -> 199 [iissddsiiio]);
    encode!(0 -> 200 [iissddsiiiio]);
    encode!(0 -> 201 [iissddsiiiiio]);
    encode!(0 -> 202 [iissddsiiiiiio]);
    encode!(0 -> 203 [iissddsiiiiiiio]);
    encode!(0 -> 204 [iissddsiiiiiiiio]);
    encode!(0 -> 205 [iissddsiiiiiiiiio]);
    encode!(0 -> 206 [iissddsiiiiiiiiiio]);
    encode!(0 -> 207 [iissddsiiiiiiiiiiio]);
    encode!(0 -> 208 [iissddsiiiiiiiiiiiio]);
    encode!(0 -> 209 [iissddsiiiiiiiiiiiiio]);
    encode!(0 -> 210 [iissddsiiiiiiiiiiiiiio, iissdsdddddddddddddddo]);
    encode!(0 -> 211 [iissdsddddddddddddddo]);
    encode!(0 -> 212 [iissdsdddddddddddddo]);
    encode!(0 -> 213 [iissdsddddddddddddo]);
    encode!(0 -> 214 [iissdsdddddddddddo]);
    encode!(0 -> 215 [iissdsddddddddddo]);
    encode!(0 -> 216 [iissdsdddddddddo]);
    encode!(0 -> 217 [iissdsddddddddo]);
    encode!(0 -> 218 [iissdsdddddddo]);
    encode!(0 -> 219 [iissdsddddddo]);
    encode!(0 -> 220 [iissdsdddddo]);
    encode!(0 -> 221 [iissdsddddo]);
    encode!(0 -> 222 [iissdsdddo]);
    encode!(0 -> 223 [iissdsddo]);
    encode!(0 -> 224 [iissdsdo]);
    encode!(0 -> 225 [iissdso]);
    encode!(0 -> 226 [iissdsio]);
    encode!(0 -> 227 [iissdsiio]);
    encode!(0 -> 228 [iissdsiiio]);
    encode!(0 -> 229 [iissdsiiiio]);
    encode!(0 -> 230 [iissdsiiiiio]);
    encode!(0 -> 231 [iissdsiiiiiio]);
    encode!(0 -> 232 [iissdsiiiiiiio]);
    encode!(0 -> 233 [iissdsiiiiiiiio]);
    encode!(0 -> 234 [iissdsiiiiiiiiio]);
    encode!(0 -> 235 [iissdsiiiiiiiiiio]);
    encode!(0 -> 236 [iissdsiiiiiiiiiiio]);
    encode!(0 -> 237 [iissdsiiiiiiiiiiiio]);
    encode!(0 -> 238 [iissdsiiiiiiiiiiiiio]);
    encode!(0 -> 239 [iissdsiiiiiiiiiiiiiio]);
    encode!(0 -> 240 [iissdsiiiiiiiiiiiiiiio]);
    encode!(0 -> 241 [iissdsiiiiiiiiiiiiiiiio]);
    encode!(0 -> 242 [iissdsiiiiiiiiiiiiiiiiio]);
    encode!(0 -> 243 [iissdsiiiiiiiiiiiiiiiiiio]);
    encode!(0 -> 244 [iissdsiiiiiiiiiiiiiiiiiiio]);
    encode!(0 -> 245 [iissdsiiiiiiiiiiiiiiiiiiiio]);
    encode!(0 -> 246 [iissdsiiiiiiiiiiiiiiiiiiiiio]);
    encode!(0 -> 247 [iissdsiiiiiiiiiiiiiiiiiiiiiio]);
    encode!(0 -> 248 [iissdsiiiiiiiiiiiiiiiiiiiiiiio]);
    encode!(0 -> 249 [iissdsiiiiiiiiiiiiiiiiiiiiiiiio]);
    encode!(0 -> 250 [iissdsiiiiiiiiiiiiiiiiiiiiiiiiio]);
    encode!(0 -> 251 [iissdsiiiiiiiiiiiiiiiiiiiiiiiiiio]);
    encode!(0 -> 252 [iissdsiiiiiiiiiiiiiiiiiiiiiiiiiiio]);
    encode!(0 -> 253 [iissdsiiiiiiiiiiiiiiiiiiiiiiiiiiiio]);
    encode!(0 -> 254 [iissdsiiiiiiiiiiiiiiiiiiiiiiiiiiiiio]);
    encode!(0 -> 255 [iissdsiiiiiiiiiiiiiiiiiiiiiiiiiiiiiio]);

    encode!(0 -> 257 [iissisddddddddddddddddddddddddddddddddo]);

    // "Hello, World!"
    encode!(0 -> 72 [iiisdsiiiiiiiio]);
    encode!(72 -> 101 [ssssiiisisio]);
    encode!(101 -> 108 [iiiiiiio]);
    encode!(108 -> 108 [o]);
    encode!(108 -> 111 [iiio]);
    encode!(111 -> 44 [isssiiisddsdddddo]);
    encode!(44 -> 32 [ddddddddddddo]);
    encode!(32 -> 87 [sssiiissiiiiiio]);
    encode!(87 -> 111 [issssiiisiisddddddddddo]);
    encode!(111 -> 114 [iiio]);
    encode!(114 -> 108 [ddddddo]);
    encode!(108 -> 100 [ddddddddo]);
    encode!(100 -> 33 [ssssiisiisdddo]);
}

#[ignore]
#[test]
fn slow_bfs() {
    // "Wo" in, e.g., "Hello, World!"
    let acc = 87;
    let n = 111;

    let heuristic_path = Inst::encode_number(acc, n);

    let mut enc = BfsEncoder::new();
    let bfs_path = enc.encode(acc, n);

    assert_eq!(Some(heuristic_path), bfs_path);
}
