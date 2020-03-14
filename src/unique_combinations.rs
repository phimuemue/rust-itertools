use std::fmt;

/// An iterator to iterate through all the `n`-length combinations in an iterator.
///
/// See [`.unique_combinations()`](../trait.Itertools.html#method.unique_combinations) for moref information.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct UniqueCombinations<I: Iterator> {
    indices: Vec<usize>,
    pool: Vec<I::Item>,
    first: bool,
}

impl<I> Clone for UniqueCombinations<I>
where
    I: Iterator,
    I::Item: Clone,
{
    clone_fields!(indices, pool, first);
}

impl<I> fmt::Debug for UniqueCombinations<I>
where
    I: Iterator + fmt::Debug,
    I::Item: fmt::Debug,
{
    debug_fmt_fields!(UniqueCombinations, indices, pool, first);
}

/// Create a new `UniqueCombinations` from a iterator with clonable and sorable Items.
pub fn unique_combinations<I>(iter: I, len: usize) -> UniqueCombinations<I>
where
    I: Iterator,
    I::Item: Ord,
{
    let mut pool: Vec<_> = iter.collect();
    pool.sort_unstable();
    UniqueCombinations {
        indices: (0..len).collect(),
        pool,
        first: true,
    }
}

impl<I> UniqueCombinations<I>
where
    I: Iterator,
    I::Item: Ord + Clone,
{
    #[inline]
    fn generate(&self) -> Option<Vec<I::Item>> {
        Some(self.indices.iter().map(|n| self.pool[*n].clone()).collect())
    }
}

impl<I> Iterator for UniqueCombinations<I>
where
    I: Iterator,
    I::Item: Clone + Ord,
{
    type Item = Vec<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        let indices_len = self.indices.len();
        let pool_len = self.pool.len();
        if self.first {
            if indices_len > pool_len {
                return None;
            }
            self.first = false;
        } else if indices_len == 0 {
            return None;
        } else {
            // locate the back_most digit that can be bumped
            let mut i = indices_len;
            while true {
                if i==0 {
                    break;
                }
                i -= 1;
                if self.pool[self.indices[i]] != self.pool[i + pool_len - indices_len] {
                    let bump_source = self.indices[i];
                    // locate the position where the number needs to be set
                    for bump_target in bump_source + 1..pool_len {
                        if self.pool[bump_source] < self.pool[bump_target] { // must be true for at least one bump_target
                            //sets all the indices right of the bump_target
                            self.indices[i] = bump_target;
                            for j in i+1..indices_len {
                                self.indices[j] = self.indices[j - 1] + 1;
                            }
                            return self.generate();
                        }
                    }
                    assert!(false);
                }
            }
            return None;
        }
        self.generate()
    }
}
