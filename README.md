# yei registry

The component registry for [yei](https://github.com/aodihis/yei-monorepo). Contains source files for all components, the CSS design system, and versioned release snapshots.

## Structure

```
registry/
├── components/         component .rs source files
├── versions/           versioned snapshots (created automatically on GitHub release)
│   └── 0.1.0/
│       ├── components/
│       ├── registry.json
│       └── yei.css
├── registry.json       component manifest
└── yei.css             design tokens + Tailwind theme
```

## Components

| Name | Description |
|---|---|
| `accordion` | Collapsible content sections with animated open/close |
| `alert` | Contextual feedback messages with variant styles |
| `badge` | Small status and label indicators |
| `button` | Accessible button with variant and size options |
| `calendar` | Interactive month calendar with day selection |
| `card` | Composable card layout with header, body, and footer slots |
| `carousel` | Animated slide carousel with previous/next navigation |
| `checkbox` | Controlled checkbox input |
| `data_table` | Data-driven table with sortable columns and pagination |
| `date_input` | Date picker input backed by a Calendar popover |
| `drawer` | Slide-in panel from any edge of the screen |
| `dropdown` | Accessible dropdown menu with keyboard navigation |
| `file_upload` | File input as a drag-and-drop zone or button |
| `form_field` | Form field wrapper with label, input slot, and error message |
| `icons` | SVG icon set (installed automatically as a dependency) |
| `input` | Styled text input with variant and size options |
| `label` | Accessible form label |
| `modal` | Accessible dialog with overlay and animated panel |
| `nav` | Vertical accordion navigation with nested items |
| `nav_menu` | Horizontal navigation bar with hover flyout submenus |
| `pagination` | Page navigation with previous, next, and numbered page items |
| `popover` | Floating content panel anchored to a trigger element |
| `progress` | Horizontal progress bar |
| `radio` | Controlled radio button group |
| `select` | Styled select dropdown with icon indicator |
| `separator` | Horizontal or vertical divider line |
| `sidebar` | Collapsible application sidebar with minimize support |
| `slider` | Controlled range input with styled track fill |
| `spinner` | Animated loading spinner |
| `table` | Composable table primitives |
| `tabs` | Tab bar with panel switching |
| `textarea` | Styled multi-line text input |
| `toast` | Ephemeral notification toasts with auto-dismiss |
| `tooltip` | Hover tooltip anchored to any element |

## registry.json format

```json
{
  "version": "0.1.0",
  "components": [
    {
      "name": "button",
      "description": "Accessible button with variant and size options",
      "files": ["components/button.rs"],
      "deps": [],
      "cargo_deps": [
        { "name": "yew", "version": "0.23", "features": ["csr"] }
      ]
    }
  ]
}
```

## Versioning

Publish a new release by creating a GitHub Release. CI automatically snapshots `components/`, `registry.json`, and `yei.css` into `versions/{tag}/` and commits it back. The API server can then serve any pinned version.

## Theming

`yei.css` ships default design tokens and maps them to Tailwind v4 utility classes via `@theme`. Override any token in your own CSS after the import:

```css
@import "tailwindcss";
@import "./yei.css";

:root {
  --primary: oklch(0.5 0.2 250);
  --radius: 0.375rem;
}
```

Available tokens: `--background`, `--foreground`, `--primary`, `--primary-foreground`, `--secondary`, `--secondary-foreground`, `--muted`, `--muted-foreground`, `--danger`, `--on-danger`, `--success`, `--warning`, `--field`, `--border`, `--focus`, `--radius`.
