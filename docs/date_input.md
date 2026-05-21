# DateInput

Date picker input backed by a Calendar popover.

## Installation

```bash
yei add date_input
```

## Dependencies

- `calendar`
- `icons`

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `selected` | `Option<AttrValue>` | `None` | Selected date in `YYYY-MM-DD` format |
| `placeholder` | `AttrValue` | `""` | Placeholder text shown when no date is selected (defaults to "Pick a date") |
| `id` | `AttrValue` | `""` | HTML `id` for the trigger button |
| `disabled` | `bool` | `false` | Prevents opening the calendar |
| `class` | `Classes` | `""` | Extra CSS classes |
| `error` | `bool` | `false` | Shows error styling on the trigger |
| `onselect` | `Callback<AttrValue>` | no-op | Fires with the chosen date string (`YYYY-MM-DD`) |

## Usage

```rust
use crate::components::date_input::DateInput;

html! {
    <DateInput
        id="dob"
        selected={date.clone()}
        placeholder="Select date of birth"
        onselect={on_date_change}
    />
}
```

## Variants / Notes

The selected date is displayed in human-readable format (e.g. "June 15, 2025") inside the trigger button. The Calendar popover closes automatically when a date is picked. Use `error={true}` to show danger-coloured borders when validation fails.
