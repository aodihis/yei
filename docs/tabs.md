# Tabs

Tab bar with panel switching.

## Installation

```bash
yei add tabs
```

## Dependencies

None

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `items` | `Rc<Vec<TabItem>>` | required | Tab definitions; create with `TabItem::new("Label", html! { ... })` |
| `class` | `Classes` | `""` | Extra CSS classes on the root element |

## Usage

```rust
use std::rc::Rc;
use crate::components::tabs::{Tabs, TabItem};

let items = Rc::new(vec![
    TabItem::new("Overview", html! { <p>{"Overview content"}</p> }),
    TabItem::new("Analytics", html! { <p>{"Analytics content"}</p> }),
    TabItem::new("Settings", html! { <p>{"Settings content"}</p> }),
]);

html! {
    <Tabs items={items} />
}
```

## Variants / Notes

The active tab is underlined with `border-primary`. Inactive tabs show a transparent bottom border and transition on hover. The active panel is rendered with `role="tabpanel"`. Tabs manage active state internally; the first tab is active by default. If `items` shrinks and the active index is out of range, it is clamped to the last available tab.
