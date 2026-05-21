# Calendar

Interactive month calendar with day selection.

## Installation

```bash
yei add calendar
```

## Dependencies

- `icons`

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `selected` | `Option<AttrValue>` | `None` | Currently selected date in `YYYY-MM-DD` format |
| `class` | `Classes` | `""` | Extra CSS classes |
| `onselect` | `Callback<AttrValue>` | no-op | Fires with the chosen date string (`YYYY-MM-DD`) |

## Usage

```rust
use crate::components::calendar::Calendar;

// In a function component or struct component that holds `selected: Option<String>`:
html! {
    <Calendar
        selected={Some(AttrValue::from("2025-06-15"))}
        onselect={on_date_selected}
    />
}
```

## Variants / Notes

The calendar supports three drill-down views: **Days** (default), **Months**, and **Years**. Clicking the month name in the header switches to month picker; clicking the year switches to a 12-year grid. Navigation arrows change the displayed period in the active view.

Dates are passed and emitted as ISO `YYYY-MM-DD` strings.
