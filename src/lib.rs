//! This crate provides tools to iteratively compute the binomial coefficient.

use std::ops;

#[inline(always)]
fn gcd(mut u: u32, mut v: u32) -> u32 {
    if u == 0 || v == 0 { return 1 } // This is matematically not neccessarily
                                     // but acceptable for our uses (and better
                                     // than returning 0.

    while v > 0 {
        let t = u;
        u = v;
        v = t % v;
    }
    u
}

/// Calculate (a * b) / c after removing the gcd. This function should only be
/// used if the result is guaranteed to be an integer.
#[inline(always)]
fn mul_div_gcd(mut a: u32, mut b: u32, mut c: u32) -> Option<u32> {
    let g = gcd(a, c);
    a /= g;
    c /= g;
    let g = gcd(b, c);
    b /= g;
    c /= g;
    a.checked_mul(b).and_then(|ab| ab.checked_div(c))
}

fn binom(n: u32, k: u32) -> u32 {
    if k == 0 || k == n {
        1
    } else if n < k {
        0
    } else {
        mul_div_gcd(binom(n - 1, k - 1), n, k).expect(&format!("cannot calculate `{}` choose `{}`, would overflow", n, k))
    }
}

/// Provides methods to calculate the binomial coefficient for the next
/// higher/lower `n`/`k`.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BinomialIter {
    n: u32,
    k: u32,
    binom: u32,
}

impl BinomialIter {
    /// Create a new `BinomialIter`. This will calculate the binomial
    /// coefficient once using the recursive definition.
    ///
    /// # Panics
    /// If `k > n`
    #[inline]
    pub fn new(n: u32, k: u32) -> BinomialIter {
        assert!(k <= n, "k <= is currently no supported");

        BinomialIter {
            n: n,
            k: k,
            binom: binom(n, k),
        }
    }

    /// Access the current value of `n`.
    #[inline]
    pub fn n(&self) -> u32 { self.n }

    /// Access the current value of `k`.
    #[inline]
    pub fn k(&self) -> u32 { self.k }

    /// Access the current value of `n` choose `k`.
    #[inline]
    pub fn binom(&self) -> u32 { self.binom }

    /// Increase `n` by one and update the internal state accordingly.
    ///
    /// Returns `None` when calculating `n + 1` choose `k` would overflow,
    /// otherwise `Some((n + 1, binom))`.
    #[inline]
    pub fn inc_n(&mut self) -> Option<(u32, u32)> {
        match mul_div_gcd(self.binom, self.n + 1, self.n + 1 - self.k) {
            Some(binom) => {
                self.n += 1;
                self.binom = binom;
                Some((self.n, binom))
            }
            _ => None,
        }
    }

    /// Decrease `n` by one and update the internal state accordingly.
    ///
    /// Returns `None` when calculating `n - 1` choose `k` would overflow or
    /// `n - 1 < k`, otherwise `Some((n - 1, binom))`.
    #[inline]
    pub fn dec_n(&mut self) -> Option<(u32, u32)> {
        match mul_div_gcd(self.binom, self.n - self.k, self.n) {
            Some(binom) if self.n > self.k => {
                self.n -= 1;
                self.binom = binom;
                Some((self.n, binom))
            }
            _ => None,
        }
    }

    /// Increase `k` by one and update the internal state accordingly.
    ///
    /// Returns `None` when calculating `n` choose `k + 1` would overflow or
    /// `n < k + 1`, otherwise `Some((k + 1, binom))`.
    #[inline]
    pub fn inc_k(&mut self) -> Option<(u32, u32)> {
        match mul_div_gcd(self.binom, self.n - self.k, self.k + 1) {
            Some(binom) if self.n > self.k => {
                self.k += 1;
                self.binom = binom;
                Some((self.k, binom))
            }
            _ => None
        }
    }

    /// Decrease `k` by one and update the internal state accordingly.
    ///
    /// Returns `None` when calculating `n` choose `k - 1` would overflow or
    /// `k - 1 < 0`, otherwise `Some((k - 1, binom))`
    #[inline]
    pub fn dec_k(&mut self) -> Option<(u32, u32)> {
        match mul_div_gcd(self.binom, self.k, self.n - self.k + 1) {
            Some(binom) if self.k > 0 => {
                self.k -= 1;
                self.binom = binom;
                Some((self.k, binom))
            }
            _ => None
        }
    }
}

macro_rules! def_iter {{
    $(#[$($iter_doc:meta)*])*
    > $Iter:ident: $nk:ident, $nfn:ident;
    $(#[$($cfn_doc:meta)*])*
    > $cfn:ident
} => {
    $(#[$($iter_doc)*])*
    pub struct $Iter {
        inner: BinomialIter,
        first: bool,
    }

    impl BinomialIter {
        $(#[$($cfn_doc)*])*
        #[inline]
        pub fn $cfn(self) -> $Iter {
            $Iter {
                inner: self,
                first: true,
            }
        }
    }

    impl Iterator for $Iter {
        type Item = (u32, u32);

        #[inline]
        fn next(&mut self) -> Option<(u32, u32)> {
            if self.first {
                self.first = false;
                Some((self.inner.$nk, self.inner.binom))
            } else {
                self.inner.$nfn()
            }
        }
    }

    impl ops::Deref for $Iter {
        type Target = BinomialIter;

        fn deref(&self) -> &BinomialIter {
            &self.inner
        }
    }

    impl ops::DerefMut for $Iter {
        fn deref_mut(&mut self) -> &mut BinomialIter {
            &mut self.inner
        }
    }
}}

def_iter! {
    /// An iterator which wraps a `BinomialIter` and returns the result of it's
    /// `inc_n` method when `next` is called.
    > IncNIter: n, inc_n;
    /// Returns an iterator which wraps this `BinomialIter`, returns the current
    /// value of `n` and `binom` on the first call to `next` and the result of
    /// calling `inc_n` on the underlying `BinominalIter` for subsequent calls
    /// to next.
    > iter_inc_n
}

def_iter! {
    /// An iterator which wraps a `BinomialIter` and returns the result of it's
    /// `dec_n` method when `next` is called.
    > DecNIter: n, dec_n;
    /// Returns an iterator which wraps this `BinomialIter`, returns the current
    /// value of `n` and `binom` on the first call to `next` and the result of
    /// calling `dec_n` on the underlying `BinominalIter` for subsequent calls
    /// to next.
    > iter_dec_n
}

def_iter! {
    /// An iterator which wraps a `BinomialIter` and returns the result of it's
    /// `inc_k` method when `next` is called.
    > IncKIter: k, inc_k;
    /// Returns an iterator which wraps this `BinomialIter`, returns the current
    /// value of `k` and `binom` on the first call to `next` and the result of
    /// calling `inc_k` on the underlying `BinominalIter` for subsequent calls
    /// to next.
    > iter_inc_k
}

def_iter! {
    /// An iterator which wraps a `BinomialIter` and returns the result of it's
    /// `dec_k` method when `next` is called.
    > DecKIter: k, dec_k;
    /// Returns an iterator which wraps this `BinomialIter`, returns the current
    /// value of `k` and `binom` on the first call to `next` and the result of
    /// calling `dec_k` on the underlying `BinominalIter` for subsequent calls
    /// to next.
    > iter_dec_k
}

#[cfg(test)]
mod test;
