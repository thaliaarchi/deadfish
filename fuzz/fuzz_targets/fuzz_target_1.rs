#![no_main]

use deadfish::{BfsEncoder, Builder, Inst, Value};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: [u32; 2]| {
    let from = Value::from_checked(data[0]);
    let to = Value::from_checked(data[1]);
    if let (Some(from), Some(to)) = (from, to) {
        let bfs = BfsEncoder::with_bound(16).encode(from, to);
        if let (Some(mut bfs_path), true) = bfs {
            bfs_path.push(Inst::O);

            let mut b = Builder::new(from);
            b.push_number(to);
            let heuristic_path = b.into_insts();

            assert_eq!(
                heuristic_path.len(),
                bfs_path.len(),
                "{from} -> {to} (signed)
{from_unsigned} -> {to_unsigned} (unsigned)
{from_unsigned:032b} -> {to_unsigned:032b} (binary)
Heuristic (len={heuristic_len}) {heuristic_path:?}
BFS       (len={bfs_len}) {bfs_path:?}",
                from_unsigned = from.value(),
                to_unsigned = to.value(),
                heuristic_len = heuristic_path.len(),
                bfs_len = bfs_path.len(),
            );
        }
    }
});
