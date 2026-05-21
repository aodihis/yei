# Radio

Controlled radio button group.

## Installation

```bash
yei add radio
```

## Dependencies

None

## Props

### `Radio`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `checked` | `bool` | `false` | Whether this radio is selected |
| `disabled` | `bool` | `false` | Disables the input |
| `name` | `AttrValue` | `""` | Group name — must be the same for all radios in a group |
| `value` | `AttrValue` | `""` | Value submitted with the form |
| `id` | `AttrValue` | `""` | HTML `id` for label association |
| `class` | `Classes` | `""` | Extra CSS classes |
| `onchange` | `Callback<Event>` | no-op | Fires when this radio is selected |

### `RadioGroup`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | `Radio` elements with associated labels |
| `name` | `AttrValue` | `""` | Shared name for the group (informational only — set `name` on each `Radio`) |
| `class` | `Classes` | `""` | Extra CSS classes |

## Usage

```rust
use crate::components::radio::{Radio, RadioGroup};

html! {
    <RadioGroup>
        <div class="flex items-center gap-2">
            <Radio id="plan-free" name="plan" value="free" checked={*plan == "free"} onchange={on_change.clone()} />
            <label for="plan-free">{"Free"}</label>
        </div>
        <div class="flex items-center gap-2">
            <Radio id="plan-pro" name="plan" value="pro" checked={*plan == "pro"} onchange={on_change.clone()} />
            <label for="plan-pro">{"Pro"}</label>
        </div>
    </RadioGroup>
}
```
