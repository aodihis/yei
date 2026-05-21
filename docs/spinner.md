# Spinner

Animated loading spinner.

## Installation

```bash
yei add spinner
```

## Dependencies

None

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `size` | `SpinnerSize` | `Default` | Size of the spinner SVG |
| `class` | `Classes` | `""` | Extra CSS classes |

### `SpinnerSize` values

| Variant | Size |
|---------|------|
| `Sm` | 16×16 px (`size-4`) |
| `Default` | 24×24 px (`size-6`) |
| `Lg` | 32×32 px (`size-8`) |

## Usage

```rust
use crate::components::spinner::{Spinner, SpinnerSize};

html! {
    <div class="flex items-center gap-2">
        <Spinner />
        <span>{"Loading..."}</span>
    </div>
}
```

## Variants / Notes

The spinner is an inline SVG with `role="status"` and `aria-label="Loading"`. It inherits colour from `currentColor`, so wrapping it in a coloured container (e.g. `text-primary`) changes the spinner colour.
