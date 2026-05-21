# Toast

Ephemeral notification toasts with auto-dismiss.

## Installation

```bash
yei add toast
```

## Dependencies

- `icons`

## Props

### `ToastProvider`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | App content that needs access to `use_toast()` |

Wrap your app (or the relevant subtree) in `ToastProvider`. It renders a `ToastContainer` in the top-right corner automatically.

### `ToastItem` fields (passed to `push`)

| Field | Type | Description |
|-------|------|-------------|
| `id` | `u32` | Auto-assigned by the provider |
| `title` | `AttrValue` | Primary message text |
| `description` | `Option<AttrValue>` | Optional subtitle |
| `variant` | `ToastVariant` | Visual style |
| `duration_ms` | `u32` | Milliseconds before auto-dismiss |

### `ToastVariant` values

`Default` · `Success` · `Warning` · `Danger`

## Usage

```rust
use crate::components::toast::{ToastProvider, ToastItem, ToastVariant, use_toast};

// Wrap your app:
html! {
    <ToastProvider>
        <App />
    </ToastProvider>
}

// Inside any descendant component:
#[function_component(App)]
pub fn app() -> Html {
    let toast = use_toast();

    let show_success = {
        let toast = toast.clone();
        Callback::from(move |_: MouseEvent| {
            toast.push.emit(ToastItem {
                id: 0,
                title: "Saved!".into(),
                description: Some("Your changes have been saved.".into()),
                variant: ToastVariant::Success,
                duration_ms: 4000,
            });
        })
    };

    html! {
        <button onclick={show_success}>{"Save"}</button>
    }
}
```

## Variants / Notes

`use_toast()` is a hook that returns a `ToastContext` with a `push` callback. Set `duration_ms` to control how long each toast stays visible. The dismiss X button is always shown and calls the same dismiss callback as the timer.
