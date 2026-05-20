use std::rc::Rc;
use yew::prelude::*;
use crate::components::icons::{icon_chevron_right, icon_panel_left};

// --- NavIcon ---

#[derive(Clone)]
pub struct NavIcon(pub Rc<Html>);

impl NavIcon {
    pub fn new(html: Html) -> Self { Self(Rc::new(html)) }
}

impl PartialEq for NavIcon {
    fn eq(&self, other: &Self) -> bool { Rc::ptr_eq(&self.0, &other.0) }
}

// --- SidebarContext ---

#[derive(Clone, PartialEq)]
pub struct SidebarContext {
    pub open:      bool,
    pub minimized: bool,
    pub toggle:    Callback<()>,
    pub minimize:  Callback<()>,
}

// --- SidebarNavContext (internal) ---

#[derive(Clone, PartialEq)]
struct SidebarNavContext {
    minimized: bool,
    depth:     u8,
}

// --- Sidebar ---

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(true)]
    pub default_open: bool,
    #[prop_or(true)]
    pub minimizable: bool,
    #[prop_or_default]
    pub class: Classes,
}

pub enum SidebarMsg {
    Toggle,
    ToggleMinimize,
}

pub struct Sidebar {
    open:      bool,
    minimized: bool,
}

impl Component for Sidebar {
    type Message = SidebarMsg;
    type Properties = SidebarProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { open: ctx.props().default_open, minimized: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SidebarMsg::Toggle         => { self.open      = !self.open;      true }
            SidebarMsg::ToggleMinimize => { self.minimized = !self.minimized; true }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p        = ctx.props();
        let toggle   = ctx.link().callback(|_: ()| SidebarMsg::Toggle);
        let minimize = ctx.link().callback(|_: ()| SidebarMsg::ToggleMinimize);
        let minimize_cb = minimize.reform(|_: MouseEvent| ());
        let ctx_val  = SidebarContext {
            open: self.open, minimized: self.minimized,
            toggle, minimize,
        };

        let aside_cls    = sidebar_classes(self.open, self.minimized, &p.class);
        let btn_cls      = "flex items-center gap-2 w-full px-3 py-3 border-t border-border text-foreground/60 hover:bg-muted hover:text-foreground transition-colors shrink-0 text-sm";
        let chevron_base = "transition-transform duration-200";
        let chevron_cls  = classes!(chevron_base, if self.minimized { "rotate-180" } else { "" });

        html! {
            <ContextProvider<SidebarContext> context={ctx_val}>
                <aside class={aside_cls}>
                    { for p.children.iter() }
                    if p.minimizable {
                        <button type="button" class={btn_cls} onclick={minimize_cb} aria-label="Toggle minimize">
                            <span class={chevron_cls}>{ icon_chevron_right() }</span>
                            <span class="truncate yei-sidebar-label">{"Minimize"}</span>
                        </button>
                    }
                </aside>
            </ContextProvider<SidebarContext>>
        }
    }
}

// --- SidebarHeader ---

#[derive(Properties, PartialEq)]
pub struct SidebarHeaderProps {
    #[prop_or_default]
    pub logo: Option<NavIcon>,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SidebarHeader)]
pub fn sidebar_header(props: &SidebarHeaderProps) -> Html {
    if props.logo.is_none() && props.name.is_none() {
        return html! {};
    }

    let base = "flex items-center gap-3 px-4 py-4 border-b border-border shrink-0";
    let cls  = classes!(base, props.class.clone());

    html! {
        <div class={cls}>
            if let Some(logo) = &props.logo {
                <span class="shrink-0 text-xl">{ (*logo.0).clone() }</span>
            }
            if let Some(name) = &props.name {
                <span class="font-bold text-sm truncate yei-sidebar-label">{ name }</span>
            }
        </div>
    }
}

// --- SidebarTrigger ---

#[derive(Properties, PartialEq)]
pub struct SidebarTriggerProps {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SidebarTrigger)]
pub fn sidebar_trigger(props: &SidebarTriggerProps) -> Html {
    let sidebar_ctx = use_context::<SidebarContext>();
    let onclick = sidebar_ctx
        .map(|c| c.toggle.reform(|_: MouseEvent| ()))
        .unwrap_or_default();

    let base = "inline-flex items-center justify-center h-8 w-8 rounded-md text-foreground/70 hover:bg-muted hover:text-foreground transition-colors";
    let cls  = classes!(base, props.class.clone());
    html! {
        <button type="button" class={cls} onclick={onclick} aria-label="Toggle sidebar">
            { icon_panel_left() }
        </button>
    }
}

// --- SidebarContent ---

#[derive(Properties, PartialEq)]
pub struct SidebarContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SidebarContent)]
pub fn sidebar_content(props: &SidebarContentProps) -> Html {
    let base = "flex-1 overflow-y-auto overflow-x-hidden p-2";
    let cls  = classes!(base, props.class.clone());
    html! { <div class={cls}>{ for props.children.iter() }</div> }
}

// --- SidebarFooter ---

#[derive(Properties, PartialEq)]
pub struct SidebarFooterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SidebarFooter)]
pub fn sidebar_footer(props: &SidebarFooterProps) -> Html {
    let base = "px-4 py-3 border-t border-border shrink-0 yei-sidebar-label";
    let cls  = classes!(base, props.class.clone());
    html! { <div class={cls}>{ for props.children.iter() }</div> }
}

// --- SidebarSeparator ---

#[derive(Properties, PartialEq)]
pub struct SidebarSeparatorProps {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SidebarSeparator)]
pub fn sidebar_separator(props: &SidebarSeparatorProps) -> Html {
    let base = "my-2 h-px bg-border mx-2";
    let cls  = classes!(base, props.class.clone());
    html! { <div class={cls} role="separator" /> }
}

// --- SidebarNav ---

#[derive(Properties, PartialEq)]
pub struct SidebarNavProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SidebarNav)]
pub fn sidebar_nav(props: &SidebarNavProps) -> Html {
    let minimized = use_context::<SidebarContext>()
        .map(|c| c.minimized)
        .unwrap_or(false);
    let nav_ctx = SidebarNavContext { minimized, depth: 0 };
    let base = "flex flex-col space-y-0.5";
    let cls  = classes!(base, props.class.clone());
    html! {
        <ContextProvider<SidebarNavContext> context={nav_ctx}>
            <ul class={cls}>{ for props.children.iter() }</ul>
        </ContextProvider<SidebarNavContext>>
    }
}

// --- SidebarNavItem ---

#[derive(Properties, PartialEq)]
pub struct SidebarNavItemProps {
    pub title: AttrValue,
    #[prop_or(AttrValue::Static("#"))]
    pub href: AttrValue,
    #[prop_or_default]
    pub icon: Option<NavIcon>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

pub enum SidebarNavItemMsg { Toggle }

pub struct SidebarNavItem {
    open: bool,
}

impl Component for SidebarNavItem {
    type Message = SidebarNavItemMsg;
    type Properties = SidebarNavItemProps;

    fn create(_ctx: &Context<Self>) -> Self { Self { open: false } }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SidebarNavItemMsg::Toggle => { self.open = !self.open; true }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let nav_ctx = ctx.link()
            .context::<SidebarNavContext>(Callback::noop())
            .map(|(c, _)| c)
            .unwrap_or(SidebarNavContext { minimized: false, depth: 0 });

        if p.children.iter().next().is_some() {
            self.render_parent(ctx, &nav_ctx)
        } else {
            self.render_leaf(ctx, &nav_ctx)
        }
    }
}

impl SidebarNavItem {
    fn render_parent(&self, ctx: &Context<Self>, nav_ctx: &SidebarNavContext) -> Html {
        let p         = ctx.props();
        let minimized = nav_ctx.minimized;
        let child_ctx = SidebarNavContext { minimized, depth: nav_ctx.depth + 1 };

        let row_base  = "flex items-center w-full rounded-md cursor-pointer select-none text-sm font-medium transition-colors";
        let row_pad   = if minimized { "p-0 justify-center" } else { "px-3 py-2.5" };
        let row_state = if self.open { "bg-muted text-foreground" } else { "text-foreground/80 hover:bg-muted hover:text-foreground" };
        let row_cls   = classes!(row_base, row_pad, row_state, p.class.clone());

        let chevron_rot  = if self.open { "rotate-90" } else { "" };
        let chevron_base = "ml-auto shrink-0 transition-transform duration-200 yei-sidebar-label";
        let chevron_cls  = classes!(chevron_base, chevron_rot);

        let body_base = "overflow-hidden transition-all duration-200";
        let body_cls  = classes!(body_base, if !minimized && self.open { "max-h-[60rem]" } else { "max-h-0" });

        let on_toggle = if minimized {
            Callback::default()
        } else {
            ctx.link().callback(|_: MouseEvent| SidebarNavItemMsg::Toggle)
        };

        html! {
            <li>
                <button class={row_cls} onclick={on_toggle} aria-expanded={(!minimized && self.open).to_string()}>
                    { sidebar_icon_slot(&p.icon, minimized) }
                    <span class="ml-2 flex-1 text-left truncate yei-sidebar-label">{ &p.title }</span>
                    <span class={chevron_cls}>{ icon_chevron_right() }</span>
                </button>
                <div class={body_cls}>
                    <ul class="pl-3 mt-0.5 space-y-0.5">
                        <ContextProvider<SidebarNavContext> context={child_ctx}>
                            { for p.children.iter() }
                        </ContextProvider<SidebarNavContext>>
                    </ul>
                </div>
            </li>
        }
    }

    fn render_leaf(&self, ctx: &Context<Self>, nav_ctx: &SidebarNavContext) -> Html {
        let p         = ctx.props();
        let minimized = nav_ctx.minimized;

        let base    = "flex items-center rounded-md text-sm text-foreground/70 hover:bg-muted hover:text-foreground transition-colors select-none";
        let pad_cls = if minimized { "p-0 justify-center w-full" } else { "px-3 py-2 w-full" };
        let cls     = classes!(base, pad_cls, p.class.clone());
        let title   = if minimized { p.title.clone() } else { AttrValue::default() };

        html! {
            <li>
                <a class={cls} href={p.href.clone()} onclick={p.onclick.clone()} title={title}>
                    { sidebar_icon_slot(&p.icon, minimized) }
                    <span class="ml-2 truncate yei-sidebar-label">{ &p.title }</span>
                </a>
            </li>
        }
    }
}

// --- SidebarNavLabel ---

#[derive(Properties, PartialEq)]
pub struct SidebarNavLabelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SidebarNavLabel)]
pub fn sidebar_nav_label(props: &SidebarNavLabelProps) -> Html {
    let base = "px-3 py-1.5 text-xs font-semibold text-foreground/50 select-none yei-sidebar-label";
    let cls  = classes!(base, props.class.clone());
    html! { <li><div class={cls}>{ for props.children.iter() }</div></li> }
}

// --- Helpers ---

fn sidebar_icon_slot(icon: &Option<NavIcon>, minimized: bool) -> Html {
    let content = icon.as_ref().map(|i| (*i.0).clone()).unwrap_or_default();
    let cls = if minimized {
        "w-10 h-10 flex items-center justify-center shrink-0"
    } else {
        "w-5 h-5 flex items-center justify-center shrink-0 text-base"
    };
    html! { <span class={cls}>{ content }</span> }
}

fn sidebar_classes(open: bool, minimized: bool, extra: &Classes) -> Classes {
    let base    = "yei-sidebar flex flex-col h-full bg-background border-r border-border transition-all duration-300 overflow-hidden shrink-0";
    let width   = match (open, minimized) {
        (false, _)    => "w-0",
        (true, true)  => "w-14",
        (true, false) => "w-64",
    };
    let min_cls = if minimized { "yei-sidebar--minimized" } else { "" };
    classes!(base, width, min_cls, extra.clone())
}
