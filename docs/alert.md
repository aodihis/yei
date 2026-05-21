# Alert

Contextual feedback messages with variant styles.

## Installation

```bash
yei add alert
```

## Dependencies

None

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | Content rendered inside the alert |
| `variant` | `AlertVariant` | `Primary` | Visual style of the alert |
| `class` | `Classes` | `""` | Extra CSS classes |

### `AlertVariant` values

`Primary` · `Secondary` · `Danger` · `Muted`

## Usage

```rust
use crate::components::alert::{Alert, AlertVariant};

html! {
    <Alert variant={AlertVariant::Danger}>
        {"Your session has expired. Please log in again."}
    </Alert>
}
```
