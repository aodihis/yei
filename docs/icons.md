# Icons

SVG icon set based on [Lucide](https://lucide.dev) — used by many components and installed automatically as a dependency.

## Installation

```bash
yei add icons
```

## Dependencies

None

## Props

Icons are plain functions, not components, so they have no props. Each function returns an `Html` SVG element sized 24×24.

## Usage

```rust
use crate::components::icons::{icon_check, icon_x, icon_chevron_down, icon_search};

html! {
    <div class="flex items-center gap-2">
        { icon_check() }
        { icon_x() }
        { icon_chevron_down() }
        { icon_search() }
    </div>
}
```

## Variants / Notes

Icons are Lucide-compatible SVGs rendered with `stroke="currentColor"`, so they inherit the surrounding text colour. Size can be overridden with a wrapping `<span>` and Tailwind sizing classes.

The icon set is large — it contains hundreds of icons (e.g. `icon_activity`, `icon_accessibility`, `icon_alert_circle`, etc.). Browse `icons.rs` for the full list. All functions follow the naming pattern `icon_{lucide_name_snake_case}`.

This component is installed automatically when any other component depends on it (e.g. `accordion`, `calendar`, `carousel`).
