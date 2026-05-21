# Progress

Horizontal progress bar.

## Installation

```bash
yei add progress
```

## Dependencies

None

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `f64` | required | Current progress value |
| `max` | `f64` | `100.0` | Maximum value (used to calculate the fill percentage) |
| `class` | `Classes` | `""` | Extra CSS classes on the track |

## Usage

```rust
use crate::components::progress::Progress;

html! {
    <Progress value={75.0} max={100.0} />
}
```

## Variants / Notes

The fill percentage is clamped to `0–100%`. The component uses `role="progressbar"` with `aria-valuenow`, `aria-valuemin`, and `aria-valuemax` attributes. The fill width transitions smoothly (300 ms) when `value` changes.
