# Card

Composable card layout with header, body, and footer slots.

## Installation

```bash
yei add card
```

## Dependencies

None

## Props

All four sub-components share the same prop shape:

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | Slot content |
| `class` | `Classes` | `""` | Extra CSS classes |

### Components

| Component | Element | Notes |
|-----------|---------|-------|
| `Card` | `<div>` | Root wrapper with rounded border and shadow |
| `CardHeader` | `<div>` | Top padding area for title and description |
| `CardContent` | `<div>` | Main body area |
| `CardFooter` | `<div>` | Bottom row, flex with gap for action buttons |

## Usage

```rust
use crate::components::card::{Card, CardHeader, CardContent, CardFooter};
use crate::components::button::Button;

html! {
    <Card>
        <CardHeader>
            <h3 class="font-semibold text-lg">{"Account Settings"}</h3>
            <p class="text-sm text-foreground/60">{"Manage your profile details."}</p>
        </CardHeader>
        <CardContent>
            <p>{"Card body content goes here."}</p>
        </CardContent>
        <CardFooter>
            <Button>{"Save"}</Button>
        </CardFooter>
    </Card>
}
```
