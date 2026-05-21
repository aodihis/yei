# Slider

Controlled range input with styled track fill.

## Installation

```bash
yei add slider
```

## Dependencies

None

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `f64` | required | Current slider value |
| `min` | `f64` | `0.0` | Minimum value |
| `max` | `f64` | `100.0` | Maximum value |
| `step` | `f64` | `1.0` | Step increment |
| `disabled` | `bool` | `false` | Disables the slider |
| `id` | `AttrValue` | `""` | HTML `id` |
| `class` | `Classes` | `""` | Extra CSS classes on the wrapper |
| `onchange` | `Callback<f64>` | no-op | Fires with the new `f64` value on every input event |

## Usage

```rust
use crate::components::slider::Slider;

html! {
    <div class="space-y-2">
        <label>{ format!("Volume: {}", *volume) }</label>
        <Slider
            value={*volume}
            min={0.0}
            max={100.0}
            step={5.0}
            onchange={on_volume_change}
        />
    </div>
}
```

## Variants / Notes

The track fill is implemented via the CSS custom property `--yei-slider-fill`, set inline as a percentage. The `yei-slider` class in `yei.css` uses this property to render the filled portion. ARIA attributes (`aria-valuemin`, `aria-valuemax`, `aria-valuenow`) are set automatically.
