# Nav

Vertical accordion navigation with nested items.

## Installation

```bash
yei add nav
```

## Dependencies

- `icons`

## Props

### `Nav`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | `NavItem` or `NavLabel` elements |
| `class` | `Classes` | `""` | Extra CSS classes |

### `NavItem`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `title` | `AttrValue` | required | Display text for the item |
| `href` | `AttrValue` | `"#"` | Link URL (used only for leaf items) |
| `children` | `Children` | — | Nested `NavItem` elements; presence triggers accordion behaviour |
| `class` | `Classes` | `""` | Extra CSS classes |
| `onclick` | `Callback<MouseEvent>` | no-op | Click handler for leaf items |

### `NavLabel`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | Label text |
| `class` | `Classes` | `""` | Extra CSS classes |

## Usage

```rust
use crate::components::nav::{Nav, NavItem, NavLabel};

html! {
    <Nav>
        <NavLabel>{"Main"}</NavLabel>
        <NavItem title="Dashboard" href="/dashboard" />
        <NavItem title="Settings">
            <NavItem title="Profile" href="/settings/profile" />
            <NavItem title="Security" href="/settings/security" />
        </NavItem>
    </Nav>
}
```

## Variants / Notes

A `NavItem` with children renders as an accordion trigger (button) that toggles its sub-list. A `NavItem` without children renders as a plain `<a>` link. Nesting can be arbitrarily deep. `NavLabel` renders a non-interactive section heading.
