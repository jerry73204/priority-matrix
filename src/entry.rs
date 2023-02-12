//! Entry types.

#[derive(Debug, PartialEq, Eq)]
pub struct BorrowedEntry<'a, R, C, W> {
    pub row: &'a R,
    pub column: &'a C,
    pub weight: &'a W,
}

impl<'a, R, C, W> Clone for BorrowedEntry<'a, R, C, W> {
    fn clone(&self) -> Self {
        Self {
            row: self.row,
            column: self.column,
            weight: self.weight,
        }
    }
}

impl<'a, R, C, W> Copy for BorrowedEntry<'a, R, C, W> {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedEntry<R, C, W> {
    pub row: R,
    pub column: C,
    pub weight: W,
}
