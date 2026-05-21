# DataTable

Data-driven table with sortable columns and optional pagination.

## Installation

```bash
yei add data_table
```

## Dependencies

- `table`
- `pagination`
- `icons`

## Props

### `DataTable` — positional rows

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `columns` | `Rc<Vec<TableCol>>` | required | Column definitions |
| `rows` | `Rc<Vec<Vec<AttrValue>>>` | required | Rows as ordered lists of cell values |
| `class` | `Classes` | `""` | Extra CSS classes |

### `JsonDataTable` — key-value rows with pagination

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `columns` | `Rc<Vec<ColumnDef>>` | required | Column definitions (key + label) |
| `rows` | `Rc<Vec<DataRow>>` | required | Rows as `Vec<(AttrValue, AttrValue)>` key-value pairs |
| `page_size` | `usize` | `0` | Rows per page; `0` disables pagination |
| `class` | `Classes` | `""` | Extra CSS classes |

### Column definition helpers

`TableCol::new("Label")` — non-sortable column.
`TableCol::new("Label").sortable()` — enables click-to-sort on that column.

`ColumnDef::new("key", "Label")` / `.sortable()` — same pattern for `JsonDataTable`.

## Usage

```rust
use std::rc::Rc;
use crate::components::data_table::{JsonDataTable, ColumnDef, DataRow};

let columns = Rc::new(vec![
    ColumnDef::new("name", "Name").sortable(),
    ColumnDef::new("role", "Role"),
    ColumnDef::new("joined", "Joined").sortable(),
]);

let rows: Rc<Vec<DataRow>> = Rc::new(vec![
    vec![
        ("name".into(), "Alice".into()),
        ("role".into(), "Admin".into()),
        ("joined".into(), "2024-01-15".into()),
    ],
    vec![
        ("name".into(), "Bob".into()),
        ("role".into(), "Editor".into()),
        ("joined".into(), "2024-03-22".into()),
    ],
]);

html! {
    <JsonDataTable columns={columns} rows={rows} page_size={10} />
}
```

## Variants / Notes

`DataTable` uses positional rows (`Vec<AttrValue>`); column order must match row order. `JsonDataTable` matches cells by key, so missing keys render as empty cells and extra keys are ignored. Both tables manage sort state internally; clicking a sorted column header toggles ascending/descending order.
