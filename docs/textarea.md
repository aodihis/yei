# Textarea

Styled multi-line text input.

## Installation

```bash
yei add textarea
```

## Dependencies

None

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `AttrValue` | `""` | Controlled textarea value |
| `placeholder` | `AttrValue` | `""` | Placeholder text |
| `id` | `AttrValue` | `""` | HTML `id` for label association |
| `disabled` | `bool` | `false` | Disables the textarea |
| `rows` | `u32` | `4` | Number of visible text rows |
| `class` | `Classes` | `""` | Extra CSS classes |
| `error` | `bool` | `false` | Shows danger-coloured border |
| `oninput` | `Callback<InputEvent>` | no-op | Fires on every keystroke |
| `onchange` | `Callback<Event>` | no-op | Fires on blur |

## Usage

```rust
use crate::components::textarea::Textarea;

html! {
    <Textarea
        id="bio"
        value={bio.clone()}
        placeholder="Tell us about yourself..."
        rows={6}
        error={bio_too_long}
        oninput={on_bio_input}
    />
}
```

## Variants / Notes

The textarea is vertically resizable (`resize-y`). Error state adds a danger-coloured border. Styling mirrors the `Input` component for visual consistency in forms.
