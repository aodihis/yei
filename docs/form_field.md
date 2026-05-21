# FormField

Form field wrapper with label, input slot, and error message.

## Installation

```bash
yei add form_field
```

## Dependencies

- `label`

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | The input element(s) to wrap |
| `label` | `AttrValue` | `""` | Label text; hidden when empty |
| `description` | `AttrValue` | `""` | Helper text shown below the input |
| `error` | `AttrValue` | `""` | Validation error message shown in danger colour |
| `for` | `AttrValue` | `""` | `for` attribute on the rendered `<label>` |
| `horizontal` | `bool` | `false` | Side-by-side label and input layout |
| `class` | `Classes` | `""` | Extra CSS classes on the wrapper |
| `label_class` | `Classes` | `""` | Extra CSS classes on the label element |

## Usage

```rust
use crate::components::form_field::FormField;
use crate::components::input::Input;

html! {
    <FormField
        label="Email address"
        r#for="email"
        description="We'll never share your email."
        error={if email_invalid { "Please enter a valid email." } else { "" }}
    >
        <Input id="email" value={email.clone()} oninput={on_email_input} />
    </FormField>
}
```

## Variants / Notes

In horizontal layout (`horizontal={true}`), the label is fixed at `w-32` and the input area fills the remaining space. Error messages are rendered in `text-danger`; descriptions in `text-foreground/60`.
