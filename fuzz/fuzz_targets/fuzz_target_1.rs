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
                bfs_path.len(),
                heuristic_path.len(),
                "{from} -> {to} (signed)
{} -> {} (unsigned)
{:032b} -> {:032b} (binary)
BFS       (len={}) {bfs_path:?}
Heuristic (len={}) {heuristic_path:?}",
                from.value(),
                to.value(),
                from.value(),
                to.value(),
                bfs_path.len(),
                heuristic_path.len(),
            );
        }
    }
});
