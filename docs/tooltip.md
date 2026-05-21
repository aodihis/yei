# Tooltip

Hover tooltip anchored to any element.

## Installation

```bash
yei add tooltip
```

## Dependencies

None

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | The element the tooltip is anchored to |
| `tip` | `AttrValue` | required | Tooltip text |
| `position` | `TooltipPosition` | `Top` | Where the tooltip appears |
| `class` | `Classes` | `""` | Extra CSS classes on the wrapper span |

### `TooltipPosition` values

`Top` · `Bottom` · `Left` · `Right`

## Usage

```rust
use crate::components::tooltip::{Tooltip, TooltipPosition};
use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::icons::icon_info;

html! {
    <Tooltip tip="This field is required" position={TooltipPosition::Top}>
        <span class="text-foreground/50">{ icon_info() }</span>
    </Tooltip>
}
```

## Variants / Notes

The tooltip is shown via CSS `group-hover:opacity-100` — no JavaScript or state is needed. The wrapper `<span>` gets the `group` class; the tooltip text gets `opacity-0 group-hover:opacity-100`. Requires Tailwind's `group` and `group-hover` utilities to be available.
