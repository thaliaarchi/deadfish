/// Wrapping integer square root.
///
/// Reference SageMath [`IntegerModRing(2**32)(x).sqrt(all=True, extend=False)`][sagecell],
/// which is `sqrt` from [`IntegerMod_abstract`][sqrt-src] [[docs][sqrt-docs]].
///
/// [sagecell]: https://sagecell.sagemath.org/?z=eJzzzCtJTU8t8s1PCcrMS9cw0tIyNtLUMDTX1CsuLCrRSMzJsQ0pKk3VUUitKEnNS7F1S8wpTtUEAOXDEc8=&lang=sage&interacts=eJyLjgUAARUAuQ==
/// [sqrt-src]: https://github.com/sagemath/sage/blob/1be0a58926030043e4f5dbed850b4c3f809f376b/src/sage/rings/finite_rings/integer_mod.pyx#L1133
/// [sqrt-docs]: https://doc.sagemath.org/html/en/reference/finite_rings/sage/rings/finite_rings/integer_mod.html#sage.rings.finite_rings.integer_mod.IntegerMod_abstract.sqrt

/// Wrapping integer square root.
pub trait WrappingSqrt {
    /// Returns whether it is a square under wrapping arithmetic. That is,
    /// whether there is some value `y` such that `y.wrapping_mul(y) == self`.
    fn is_wrapping_square(&self) -> bool;
}

macro_rules! impl_wrapping_sqrt(($($T:ident $UnsignedT:ident),* $(,)?) => {
    $(impl WrappingSqrt for $T {
        fn is_wrapping_square(&self) -> bool {
            let x = *self;
            if x == 0 {
                return true;
            }
            let tz = x.trailing_zeros();
            tz % 2 == 0 && (x as $UnsignedT >> tz) % 8 == 1
        }
    })*
});

impl_wrapping_sqrt!(
    u8 u8, u16 u16, u32 u32, u64 u64, u128 u128, usize usize,
    i8 u8, i16 u16, i32 u32, i64 u64, i128 u128, isize usize,
);

#[cfg(test)]
mod tests {
    use crate::WrappingSqrt;

    #[test]
    fn is_wrapping_square_u16() {
        let mut is_square = vec![false; 1 << u16::BITS];
        for y in u16::MIN..=u16::MAX {
            is_square[y.wrapping_mul(y) as usize] = true;
        }
        for (x, expect) in (u16::MIN..=u16::MAX).zip(is_square) {
            assert_eq!(x.is_wrapping_square(), expect, "{x}.is_wrapping_square()");
        }
    }

    #[test]
    fn is_wrapping_square_i16() {
        let mut is_square = vec![false; 1 << i16::BITS];
        for y in i16::MIN..=i16::MAX {
            is_square[y.wrapping_mul(y).wrapping_sub(i16::MIN) as u16 as usize] = true;
        }
        for (x, expect) in (i16::MIN..=i16::MAX).zip(is_square) {
            assert_eq!(x.is_wrapping_square(), expect, "{x}.is_wrapping_square()");
        }
    }
}
