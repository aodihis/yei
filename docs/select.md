# Select

Styled select dropdown with icon indicator.

## Installation

```bash
yei add select
```

## Dependencies

- `icons`

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `options` | `Rc<Vec<SelectOption>>` | `[]` | List of options; each has `value: AttrValue` and `label: AttrValue` |
| `value` | `AttrValue` | `""` | Currently selected value |
| `placeholder` | `AttrValue` | `""` | Disabled placeholder option shown when no value is selected |
| `id` | `AttrValue` | `""` | HTML `id` for label association |
| `disabled` | `bool` | `false` | Disables the select |
| `class` | `Classes` | `""` | Extra CSS classes on the wrapper |
| `onchange` | `Callback<Event>` | no-op | Fires when the selection changes |

## Usage

```rust
use std::rc::Rc;
use crate::components::select::{Select, SelectOption};

let options = Rc::new(vec![
    SelectOption { value: "us".into(), label: "United States".into() },
    SelectOption { value: "gb".into(), label: "United Kingdom".into() },
    SelectOption { value: "ca".into(), label: "Canada".into() },
]);

html! {
    <Select
        id="country"
        options={options}
        value={selected_country.clone()}
        placeholder="Select a country"
        onchange={on_country_change}
    />
}
```

## Variants / Notes

A chevron icon (up/down) is overlaid on the right side and reflects whether the native select is focused. The underlying element is a native `<select>` for maximum accessibility and mobile compatibility.
