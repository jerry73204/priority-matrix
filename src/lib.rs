//! The crate implements priority matrix that supports per-row and
//! per-column maximum key queries.
//!
//! ```rust
//! use priority_matrix::PriorityMatrix;
//!
//! let matrix: PriorityMatrix<char, &str, i32> = [
//!     ('a', "alpha", 0),
//!     ('a', "beta", 3),
//!     ('b', "alpha", 2),
//!     ('b', "beta", 1),
//! ]
//! .into_iter()
//! .collect();
//!
//! // Get the maximum entry
//! let entry = matrix.peek().unwrap();
//! assert_eq!(entry.row, &'a');
//! assert_eq!(entry.column, &"beta");
//! assert_eq!(entry.weight, &3);
//!
//! // Get the maximum entry in a row
//! let entry = matrix.peek_from_row(&'b').unwrap();
//! assert_eq!(entry.row, &'b');
//! assert_eq!(entry.column, &"alpha");
//! assert_eq!(entry.weight, &2);
//!
//! // Get the maximum entry in a column
//! let entry = matrix.peek_from_column(&"alpha").unwrap();
//! assert_eq!(entry.row, &'b');
//! assert_eq!(entry.column, &"alpha");
//! assert_eq!(entry.weight, &2);
//! ```

pub mod entry;
pub mod iter;

use crate::{
    entry::{BorrowedEntry, OwnedEntry},
    iter::{IntoIter, Iter},
};
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::{collections::HashMap, hash::Hash};

/// The 2-dimensional matrix that supports per-row and per-column
/// maximum key queries.
#[derive(Debug, Clone)]
pub struct PriorityMatrix<R, C, W>
where
    R: Clone + Eq + Hash,
    C: Clone + Eq + Hash,
    W: Clone + Ord,
{
    entries: PriorityQueue<(R, C), W>,
    rows: HashMap<R, PriorityQueue<C, W>>,
    cols: HashMap<C, PriorityQueue<R, W>>,
}

impl<R, C, W> PriorityMatrix<R, C, W>
where
    R: Clone + Eq + Hash,
    C: Clone + Eq + Hash,
    W: Clone + Ord,
{
    pub fn new(&self) -> Self {
        Self::default()
    }

    pub fn insert(&mut self, row: R, col: C, weight: W) -> Option<W> {
        let prev_weight = self
            .entries
            .push((row.clone(), col.clone()), weight.clone());
        self.rows
            .entry(row.clone())
            .or_insert_with(PriorityQueue::default)
            .push(col.clone(), weight.clone());
        self.cols
            .entry(col)
            .or_insert_with(PriorityQueue::default)
            .push(row, weight);
        prev_weight
    }

    pub fn peek(&self) -> Option<BorrowedEntry<'_, R, C, W>> {
        let ((row, col), weight) = self.entries.peek()?;
        Some(BorrowedEntry {
            row,
            column: col,
            weight,
        })
    }

    pub fn peek_from_row<'a>(&'a self, row: &'a R) -> Option<BorrowedEntry<'_, R, C, W>> {
        let (col, _) = self.rows.get(row)?.peek().unwrap();
        let key = (row.clone(), col.clone());
        let (_, weight) = self.entries.get(&key).unwrap();
        Some(BorrowedEntry {
            row,
            column: col,
            weight,
        })
    }

    pub fn peek_from_column<'a>(&'a self, col: &'a C) -> Option<BorrowedEntry<'a, R, C, W>> {
        let (row, _) = self.cols.get(col)?.peek().unwrap();
        let key = (row.clone(), col.clone());
        let (_, weight) = self.entries.get(&key).unwrap();
        Some(BorrowedEntry {
            row,
            column: col,
            weight,
        })
    }

    pub fn pop(&mut self) -> Option<OwnedEntry<R, C, W>> {
        let ((row, col), weight) = self.entries.pop()?;
        self.rows.get_mut(&row).unwrap().remove(&col);
        self.rows.get_mut(&row).unwrap().remove(&col);
        Some(OwnedEntry {
            row,
            column: col,
            weight,
        })
    }

    pub fn pop_from_row(&mut self, row: &R) -> Option<OwnedEntry<R, C, W>> {
        let (col, weight) = self.rows.get_mut(row)?.pop().unwrap();
        let key = (row.clone(), col.clone());
        self.entries.remove(&key);
        self.cols.get_mut(&col).unwrap().remove(row);
        Some(OwnedEntry {
            row: row.clone(),
            column: col,
            weight,
        })
    }

    pub fn pop_from_column(&mut self, col: &C) -> Option<OwnedEntry<R, C, W>> {
        let (row, weight) = self.cols.get_mut(col)?.pop().unwrap();
        let key = (row.clone(), col.clone());
        self.entries.remove(&key);
        self.rows.get_mut(&row).unwrap().remove(col);
        Some(OwnedEntry {
            row,
            column: col.clone(),
            weight,
        })
    }

    pub fn remove(&mut self, row: &R, col: &C) -> bool {
        let ok = self.entries.remove(&(row.clone(), col.clone())).is_some();
        if !ok {
            return false;
        }

        self.rows.get_mut(row).unwrap().remove(col);
        self.cols.get_mut(col).unwrap().remove(row);
        true
    }

    pub fn remove_row(&mut self, row: &R) {
        self.rows
            .remove(row)
            .into_iter()
            .flatten()
            .map(|(curr_col, _)| (row.clone(), curr_col))
            .for_each(|(row, col)| {
                if let Some(queue) = self.cols.get_mut(&col) {
                    queue.remove(&row);
                }
                self.entries.remove(&(row, col));
            });
    }

    pub fn remove_column(&mut self, col: &C) {
        self.cols
            .remove(col)
            .into_iter()
            .flatten()
            .map(|(curr_row, _)| (curr_row, col.clone()))
            .for_each(|(row, col)| {
                if let Some(queue) = self.rows.get_mut(&row) {
                    queue.remove(&col);
                }
                self.entries.remove(&(row, col));
            });
    }

    pub fn remove_row_and_column(&mut self, row: &R, col: &C) {
        let row_keys = self
            .rows
            .remove(row)
            .into_iter()
            .flatten()
            .map(|(curr_col, _)| (row.clone(), curr_col));
        let col_keys = self
            .cols
            .remove(col)
            .into_iter()
            .flatten()
            .map(|(curr_row, _)| (curr_row, col.clone()));
        let all_keys = row_keys.chain(col_keys);

        all_keys.for_each(|key| {
            self.entries.remove(&key);
        });
    }

    pub fn iter(&self) -> Iter<'_, R, C, W> {
        Iter {
            iter: self.entries.iter(),
        }
    }
}

impl<R, C, W> Default for PriorityMatrix<R, C, W>
where
    R: Clone + Eq + Hash,
    C: Clone + Eq + Hash,
    W: Clone + Ord,
{
    fn default() -> Self {
        Self {
            entries: PriorityQueue::new(),
            rows: HashMap::new(),
            cols: HashMap::new(),
        }
    }
}

impl<R, C, W> FromIterator<(R, C, W)> for PriorityMatrix<R, C, W>
where
    R: Clone + Eq + Hash,
    C: Clone + Eq + Hash,
    W: Clone + Ord,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (R, C, W)>,
    {
        let entries: PriorityQueue<(R, C), W> = iter
            .into_iter()
            .map(|(row, col, val)| ((row, col), val))
            .collect();
        let rows: HashMap<R, PriorityQueue<C, W>> = entries
            .iter()
            .map(|((row, col), iou)| (row.clone(), (col.clone(), iou.clone())))
            .into_grouping_map()
            .collect();
        let cols: HashMap<C, PriorityQueue<R, W>> = entries
            .iter()
            .map(|((row, col), iou)| (col.clone(), (row.clone(), iou.clone())))
            .into_grouping_map()
            .collect();

        PriorityMatrix {
            entries,
            rows,
            cols,
        }
    }
}

impl<R, C, W> IntoIterator for PriorityMatrix<R, C, W>
where
    R: Clone + Eq + Hash,
    C: Clone + Eq + Hash,
    W: Clone + Ord,
{
    type Item = (R, C, W);
    type IntoIter = IntoIter<R, C, W>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.entries.into_iter(),
        }
    }
}
