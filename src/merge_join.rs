use std::cmp::Ordering;
use std::iter::Fuse;
use std::fmt;

use either::Either;

use super::adaptors::{PutBack, put_back};
use crate::either_or_both::EitherOrBoth;
use crate::size_hint::{self, SizeHint};
#[cfg(doc)]
use crate::Itertools;

pub trait MergePredicate<I, J, T>: FnMut(&I, &J) -> T {
    type Item;
    fn left(left: I) -> Self::Item;
    fn right(right: J) -> Self::Item;
    fn merge(&mut self, left: I, right: J) -> (Option<I>, Option<J>, Self::Item);
    fn size_hint(left: SizeHint, right: SizeHint) -> SizeHint;
}

/// Return an iterator adaptor that merge-joins items from the two base iterators in ascending order.
///
/// [`IntoIterator`] enabled version of [`Itertools::merge_join_by`].
pub fn merge_join_by<I, J, F, T>(left: I, right: J, cmp_fn: F)
    -> MergeJoinBy<I::IntoIter, J::IntoIter, F, T>
    where I: IntoIterator,
          J: IntoIterator,
          F: FnMut(&I::Item, &J::Item) -> T,
{
    MergeJoinBy {
        left: put_back(left.into_iter().fuse()),
        right: put_back(right.into_iter().fuse()),
        cmp_fn,
    }
}

/// An iterator adaptor that merge-joins items from the two base iterators in ascending order.
///
/// See [`.merge_join_by()`](crate::Itertools::merge_join_by) for more information.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct MergeJoinBy<I, J, F, T>
    where I: Iterator,
          J: Iterator,
          F: FnMut(&I::Item, &J::Item) -> T,
{
    left: PutBack<Fuse<I>>,
    right: PutBack<Fuse<J>>,
    cmp_fn: F
}

impl<I, J, F: FnMut(&I, &J) -> Ordering> MergePredicate<I, J, Ordering> for F {
    type Item = EitherOrBoth<I, J>;

    fn left(left: I) -> Self::Item {
        EitherOrBoth::Left(left)
    }

    fn right(right: J) -> Self::Item {
        EitherOrBoth::Right(right)
    }

    fn merge(&mut self, left: I, right: J) -> (Option<I>, Option<J>, Self::Item) {
        match self(&left, &right) {
            Ordering::Equal => (None, None, EitherOrBoth::Both(left, right)),
            Ordering::Less => (None, Some(right), EitherOrBoth::Left(left)),
            Ordering::Greater => (Some(left), None, EitherOrBoth::Right(right)),
        }
    }

    fn size_hint(left: SizeHint, right: SizeHint) -> SizeHint {
        let (a_lower, a_upper) = left;
        let (b_lower, b_upper) = right;

        let lower = ::std::cmp::max(a_lower, b_lower);

        let upper = match (a_upper, b_upper) {
            (Some(x), Some(y)) => x.checked_add(y),
            _ => None,
        };

        (lower, upper)
    }
}

impl<I, J, F: FnMut(&I, &J) -> bool> MergePredicate<I, J, bool> for F {
    type Item = Either<I, J>;

    fn left(left: I) -> Self::Item {
        Either::Left(left)
    }

    fn right(right: J) -> Self::Item {
        Either::Right(right)
    }

    fn merge(&mut self, left: I, right: J) -> (Option<I>, Option<J>, Self::Item) {
        if self(&left, &right) {
            (None, Some(right), Either::Left(left))
        } else {
            (Some(left), None, Either::Right(right))
        }
    }

    fn size_hint(left: SizeHint, right: SizeHint) -> SizeHint {
        // Not ExactSizeIterator because size may be larger than usize
        size_hint::add(left, right)
    }
}

impl<I, J, F, T> Clone for MergeJoinBy<I, J, F, T>
    where I: Iterator,
          J: Iterator,
          PutBack<Fuse<I>>: Clone,
          PutBack<Fuse<J>>: Clone,
          F: FnMut(&I::Item, &J::Item) -> T + Clone,
{
    clone_fields!(left, right, cmp_fn);
}

impl<I, J, F, T> fmt::Debug for MergeJoinBy<I, J, F, T>
    where I: Iterator + fmt::Debug,
          I::Item: fmt::Debug,
          J: Iterator + fmt::Debug,
          J::Item: fmt::Debug,
          F: FnMut(&I::Item, &J::Item) -> T,
{
    debug_fmt_fields!(MergeJoinBy, left, right);
}

impl<I, J, F, T> Iterator for MergeJoinBy<I, J, F, T>
    where I: Iterator,
          J: Iterator,
          F: MergePredicate<I::Item, J::Item, T>,
{
    type Item = F::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.left.next(), self.right.next()) {
            (None, None) => None,
            (Some(left), None) => Some(F::left(left)),
            (None, Some(right)) => Some(F::right(right)),
            (Some(left), Some(right)) => {
                let (left, right, next) = self.cmp_fn.merge(left, right);
                if let Some(left) = left {
                    self.left.put_back(left);
                }
                if let Some(right) = right {
                    self.right.put_back(right);
                }
                Some(next)
            }
        }
    }

    fn size_hint(&self) -> SizeHint {
        F::size_hint(self.left.size_hint(), self.right.size_hint())
    }

    fn count(mut self) -> usize {
        let mut count = 0;
        loop {
            match (self.left.next(), self.right.next()) {
                (None, None) => break count,
                (Some(_left), None) => break count + 1 + self.left.into_parts().1.count(),
                (None, Some(_right)) => break count + 1 + self.right.into_parts().1.count(),
                (Some(left), Some(right)) => {
                    count += 1;
                    let (left, right, _) = self.cmp_fn.merge(left, right);
                    if let Some(left) = left {
                        self.left.put_back(left);
                    }
                    if let Some(right) = right {
                        self.right.put_back(right);
                    }
                }
            }
        }
    }

    fn last(mut self) -> Option<Self::Item> {
        let mut previous_element = None;
        loop {
            match (self.left.next(), self.right.next()) {
                (None, None) => break previous_element,
                (Some(left), None) => {
                    break Some(F::left(
                        self.left.into_parts().1.last().unwrap_or(left),
                    ))
                }
                (None, Some(right)) => {
                    break Some(F::right(
                        self.right.into_parts().1.last().unwrap_or(right),
                    ))
                }
                (Some(left), Some(right)) => {
                    let (left, right, elem) = self.cmp_fn.merge(left, right);
                    previous_element = Some(elem);
                    if let Some(left) = left {
                        self.left.put_back(left);
                    }
                    if let Some(right) = right {
                        self.right.put_back(right);
                    }
                }
            }
        }
    }

    fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
        loop {
            if n == 0 {
                break self.next();
            }
            n -= 1;
            match (self.left.next(), self.right.next()) {
                (None, None) => break None,
                (Some(_left), None) => break self.left.nth(n).map(F::left),
                (None, Some(_right)) => break self.right.nth(n).map(F::right),
                (Some(left), Some(right)) => {
                    let (left, right, _) = self.cmp_fn.merge(left, right);
                    if let Some(left) = left {
                        self.left.put_back(left);
                    }
                    if let Some(right) = right {
                        self.right.put_back(right);
                    }
                }
            }
        }
    }
}
