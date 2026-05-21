# Drawer

Slide-in panel from any edge of the screen.

## Installation

```bash
yei add drawer
```

## Dependencies

- `icons`

## Props

### `Drawer`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | Panel content |
| `open` | `bool` | required | Controls visibility |
| `show_close` | `bool` | `true` | Show the X close button |
| `close_on_backdrop` | `bool` | `true` | Close when clicking the backdrop |
| `side` | `DrawerSide` | `Right` | Edge the panel slides in from |
| `class` | `Classes` | `""` | Extra CSS classes on the panel |
| `on_close` | `Callback<()>` | required | Called when the drawer should close |

### `DrawerSide` values

`Right` · `Left` · `Top` · `Bottom`

### Sub-components

| Component | Description |
|-----------|-------------|
| `DrawerHeader` | Top section with extra right padding for the close button |
| `DrawerContent` | Scrollable body area |
| `DrawerFooter` | Bottom action row with border-top |

All sub-components accept `children` and `class`.

## Usage

```rust
use crate::components::drawer::{Drawer, DrawerHeader, DrawerContent, DrawerFooter, DrawerSide};
use crate::components::button::Button;

html! {
    <Drawer open={*open} on_close={on_close} side={DrawerSide::Right}>
        <DrawerHeader>
            <h2 class="font-semibold text-lg">{"Settings"}</h2>
        </DrawerHeader>
        <DrawerContent>
            <p>{"Drawer body content."}</p>
        </DrawerContent>
        <DrawerFooter>
            <Button onclick={on_close.reform(|_| ())}>{"Close"}</Button>
        </DrawerFooter>
    </Drawer>
}
```
