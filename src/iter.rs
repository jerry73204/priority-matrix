//! Iterator types.

use std::hash::Hash;

pub struct Iter<'a, R, C, W>
where
    R: Eq + Hash,
    C: Eq + Hash,
    W: Ord,
{
    pub(crate) iter: priority_queue::core_iterators::Iter<'a, (R, C), W>,
}

impl<'a, R, C, W> Iterator for Iter<'a, R, C, W>
where
    R: Eq + Hash,
    C: Eq + Hash,
    W: Ord,
{
    type Item = (&'a R, &'a C, &'a W);

    fn next(&mut self) -> Option<Self::Item> {
        let ((row, col), weight) = self.iter.next()?;
        Some((row, col, weight))
    }
}

pub struct IntoIter<R, C, W>
where
    R: Eq + Hash,
    C: Eq + Hash,
    W: Ord,
{
    pub(crate) iter: priority_queue::core_iterators::IntoIter<(R, C), W>,
}

impl<R, C, W> Iterator for IntoIter<R, C, W>
where
    R: Eq + Hash,
    C: Eq + Hash,
    W: Ord,
{
    type Item = (R, C, W);

    fn next(&mut self) -> Option<Self::Item> {
        let ((row, col), weight) = self.iter.next()?;
        Some((row, col, weight))
    }
}
