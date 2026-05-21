# Dropdown

Accessible dropdown menu with keyboard navigation.

## Installation

```bash
yei add dropdown
```

## Dependencies

None

## Props

### `Dropdown`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | Trigger element (e.g. a `Button`) |
| `content` | `DropdownContent` | required | Menu panel body; wrap with `DropdownContent::new(html! { ... })` |
| `position` | `DropdownPosition` | `Bottom` | Where the menu opens relative to the trigger |
| `class` | `Classes` | `""` | Extra CSS classes on the wrapper |

### `DropdownPosition` values

`Bottom` · `Top` · `Left` · `Right`

### `DropdownItem`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | Item content |
| `disabled` | `bool` | `false` | Disables the item |
| `variant` | `DropdownItemVariant` | `Default` | `Default` or `Danger` |
| `class` | `Classes` | `""` | Extra CSS classes |
| `onclick` | `Callback<MouseEvent>` | no-op | Click handler |

### Additional sub-components

`DropdownLabel` — section heading (non-interactive).
`DropdownSeparator` — horizontal rule between sections.

## Usage

```rust
use crate::components::dropdown::{
    Dropdown, DropdownContent, DropdownItem, DropdownItemVariant, DropdownLabel, DropdownSeparator,
};
use crate::components::button::Button;

let menu = DropdownContent::new(html! {
    <>
        <DropdownLabel>{"Actions"}</DropdownLabel>
        <DropdownItem onclick={on_edit}>{"Edit"}</DropdownItem>
        <DropdownSeparator />
        <DropdownItem variant={DropdownItemVariant::Danger} onclick={on_delete}>{"Delete"}</DropdownItem>
    </>
});

html! {
    <Dropdown content={menu}>
        <Button>{"Options"}</Button>
    </Dropdown>
}
```

## Variants / Notes

The menu closes automatically when any `DropdownItem` is clicked (click propagation is stopped at the panel level). A transparent backdrop is placed behind the panel so that clicking outside also closes it.
