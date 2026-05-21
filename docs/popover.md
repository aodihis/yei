# Popover

Floating content panel anchored to a trigger element.

## Installation

```bash
yei add popover
```

## Dependencies

None

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | Trigger element |
| `content` | `PopoverContent` | required | Panel body; wrap with `PopoverContent::new(html! { ... })` |
| `position` | `PopoverPosition` | `Bottom` | Where the panel opens relative to the trigger |
| `class` | `Classes` | `""` | Extra CSS classes on the wrapper |

### `PopoverPosition` values

`Bottom` · `Top` · `Left` · `Right`

## Usage

```rust
use crate::components::popover::{Popover, PopoverContent, PopoverPosition};
use crate::components::button::Button;

let panel = PopoverContent::new(html! {
    <div class="space-y-2">
        <p class="font-medium text-sm">{"More info"}</p>
        <p class="text-xs text-foreground/60">{"This field accepts ISO 8601 date strings."}</p>
    </div>
});

html! {
    <Popover content={panel} position={PopoverPosition::Bottom}>
        <Button variant={ButtonVariant::Outline}>{"?"}</Button>
    </Popover>
}
```

## Variants / Notes

Clicking the trigger toggles the panel. Clicking outside (on the transparent backdrop) closes it. The panel renders with `role="dialog"`. Unlike `Dropdown`, `Popover` does not close when its content is clicked — it is designed for informational or interactive content, not menu items.
