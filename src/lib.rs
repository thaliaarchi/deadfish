//! Deadfish language.
//!
//! # Semantics
//!
//! Deadfish consists of four commands: increment, decrement, square, and
//! output, which operate on an accumulator. The [reference implementation](https://esolangs.org/w/index.php?title=Deadfish&oldid=6598),
//! which was written in C, defines the commands as `x++`, `x--`, `x = x * x`,
//! and `printf("%d\n", x)`, respectively. The accumulator `x` is signed, with
//! the type `int` (it's defined as `unsigned int`, but printed as signed).
//! After a command, if `x` is equal to `256` or `-1`, then it is set to `0`.
//!
//! The interpreter prints `">> "` before every command and `\n` for any
//! unrecognized command (essentially another command). Command reading is
//! buffered, so input and output are not strictly ordered.
//!
//! Deadfish is not Turing complete; however, it can be interesting to search
//! for minimal programs that will produce a sequence of prints.
//!
//! # Resources
//!
//! - Implementations by the creator:
//!   - [C (reference)](https://esolangs.org/w/index.php?title=Deadfish&oldid=6598)
//!   - [Python](https://esolangs.org/w/index.php?title=Deadfish&oldid=9122#Python)
//!   - [Creator's site](https://web.archive.org/web/20100425075447/http://www.jonathantoddskinner.com/projects/deadfish.html)
//!     with [source archives](https://web.archive.org/web/20071019052558/http://www.jonathantoddskinner.com/projects/deadfish.tar.gz)
//! - [Esolang wiki](https://esolangs.org/wiki/Deadfish)

#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::module_name_repetitions
)]

pub use bfs::*;
pub use builder::*;
pub(crate) use heuristic::*;
pub use inst::*;
pub use sqrt::*;
pub use value::*;

mod bfs;
mod builder;
mod heuristic;
mod inst;
mod sqrt;
mod value;

#[cfg(test)]
mod tests;
