# Pagination

Page navigation with previous, next, and numbered page items.

## Installation

```bash
yei add pagination
```

## Dependencies

None

## Props

### `Pagination`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | `PaginationItem` and `PaginationEllipsis` elements |
| `class` | `Classes` | `""` | Extra CSS classes |

### `PaginationItem`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | Page number or icon |
| `active` | `bool` | `false` | Highlights this item as the current page |
| `disabled` | `bool` | `false` | Dims and prevents interaction |
| `class` | `Classes` | `""` | Extra CSS classes |
| `onclick` | `Callback<MouseEvent>` | no-op | Click handler |

### `PaginationEllipsis`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `Classes` | `""` | Extra CSS classes |

Renders a `…` separator between non-consecutive page numbers.

## Usage

```rust
use crate::components::pagination::{Pagination, PaginationItem, PaginationEllipsis};
use crate::components::icons::{icon_chevron_left, icon_chevron_right};

html! {
    <Pagination>
        <PaginationItem disabled={page == 0} onclick={go_prev}>
            { icon_chevron_left() }
        </PaginationItem>
        <PaginationItem active={page == 0} onclick={go_to_0}>{"1"}</PaginationItem>
        <PaginationItem active={page == 1} onclick={go_to_1}>{"2"}</PaginationItem>
        <PaginationEllipsis />
        <PaginationItem active={page == 9} onclick={go_to_9}>{"10"}</PaginationItem>
        <PaginationItem disabled={page == 9} onclick={go_next}>
            { icon_chevron_right() }
        </PaginationItem>
    </Pagination>
}
```

## Variants / Notes

`Pagination` is a set of composable primitives — page range logic (ellipsis placement, visible pages) is the caller's responsibility. `JsonDataTable` uses these primitives internally with automatic ellipsis generation.
