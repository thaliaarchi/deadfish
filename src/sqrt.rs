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

            fn modular_sqrt(x: $T, e: u32) -> Vec<$T> {
                if x == 1 {
                    SQRTS_OF_ONE.to_vec()
                } else if !x.is_wrapping_square() {
                    Vec::new()
                } else if x % 2 == 0 {
                    let max = $T::MAX >> ($T::BITS - e);
                    if x == 0 {
                        (0..=max).step_by(1 << e.div_ceil(2)).collect()
                    } else {
                        let tz = x.trailing_zeros();
                        // Squares have even number of trailing zeros.
                        let valuation = tz / 2;
                        let mut exp = e - valuation - 1;
                        if 2 * exp < e {
                            exp = exp.div_ceil(2);
                        }
                        let p_val = 1 << valuation;
                        let p_exp = 1 << exp;
                        // TODO: Pre-allocate Vec.
                        // TODO: Inline this recursive call, since it does not
                        // recurse again.
                        let mut sqrts = Vec::new();
                        for sqrt in modular_sqrt(x >> tz, $T::BITS - valuation) {
                            for a in (0..=max).step_by(p_exp) {
                                sqrts.push(sqrt.wrapping_mul(p_val).wrapping_add(a));
                            }
                        }
                        sqrts.sort_unstable();
                        sqrts.dedup();
                        sqrts
                    }
                } else {
                    let mut y = modular_sqrt_odd(x, e);
                    if y > 1 << (e - 1) {
                        y = y.wrapping_neg();
                    }
                    let mut sqrts = SQRTS_OF_ONE.map(|sqrt| sqrt.wrapping_mul(y)).to_vec();
                    sqrts.sort_unstable();
                    sqrts.dedup();
                    sqrts
                }
            }

            fn modular_sqrt_odd(x: $T, e: u32) -> $T {
                debug_assert_eq!(x.trailing_zeros(), 0);
                if x == 0 || x == 1 {
                    return x;
                }
                if x % 8 != 1 {
                    panic!("must be a square");
                }
                let mut y = (1 as $T..8)
                    .step_by(2)
                    .filter(|&i| i.wrapping_mul(i) & 31 == x & 31)
                    .next()
                    .unwrap();
                let mut t = y.wrapping_mul(y).wrapping_sub(x) >> 5;
                for i in 4..e - 1 {
                    if t & 1 != 0 {
                        y |= 1 << i;
                        t += y - (1 << i - 1);
                    }
                    t >>= 1;
                }
                y
            }

            modular_sqrt(*self, $T::BITS)
        }

        fn is_wrapping_square(&self) -> bool {
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
