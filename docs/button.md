# Button

Accessible button with variant and size options.

## Installation

```bash
yei add button
```

## Dependencies

None

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | Button label or content |
| `href` | `Option<AttrValue>` | `None` | Renders an `<a>` tag instead of `<button>` when set |
| `disabled` | `bool` | `false` | Disables the button |
| `type` | `AttrValue` | `"button"` | HTML `type` attribute (`"button"`, `"submit"`, `"reset"`) |
| `variant` | `ButtonVariant` | `Primary` | Visual style |
| `size` | `ButtonSize` | `Default` | Size preset |
| `class` | `Classes` | `""` | Extra CSS classes |
| `onclick` | `Callback<MouseEvent>` | no-op | Click handler |

### `ButtonVariant` values

`Primary` · `Secondary` · `Danger` · `Muted` · `Outline` · `Link`

### `ButtonSize` values

`Default` (h-10) · `Sm` (h-9) · `Lg` (h-11) · `Icon` (10×10 square)

## Usage

```rust
use crate::components::button::{Button, ButtonVariant, ButtonSize};

html! {
    <>
        <Button onclick={on_save}>{"Save"}</Button>
        <Button variant={ButtonVariant::Outline} size={ButtonSize::Sm}>{"Cancel"}</Button>
        <Button variant={ButtonVariant::Danger} disabled={true}>{"Delete"}</Button>
        <Button href="https://example.com" variant={ButtonVariant::Link}>{"Learn more"}</Button>
    </>
}
```

## Variants / Notes

When `href` is `Some`, the button renders as an `<a>` element. If `disabled` is also `true` alongside `href`, the link is rendered with `aria-disabled="true"` and `pointer-events-none` rather than a real `disabled` attribute (which is not valid on `<a>`).
