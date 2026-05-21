# Sidebar

Collapsible application sidebar with header, nav, footer, and minimize support.

## Installation

```bash
yei add sidebar
```

## Dependencies

- `icons`

## Props

### `Sidebar`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | — | Sidebar sections |
| `default_open` | `bool` | `true` | Whether the sidebar starts expanded |
| `minimizable` | `bool` | `true` | Show the minimize toggle button at the bottom |
| `class` | `Classes` | `""` | Extra CSS classes |

### `SidebarHeader`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `logo` | `Option<NavIcon>` | `None` | Icon/logo; wrap with `NavIcon::new(html! { ... })` |
| `name` | `Option<AttrValue>` | `None` | App or brand name |
| `class` | `Classes` | `""` | Extra CSS classes |

### `SidebarNavItem`

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `title` | `AttrValue` | required | Display text |
| `href` | `AttrValue` | `"#"` | Link URL (leaf items only) |
| `icon` | `Option<NavIcon>` | `None` | Leading icon |
| `children` | `Children` | — | Nested items (triggers accordion) |
| `class` | `Classes` | `""` | Extra CSS classes |
| `onclick` | `Callback<MouseEvent>` | no-op | Click handler for leaf items |

### Additional sub-components

| Component | Description |
|-----------|-------------|
| `SidebarTrigger` | Button that calls `SidebarContext.toggle` (reads context automatically) |
| `SidebarContent` | Scrollable nav area |
| `SidebarFooter` | Bottom section, hidden when minimized |
| `SidebarSeparator` | Thin horizontal rule inside the sidebar |
| `SidebarNav` | `<ul>` wrapper; provides minimize context to `SidebarNavItem` |
| `SidebarNavLabel` | Non-interactive section heading |

## Usage

```rust
use crate::components::sidebar::{
    Sidebar, SidebarHeader, SidebarContent, SidebarNav,
    SidebarNavItem, SidebarNavLabel, SidebarTrigger, NavIcon,
};
use crate::components::icons::icon_layout_dashboard;

html! {
    <div class="flex h-screen">
        <Sidebar>
            <SidebarHeader name={Some(AttrValue::from("MyApp"))} />
            <SidebarContent>
                <SidebarNav>
                    <SidebarNavLabel>{"Navigation"}</SidebarNavLabel>
                    <SidebarNavItem
                        title="Dashboard"
                        href="/dashboard"
                        icon={Some(NavIcon::new(icon_layout_dashboard()))}
                    />
                    <SidebarNavItem title="Settings">
                        <SidebarNavItem title="Profile" href="/settings/profile" />
                    </SidebarNavItem>
                </SidebarNav>
            </SidebarContent>
        </Sidebar>
        <main class="flex-1 p-6">
            <SidebarTrigger />
            // page content
        </main>
    </div>
}
```

## Variants / Notes

The sidebar exposes a `SidebarContext` (open, minimized, toggle, minimize callbacks) via Yew context, which `SidebarTrigger` reads automatically. In minimized state (`w-14`), only icons are visible; labels are hidden with `yei-sidebar-label` CSS class toggling. The full-open width is `w-64`.
