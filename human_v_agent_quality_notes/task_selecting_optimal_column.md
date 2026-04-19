### The task:
Add an option `optimal` to the `select_column` function that selects the column
 with the least amount of cells.


Agent added a derive statement we did not need to the `DecisionStrategy` enum.
```rs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
```

Agent's first implementation selected the minimum column in two passes of the
array. It had long chains of functions as well, so it was hard to read and
slow. After asking to do it in one pass, it produced code that was faster and
simpler than mine.

```rs
/// My code
        DecisionStrategy::Optimal => {
            let mut min_columns = vec![potential_columns[0]];
            let first_cell_count = match table.table[0][potential_columns[0]] {
                Link::ColumnHeader(ch) => ch.cell_count,
                _ => panic!(),
            };
            let mut min_cell_count = first_cell_count;

            for (pot_idx, col_num) in potential_columns.iter().enumerate() {
                let cell_count = match table.table[0][*col_num] {
                    Link::ColumnHeader(ch) => ch.cell_count,
                    _ => panic!(),
                };

                if cell_count < min_cell_count {
                    min_cell_count = cell_count;
                    min_columns = vec![pot_idx];
                }
                if cell_count == min_cell_count {
                    min_columns.push(pot_idx);
                }
            }

            let rand_idx = rand::thread_rng().gen_range(0..min_columns.len());
            let selected_pot_idx = min_columns[rand_idx];
            let selected_col = potential_columns.remove(selected_pot_idx);
            (selected_col, potential_columns)
        }
```

```rs
/// Codex's code
        DecisionStrategy::Optimal => {
            let mut rng = rand::thread_rng();
            let mut min_count = i32::MAX;
            let mut min_positions = Vec::new();
            for (i, &col) in potential_columns.iter().enumerate() {
                if let Link::ColumnHeader(ch) = table.table[0][col] {
                    let count = ch.cell_count;
                    if count < min_count {
                        min_count = count;
                        min_positions.clear();
                        min_positions.push(i);
                    } else if count == min_count {
                        min_positions.push(i);
                    }
                }
            }
            if min_positions.is_empty() {
                panic!("No columns to select");
            }
            let pick = rng.gen_range(0..min_positions.len());
            let pos = min_positions[pick];
            let selected = potential_columns.remove(pos);
            (selected, potential_columns)
        }
```

The tests were a little less concise and shallower. My single test covered the
entire optimal path, while Codex used two tests. Does not really matter though.

## Verdicts
- Small differences in prompt and luck can effect readability a lot
- With a nudge models can improve their own code without specific details


