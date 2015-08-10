use super::{BinomialIter, binom};

const N: u32 = 1000;

// Test `inc` and `dec` in the same test case because we don't know for which
// `n` choose `k` we can still use `binom`.
// #[test]
fn n_iter() {
    for k in 0 .. N {
        let mut it = BinomialIter::new(k, k).iter_inc_n();

        {
            let mut it = it.by_ref().take(N as usize);

            while let Some((n, bin)) = it.next() {
                assert!(binom(n, k) == bin);
            }
        }

        let mut it = (*it).iter_dec_n();

        while let Some((n, bin)) = it.next() {
            assert!(binom(n, k) == bin);
        }
    }
}

// Test `inc` and `dec` in the same test case because we don't know for which
// `n` choose `k` we can still use `binom`.
#[test]
fn k_iter() {
    for n in 0 .. N {
        let mut it = BinomialIter::new(n, 0).iter_inc_k();

        while let Some((k, bin)) = it.next() {
            assert!(binom(n, k) == bin);
            println!("{}|{} = {}", n, k, bin);
        }

        let mut it = (*it).iter_dec_k();

        while let Some((k, bin)) = it.next() {
            assert!(binom(n, k) == bin);
            println!("{}|{} = {}", n, k, bin);
        }
    }
}

