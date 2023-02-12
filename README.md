# priority-matrix

The Rust crate implements the matrix data structure that supports
per-row, per-column and whole-matrix maximum key queries.

## Example

The code below is an example to query the key with the maximum weight
either in the matrix, in a row or in a column. The complete example
can be found in [peek.rs](examples/peek.rs).

```rust
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
assert_eq!(entry.weight, &3);

// Get the maximum entry in a row
let entry = matrix.peek_from_row(&'b').unwrap();
assert_eq!(entry.column, &"alpha");

// Get the maximum entry in a column
let entry = matrix.peek_from_column(&"alpha").unwrap();
assert_eq!(entry.row, &'b');
```

## License

This project is distributed under MIT license. Please read the
[license file](LICENSE.txt).
