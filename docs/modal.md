# Modal

Accessible dialog with overlay and animated panel.

## Installation

```bash
yei add modal
```

## Dependencies

- `icons`

## Props

### `Modal`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | Dialog content |
| `open` | `bool` | required | Controls visibility |
| `show_close` | `bool` | `true` | Show the X close button |
| `close_on_backdrop` | `bool` | `true` | Close when clicking the overlay |
| `class` | `Classes` | `""` | Extra CSS classes on the panel |
| `on_close` | `Callback<()>` | required | Called when the modal should close |

### Sub-components

| Component | Description |
|-----------|-------------|
| `ModalHeader` | Top section with right padding for the close button |
| `ModalContent` | Body area with muted text sizing |
| `ModalFooter` | Bottom action row, right-aligned flex |

All sub-components accept `children` and `class`.

## Usage

```rust
use crate::components::modal::{Modal, ModalHeader, ModalContent, ModalFooter};
use crate::components::button::{Button, ButtonVariant};

html! {
    <Modal open={*show_modal} on_close={close_modal}>
        <ModalHeader>
            <h2 class="font-semibold text-lg">{"Confirm Delete"}</h2>
        </ModalHeader>
        <ModalContent>
            <p>{"Are you sure you want to delete this item? This action cannot be undone."}</p>
        </ModalContent>
        <ModalFooter>
            <Button variant={ButtonVariant::Outline} onclick={close_modal.reform(|_| ())}>{"Cancel"}</Button>
            <Button variant={ButtonVariant::Danger} onclick={on_confirm_delete}>{"Delete"}</Button>
        </ModalFooter>
    </Modal>
}
```
