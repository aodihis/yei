# Input

Styled text input with type and error state options.

## Installation

```bash
yei add input
```

## Dependencies

None

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `AttrValue` | `""` | Controlled input value |
| `placeholder` | `AttrValue` | `""` | Placeholder text |
| `id` | `AttrValue` | `""` | HTML `id` for label association |
| `disabled` | `bool` | `false` | Disables the input |
| `type` | `InputType` | `Text` | Input type |
| `class` | `Classes` | `""` | Extra CSS classes |
| `error` | `bool` | `false` | Shows danger-coloured border |
| `oninput` | `Callback<InputEvent>` | no-op | Fires on every keystroke |
| `onchange` | `Callback<Event>` | no-op | Fires on blur / Enter |
| `onfocus` | `Callback<FocusEvent>` | no-op | Fires when the input gains focus |
| `onblur` | `Callback<FocusEvent>` | no-op | Fires when the input loses focus |

### `InputType` values

`Text` · `Email` · `Password` · `Number` · `Tel` · `Url`

## Usage

```rust
use crate::components::input::{Input, InputType};

html! {
    <Input
        id="email"
        r#type={InputType::Email}
        value={email.clone()}
        placeholder="you@example.com"
        error={!email_valid}
        oninput={on_email_input}
    />
}
```
