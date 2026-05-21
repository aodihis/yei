# Carousel

Animated slide carousel with previous/next navigation.

## Installation

```bash
yei add carousel
```

## Dependencies

- `icons`

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `slides` | `Rc<Vec<CarouselSlide>>` | required | Slides to display; create each with `CarouselSlide::new(html! { ... })` |
| `show_controls` | `bool` | `true` | Show previous/next arrow buttons |
| `show_indicators` | `bool` | `true` | Show dot indicators at the bottom |
| `autoplay` | `bool` | `false` | Automatically advance slides |
| `interval_ms` | `u16` | `3000` | Milliseconds between auto-advances when `autoplay` is `true` |
| `class` | `Classes` | `""` | Extra CSS classes on the root element |

## Usage

```rust
use std::rc::Rc;
use crate::components::carousel::{Carousel, CarouselSlide};

let slides = Rc::new(vec![
    CarouselSlide::new(html! { <img src="slide1.jpg" class="w-full h-64 object-cover" /> }),
    CarouselSlide::new(html! { <img src="slide2.jpg" class="w-full h-64 object-cover" /> }),
    CarouselSlide::new(html! { <img src="slide3.jpg" class="w-full h-64 object-cover" /> }),
]);

html! {
    <Carousel slides={slides} autoplay={true} interval_ms={4000} />
}
```

## Variants / Notes

Slides enter from the right when advancing forward and from the left when going back. The CSS animation classes `yei-carousel-enter-right` and `yei-carousel-enter-left` must be present in your stylesheet (provided by `yei.css`). Clicking a dot indicator jumps directly to that slide and resets the autoplay timer.
