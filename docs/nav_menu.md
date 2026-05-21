# NavMenu

Horizontal navigation bar with hover flyout submenus.

## Installation

```bash
yei add nav_menu
```

## Dependencies

- `icons`

## Props

### `NavMenu`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | `NavMenuItem` elements |
| `class` | `Classes` | `""` | Extra CSS classes |

### `NavMenuItem`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `title` | `AttrValue` | required | Display text |
| `description` | `Option<AttrValue>` | `None` | Optional subtitle shown in sub-level items |
| `href` | `AttrValue` | `"#"` | Link URL (used for leaf items) |
| `children` | `Children` | — | Nested `NavMenuItem` elements; enables flyout |
| `class` | `Classes` | `""` | Extra CSS classes |
| `onclick` | `Callback<MouseEvent>` | no-op | Click handler for leaf items |

### `NavMenuLabel`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | Label text |
| `class` | `Classes` | `""` | Extra CSS classes |

## Usage

```rust
use crate::components::nav_menu::{NavMenu, NavMenuItem, NavMenuLabel};

html! {
    <NavMenu>
        <NavMenuItem title="Home" href="/" />
        <NavMenuItem title="Products">
            <NavMenuLabel>{"Categories"}</NavMenuLabel>
            <NavMenuItem title="Widgets" description="Small reusable UI pieces" href="/products/widgets" />
            <NavMenuItem title="Templates" description="Pre-built page layouts" href="/products/templates" />
        </NavMenuItem>
        <NavMenuItem title="About" href="/about" />
    </NavMenu>
}
```

## Variants / Notes

Flyouts open on `mouseenter` and close on `mouseleave`. Top-level items with children render a downward-opening dropdown; nested items with children open a rightward flyout panel. Sub-level items support an optional `description` line rendered below the title.
