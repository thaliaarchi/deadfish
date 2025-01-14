/// Wrapping integer square root.
///
/// Reference SageMath [`IntegerModRing(2**32)(x).sqrt(all=True, extend=False)`][sagecell],
/// which is `sqrt` from [`IntegerMod_abstract`][sqrt-src] [[docs][sqrt-docs]].
///
/// [sagecell]: https://sagecell.sagemath.org/?z=eJzzzCtJTU8t8s1PCcrMS9cw0tIyNtLUMDTX1CsuLCrRSMzJsQ0pKk3VUUitKEnNS7F1S8wpTtUEAOXDEc8=&lang=sage&interacts=eJyLjgUAARUAuQ==
/// [sqrt-src]: https://github.com/sagemath/sage/blob/1be0a58926030043e4f5dbed850b4c3f809f376b/src/sage/rings/finite_rings/integer_mod.pyx#L1133
/// [sqrt-docs]: https://doc.sagemath.org/html/en/reference/finite_rings/sage/rings/finite_rings/integer_mod.html#sage.rings.finite_rings.integer_mod.IntegerMod_abstract.sqrt

/// Wrapping integer square root.
pub trait WrappingSqrt
where
    Self: Sized,
{
    /// Computes all square roots under wrapping arithmetic. That is, all values
    /// `y` such that `y.wrapping_mul(y) == self`.
    fn wrapping_sqrt(&self) -> Vec<Self>;

    /// Returns whether it is a square under wrapping arithmetic. That is,
    /// whether there is some value `y` such that `y.wrapping_mul(y) == self`.
    fn is_wrapping_square(&self) -> bool;
}

macro_rules! impl_unsigned_wrapping_sqrt(($($T:ident),* $(,)?) => {
    $(impl WrappingSqrt for $T {
        fn wrapping_sqrt(&self) -> Vec<Self> {
            const SQRTS_OF_ONE: [$T; 4] = [
                1,
                (1 << $T::BITS - 1) - 1,
                (1 << $T::BITS - 1) + 1,
                $T::MAX,
            ];

            let x = *self;
            if x == 0 {
                // TODO: The step should be $T not usize.
                return (0..=$T::MAX).step_by(1 << $T::BITS.div_ceil(2)).collect();
            }
            if x == 1 {
                return SQRTS_OF_ONE.to_vec();
            }

            // Factor out powers of 2. It must be an even power of 2.
            let tz = x.trailing_zeros();
            if tz % 2 != 0 {
                return Vec::new();
            }
            let x = x >> tz;
            let e = $T::BITS - tz;

            // Find y^2 = x (mod 32).
            let mut y: $T = match x % 32 {
                1 => 1,
                9 => 3,
                25 => 5,
                17 => 7,
                _ => return Vec::new(),
            };
            let mut t = y.wrapping_mul(y).wrapping_sub(x) / 32;
            for i in 4..e - 1 {
                if t & 1 != 0 {
                    y |= 1 << i;
                    t += y - (1 << i - 1);
                }
                t >>= 1;
            }

            if tz != 0 {
                // Squares have even number of trailing zeros.
                let valuation = tz / 2;
                let exp = $T::BITS - valuation - 1;
                // TODO: Pre-allocate Vec.
                let mut sqrts = Vec::new();
                for sqrt in SQRTS_OF_ONE.map(|sqrt| sqrt.wrapping_mul(y)) {
                    for a in (0..=$T::MAX).step_by(1 << exp) {
                        sqrts.push(sqrt.wrapping_mul(1 << valuation).wrapping_add(a));
                    }
                }
                sqrts.sort_unstable();
                sqrts.dedup();
                sqrts
            } else {
                // Handle odd numbers separately to avoid deduplicating.
                let mut sqrts = SQRTS_OF_ONE.map(|sqrt| sqrt.wrapping_mul(y)).to_vec();
                sqrts.sort_unstable();
                sqrts
            }
        }

        fn is_wrapping_square(&self) -> bool {
            // Squares in Z/2^e are of the form 4^n * (8*m + 1).
            let x = *self;
            if x == 0 {
                return true;
            }
            let tz = x.trailing_zeros();
            tz % 2 == 0 && (x >> tz) % 8 == 1
        }
    })*
});

macro_rules! impl_signed_wrapping_sqrt(($($T:ident $UnsignedT:ident),* $(,)?) => {
    $(impl WrappingSqrt for $T {
        fn wrapping_sqrt(&self) -> Vec<Self> {
            let mut sqrts = (*self as $UnsignedT)
                .wrapping_sqrt()
                .into_iter()
                .map(|sqrt| sqrt as $T)
                .collect::<Vec<_>>();
            sqrts.sort_unstable();
            sqrts
        }

        fn is_wrapping_square(&self) -> bool {
            (*self as $UnsignedT).is_wrapping_square()
        }
    })*
});

impl_unsigned_wrapping_sqrt!(u8, u16, u32, u64, u128, usize);
impl_signed_wrapping_sqrt!(i8 u8, i16 u16, i32 u32, i64 u64, i128 u128, isize usize);

#[cfg(test)]
mod tests {
    use crate::WrappingSqrt;

    #[test]
    fn wrapping_sqrt_u16() {
        let mut sqrts = vec![Vec::new(); 1 << u16::BITS];
        for y in u16::MIN..=u16::MAX {
            sqrts[y.wrapping_mul(y) as usize].push(y);
        }
        for (x, expect) in (u16::MIN..=u16::MAX).zip(sqrts) {
            assert_eq!(x.wrapping_sqrt(), expect, "{x}.wrapping_sqrt()");
        }
    }

    #[test]
    fn wrapping_sqrt_i16() {
        let mut sqrts = vec![Vec::new(); 1 << i16::BITS];
        for y in i16::MIN..=i16::MAX {
            sqrts[y.wrapping_mul(y).wrapping_sub(i16::MIN) as u16 as usize].push(y);
        }
        for (x, expect) in (i16::MIN..=i16::MAX).zip(sqrts) {
            assert_eq!(x.wrapping_sqrt(), expect, "{x}.wrapping_sqrt()");
        }
    }

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
