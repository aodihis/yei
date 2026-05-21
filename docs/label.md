# Label

Accessible form label.

## Installation

```bash
yei add label
```

## Dependencies

None

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | Label text |
| `for` | `AttrValue` | `""` | ID of the associated input element |
| `class` | `Classes` | `""` | Extra CSS classes |

## Usage

```rust
use crate::components::label::Label;
use crate::components::input::Input;

html! {
    <div class="space-y-1">
        <Label r#for="username">{"Username"}</Label>
        <Input id="username" value={username.clone()} oninput={on_input} />
    </div>
}
```

## Variants / Notes

`Label` is a thin wrapper around `<label>` with `text-sm font-medium leading-none` styling. It also applies `peer-disabled` opacity/cursor utilities so it automatically dims when a sibling input is disabled (requires Tailwind peer utilities).
