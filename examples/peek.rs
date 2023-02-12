use priority_matrix::PriorityMatrix;

fn main() {
    let matrix: PriorityMatrix<char, &str, i32> = [
        ('a', "alpha", 0),
        ('a', "beta", 3),
        ('b', "alpha", 2),
        ('b', "beta", 1),
    ]
    .into_iter()
    .collect();

    // Get the maximum entry
    let entry = matrix.peek().unwrap();
    assert_eq!(entry.row, &'a');
    assert_eq!(entry.column, &"beta");
    assert_eq!(entry.weight, &3);

    // Get the maximum entry in a row
    let entry = matrix.peek_from_row(&'b').unwrap();
    assert_eq!(entry.row, &'b');
    assert_eq!(entry.column, &"alpha");
    assert_eq!(entry.weight, &2);

    // Get the maximum entry in a column
    let entry = matrix.peek_from_column(&"alpha").unwrap();
    assert_eq!(entry.row, &'b');
    assert_eq!(entry.column, &"alpha");
    assert_eq!(entry.weight, &2);
}
