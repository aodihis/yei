# FileUpload

File input as a drag-and-drop zone or button.

## Installation

```bash
yei add file_upload
```

## Dependencies

- `icons`

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `multiple` | `bool` | `false` | Allow selecting multiple files |
| `accept` | `AttrValue` | `""` | MIME types or extensions to accept (e.g. `"image/*"`, `".pdf,.docx"`) |
| `disabled` | `bool` | `false` | Disables the upload control |
| `variant` | `FileUploadVariant` | `DropZone` | Visual style |
| `class` | `Classes` | `""` | Extra CSS classes |
| `onchange` | `Callback<Vec<File>>` | no-op | Fires with the selected `web_sys::File` objects |

### `FileUploadVariant` values

`DropZone` · `Button`

## Usage

```rust
use crate::components::file_upload::{FileUpload, FileUploadVariant};

html! {
    <FileUpload
        multiple={true}
        accept="image/*"
        variant={FileUploadVariant::DropZone}
        onchange={on_files_selected}
    />
}
```

## Variants / Notes

`DropZone` renders a full drag-and-drop area that highlights on hover and changes colour when files are dragged over it. `Button` renders a compact inline button that opens the OS file picker on click. Both variants trigger the same `onchange` callback with a `Vec<web_sys::File>`.
