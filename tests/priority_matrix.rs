use priority_matrix::{
    entry::{BorrowedEntry, OwnedEntry},
    PriorityMatrix,
};
use std::fmt::Debug;

#[test]
fn insert_test() {
    let mat = {
        let mut mat = init();
        mat.insert('b', "beta", 5);
        mat
    };

    {
        let entry = mat.peek().unwrap();
        check_bentry(entry, 'b', "beta", 5);
    }

    {
        let entry = mat.peek_from_row(&'a').unwrap();
        check_bentry(entry, 'a', "beta", 3);
    }

    {
        let entry = mat.peek_from_row(&'b').unwrap();
        check_bentry(entry, 'b', "beta", 5);
    }

    {
        let entry = mat.peek_from_column(&"alpha").unwrap();
        check_bentry(entry, 'b', "alpha", 2);
    }

    {
        let entry = mat.peek_from_column(&"beta").unwrap();
        check_bentry(entry, 'b', "beta", 5);
    }

    let mut mat = mat;

    {
        let entry = mat.pop().unwrap();
        check_oentry(&entry, 'b', "beta", 5);
    }

    {
        let entry = mat.pop().unwrap();
        check_oentry(&entry, 'a', "beta", 3);
    }
}

#[test]
fn peek_test() {
    let mat = init();

    {
        let entry = mat.peek().unwrap();
        check_bentry(entry, 'a', "beta", 3);
    }

    {
        let entry = mat.peek_from_row(&'a').unwrap();
        check_bentry(entry, 'a', "beta", 3);
    }

    {
        let entry = mat.peek_from_row(&'b').unwrap();
        check_bentry(entry, 'b', "alpha", 2);
    }

    {
        let entry = mat.peek_from_column(&"alpha").unwrap();
        check_bentry(entry, 'b', "alpha", 2);
    }

    {
        let entry = mat.peek_from_column(&"beta").unwrap();
        check_bentry(entry, 'a', "beta", 3);
    }
}

#[test]
fn pop_test() {
    let mut mat = init();

    {
        let entry = mat.pop().unwrap();
        check_oentry(&entry, 'a', "beta", 3);
    }

    {
        let entry = mat.pop().unwrap();
        check_oentry(&entry, 'b', "alpha", 2);
    }

    {
        let entry = mat.pop().unwrap();
        check_oentry(&entry, 'b', "beta", 1);
    }

    {
        let entry = mat.pop().unwrap();
        check_oentry(&entry, 'a', "alpha", 0);
    }

    assert!(mat.pop().is_none());
}

#[test]
fn pop_row_test() {
    let mut mat = init();

    {
        let entry = mat.pop_from_row(&'b').unwrap();
        check_oentry(&entry, 'b', "alpha", 2);
    }

    {
        let entry = mat.pop_from_row(&'a').unwrap();
        check_oentry(&entry, 'a', "beta", 3);
    }

    {
        let entry = mat.peek().unwrap();
        check_bentry(entry, 'b', "beta", 1);
    }

    {
        let entry = mat.peek_from_row(&'a').unwrap();
        check_bentry(entry, 'a', "alpha", 0);
    }

    {
        let entry = mat.peek_from_row(&'b').unwrap();
        check_bentry(entry, 'b', "beta", 1);
    }
}

#[test]
fn pop_column_test() {
    let mut mat = init();

    {
        let entry = mat.pop_from_column(&"alpha").unwrap();
        check_oentry(&entry, 'b', "alpha", 2);
    }

    {
        let entry = mat.pop_from_column(&"beta").unwrap();
        check_oentry(&entry, 'a', "beta", 3);
    }

    {
        let entry = mat.peek().unwrap();
        check_bentry(entry, 'b', "beta", 1);
    }

    {
        let entry = mat.peek_from_row(&'a').unwrap();
        check_bentry(entry, 'a', "alpha", 0);
    }

    {
        let entry = mat.peek_from_row(&'b').unwrap();
        check_bentry(entry, 'b', "beta", 1);
    }
}

fn check_bentry<R, C, W>(entry: BorrowedEntry<'_, R, C, W>, row: R, col: C, weight: W)
where
    R: Debug + Eq,
    C: Debug + Eq,
    W: Debug + Eq,
{
    assert_eq!(*entry.row, row);
    assert_eq!(*entry.column, col);
    assert_eq!(*entry.weight, weight);
}

fn check_oentry<R, C, W>(entry: &OwnedEntry<R, C, W>, row: R, col: C, weight: W)
where
    R: Debug + Eq,
    C: Debug + Eq,
    W: Debug + Eq,
{
    assert_eq!(entry.row, row);
    assert_eq!(entry.column, col);
    assert_eq!(entry.weight, weight);
}

fn init() -> PriorityMatrix<char, &'static str, i32> {
    [
        ('a', "alpha", 0),
        ('a', "beta", 3),
        ('b', "alpha", 2),
        ('b', "beta", 1),
    ]
    .into_iter()
    .collect()
}
