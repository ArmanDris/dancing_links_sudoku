### Task
Implement the cover operation iteratively for all the columns in a row. This
means tracing the right pointers of a specified row and column index until we
wrap around.

I solved this problem with 20 lines of code and 300 lines of testing. Both
are important to look at.

**Solution:**

Codex came up with the same 20 line solution I did, but it's code is just
clunky.

```rs
// My solution:
fn hide_all_columns_in_row(
    row_idx: usize,
    column_idx: usize,
    table: &mut LinkedTable,
) {
    let mut next_column_idx = column_idx;

    loop {
        cover_column(next_column_idx, table);
        next_column_idx = match table.table[row_idx][next_column_idx] {
            Link::Cell(c) => c.right.unwrap(),
            _ => panic!(),
        };

        if next_column_idx == column_idx {
            break;
        }
    }
}

```

```rs
// Codex's solution
fn hide_columns_in_row(row_idx: usize, column_idx: usize, table: &mut LinkedTable) {
    let mut next_column_idx = column_idx;

    loop {
        let right_column_idx = match table.table[row_idx][next_column_idx] {
            Link::EmptyLink => panic!("expected cell while hiding columns in row"),
            Link::ColumnHeader(_) => panic!("expected cell while hiding columns in row"),
            Link::Cell(cell) => cell
                .right
                .expect("cell should have a right pointer while hiding columns in row"),
        };

        cover_column(next_column_idx, table);

        if right_column_idx == column_idx {
            break;
        }

        next_column_idx = right_column_idx;
    }
}
```

Notably, right_column_idx is a completly uneccessary variable, and it is
defined before the `cover_column` operation even though we dont use it until
after the `cover_column` function. It would be way more readable if we did the
call to `cover_column` then had our code that updates `next_column_idx`,
that way the two things done in each loop iteration are clearly seperated.

**Tests:**

Without specific details codex did very reasonable job testing this. For the
limited instructions I gave I am actually impressed with the mock table it 
created.

I wanted an ASCII table though, my first few broad prompts produced bad tables,
 it took some specifics of how much information I wanted to get this nice 
looking table:

```
    // Row 1 selects columns 0 and 2. Column 1 is not part of that row.
    // Column headers show L/R and U/D pointers. Cells show only U/D pointers.
    //
    // Initial table:
    //
    //             col 0            col 1            col 2
    //          +------------+   +------------+   +------------+
    // row 0    | CH0 cc2    |<->| CH1 cc0    |<->| CH2 cc2    |
    //          | L:2 R:1    |   | L:0 R:2    |   | L:1 R:0    |
    //          | U:2 D:1    |   | U:0 D:0    |   | U:3 D:1    |
    //          +------------+   +------------+   +------------+
    //              ^   v           ^   v           ^   v
    //              |   |           |   |           |   |
    //          +------------+   +------------+   +------------+
    // row 1    | C1,0       |   | .          |   | C1,2       |
    //          | U:0 D:2    |   |            |   | U:0 D:3    |
    //          +------------+   +------------+   +------------+
    //              ^   v                           ^   v
    //              |   |                           |   |
    //          +------------+   +------------+   +------------+
    // row 2    | C2,0       |   | .          |   | .          |
    //          | U:1 D:0    |   |            |   |            |
    //          +------------+   +------------+   +------------+
    //              ^   v
    //              |   |
    //          +------------+   +------------+   +------------+
    // row 3    | .          |   | .          |   | C3,2       |
    //          |            |   |            |   | U:1 D:0    |
    //          +------------+   +------------+   +------------+
    //                                              ^   v
    //                                              |   |
    //
    // After hide_columns_in_row(1, 0):
    //
    //             col 0            col 1            col 2
    //          +------------+   +------------+   +------------+
    // row 0    | CH0 hidden |   | CH1 cc0    |   | CH2 hidden |
    //          | L:2 R:1    |   | L:1 R:1    |   | L:1 R:1    |
    //          | U:2 D:1    |   | U:0 D:0    |   | U:3 D:3    |
    //          +------------+   +------------+   +------------+
    //                              ^   v           ^   v
    //                              |   |           |   |
    //          +------------+   +------------+   +------------+
    // row 1    | C1,0       |   | .          |   | C1,2 hid   |
    //          | U:0 D:2    |   |            |   | U:0 D:3    |
    //          +------------+   +------------+   +------------+
    //              ^   v
    //              |   |
    //          +------------+   +------------+   +------------+
    // row 2    | C2,0       |   | .          |   | .          |
    //          | U:1 D:0    |   |            |   |            |
    //          +------------+   +------------+   +------------+
    //
    //          +------------+   +------------+   +------------+
    // row 3    | .          |   | .          |   | C3,2       |
    //          |            |   |            |   | U:0 D:0    |
    //          +------------+   +------------+   +------------+
    //                                              ^   v

```
