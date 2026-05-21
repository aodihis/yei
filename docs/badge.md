# Badge

Small status and label indicators.

## Installation

```bash
yei add badge
```

## Dependencies

None

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | Badge text or content |
| `variant` | `BadgeVariant` | `Primary` | Visual style of the badge |
| `class` | `Classes` | `""` | Extra CSS classes |

### `BadgeVariant` values

`Primary` · `Secondary` · `Danger` · `Muted` · `Outline`

## Usage

```rust
use crate::components::badge::{Badge, BadgeVariant};

html! {
    <>
        <Badge>{"New"}</Badge>
        <Badge variant={BadgeVariant::Danger}>{"Error"}</Badge>
        <Badge variant={BadgeVariant::Outline}>{"Draft"}</Badge>
    </>
}
```
