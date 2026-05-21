# Accordion

Collapsible content sections with animated open/close.

## Installation

```bash
yei add accordion
```

## Dependencies

- `icons`

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `items` | `Rc<Vec<AccordionItem>>` | `[]` | List of accordion items, each with `title: AttrValue` and `content: AttrValue` |
| `multiple` | `bool` | `false` | Allow multiple sections open at once |
| `class` | `Classes` | `""` | Extra CSS classes for the root element |

## Usage

```rust
use std::rc::Rc;
use crate::components::accordion::{Accordion, AccordionItem};

let items = Rc::new(vec![
    AccordionItem { title: "Section 1".into(), content: "Content for section 1.".into() },
    AccordionItem { title: "Section 2".into(), content: "Content for section 2.".into() },
]);

html! {
    <Accordion items={items} multiple={false} />
}
```

## Variants / Notes

When `multiple` is `false` (default), opening one section automatically closes all others. When `multiple` is `true`, any number of sections may be open simultaneously.

`AccordionItem` is a plain struct — construct it directly:

```rust
AccordionItem {
    title: AttrValue::from("FAQ"),
    content: AttrValue::from("Answer text here."),
}
```
