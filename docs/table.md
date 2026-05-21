# Table

Composable table primitives: Table, TableHeader, TableBody, TableRow, TableHead, TableCell.

## Installation

```bash
yei add table
```

## Dependencies

None

## Props

All primitives share the same prop shape:

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | Content |
| `class` | `Classes` | `""` | Extra CSS classes |

### Components

| Component | HTML element | Notes |
|-----------|-------------|-------|
| `Table` | `<table>` inside a scrollable `<div>` | Adds rounded border and overflow scroll |
| `TableHeader` | `<thead>` | Muted background, bottom border |
| `TableBody` | `<tbody>` | Row dividers via `divide-y` |
| `TableRow` | `<tr>` | Hover highlight |
| `TableHead` | `<th>` | Small caps, uppercase, muted colour |
| `TableCell` | `<td>` | Normal foreground text |

## Usage

```rust
use crate::components::table::{
    Table, TableHeader, TableBody, TableRow, TableHead, TableCell,
};

html! {
    <Table>
        <TableHeader>
            <TableRow>
                <TableHead>{"Name"}</TableHead>
                <TableHead>{"Email"}</TableHead>
                <TableHead>{"Role"}</TableHead>
            </TableRow>
        </TableHeader>
        <TableBody>
            <TableRow>
                <TableCell>{"Alice"}</TableCell>
                <TableCell>{"alice@example.com"}</TableCell>
                <TableCell>{"Admin"}</TableCell>
            </TableRow>
        </TableBody>
    </Table>
}
```

## Variants / Notes

`Table` wraps the `<table>` in a `div` with `overflow-auto` so it scrolls horizontally on small viewports. For a feature-rich table with sorting and pagination, use `data_table` which builds on these primitives.
