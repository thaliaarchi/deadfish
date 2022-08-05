// Copyright (C) 2022 Andrew Archibald
//
// deadfish is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version. You should have received a copy of the GNU Lesser General
// Public License along with deadfish. If not, see http://www.gnu.org/licenses/.

/// Deadfish instructions.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Inst {
    /// Increment
    I,
    /// Decrement
    D,
    /// Square
    S,
    /// Output
    O,
    /// Print a line feed
    Blank,
}
