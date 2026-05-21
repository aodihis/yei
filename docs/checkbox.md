# Checkbox

Controlled checkbox input.

## Installation

```bash
yei add checkbox
```

## Dependencies

None

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `checked` | `bool` | `false` | Whether the checkbox is checked |
| `disabled` | `bool` | `false` | Disables the input |
| `id` | `AttrValue` | `""` | HTML `id` for label association |
| `name` | `AttrValue` | `""` | HTML `name` for form submission |
| `value` | `AttrValue` | `""` | HTML `value` attribute |
| `class` | `Classes` | `""` | Extra CSS classes |
| `onchange` | `Callback<Event>` | no-op | Fires when the checked state changes |

## Usage

```rust
use crate::components::checkbox::Checkbox;

html! {
    <div class="flex items-center gap-2">
        <Checkbox
            id="agree"
            checked={*agreed}
            onchange={on_agree_change}
        />
        <label for="agree">{"I agree to the terms"}</label>
    </div>
}
```
