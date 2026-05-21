# Separator

Horizontal or vertical divider line.

## Installation

```bash
yei add separator
```

## Dependencies

None

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `vertical` | `bool` | `false` | Renders a vertical rule instead of horizontal |
| `class` | `Classes` | `""` | Extra CSS classes |

## Usage

```rust
use crate::components::separator::Separator;

html! {
    <div>
        <p>{"Above"}</p>
        <Separator />
        <p>{"Below"}</p>
    </div>
}
```

## Variants / Notes

Horizontal: `block w-full h-px bg-border`. Vertical: `inline-block self-stretch w-px bg-border` — use inside a flex row. The element carries `role="separator"` and `aria-orientation` for accessibility.
