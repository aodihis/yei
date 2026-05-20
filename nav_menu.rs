use yew::prelude::*;
use crate::components::icons::{icon_chevron_down, icon_chevron_right};

// --- NavMenuContext ---

#[derive(Clone, PartialEq)]
pub struct NavMenuContext {
    pub depth: u8,
}

// --- NavMenu ---

#[derive(Properties, PartialEq)]
pub struct NavMenuProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

pub enum NavMenuMsg {}
pub struct NavMenu;

impl Component for NavMenu {
    type Message = NavMenuMsg;
    type Properties = NavMenuProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let nav_ctx = NavMenuContext { depth: 0 };
        let base = "flex items-center gap-1";
        let cls = classes!(base, p.class.clone());
        html! {
            <ContextProvider<NavMenuContext> context={nav_ctx}>
                <div class={cls}>{ for p.children.iter() }</div>
            </ContextProvider<NavMenuContext>>
        }
    }
}

// --- NavMenuItem ---

#[derive(Properties, PartialEq)]
pub struct NavMenuItemProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub description: Option<AttrValue>,
    #[prop_or(AttrValue::Static("#"))]
    pub href: AttrValue,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

pub enum NavMenuItemMsg {
    Open,
    Close,
}

pub struct NavMenuItem {
    open: bool,
}

impl Component for NavMenuItem {
    type Message = NavMenuItemMsg;
    type Properties = NavMenuItemProps;

    fn create(_ctx: &Context<Self>) -> Self { Self { open: false } }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NavMenuItemMsg::Open  => { if !self.open { self.open = true;  true } else { false } }
            NavMenuItemMsg::Close => { if  self.open { self.open = false; true } else { false } }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let has_children = p.children.iter().next().is_some();
        let nav_ctx = ctx.link()
            .context::<NavMenuContext>(Callback::noop())
            .map(|(c, _)| c)
            .unwrap_or(NavMenuContext { depth: 0 });

        match (nav_ctx.depth == 0, has_children) {
            (true,  true)  => self.render_top_trigger(ctx, &nav_ctx),
            (true,  false) => self.render_top_leaf(ctx),
            (false, true)  => self.render_sub_trigger(ctx, &nav_ctx),
            (false, false) => self.render_sub_leaf(ctx),
        }
    }
}

impl NavMenuItem {
    fn render_top_trigger(&self, ctx: &Context<Self>, _nav_ctx: &NavMenuContext) -> Html {
        let p = ctx.props();
        let on_enter  = ctx.link().callback(|_: MouseEvent| NavMenuItemMsg::Open);
        let on_leave  = ctx.link().callback(|_: MouseEvent| NavMenuItemMsg::Close);
        let child_ctx = NavMenuContext { depth: 1 };

        let trigger_base     = "inline-flex items-center gap-1 px-3 py-2 text-sm rounded-md transition-colors select-none cursor-pointer";
        let trigger_active   = "bg-muted text-foreground";
        let trigger_inactive = "text-foreground/70 hover:bg-muted hover:text-foreground";
        let trigger_cls = classes!(trigger_base, if self.open { trigger_active } else { trigger_inactive }, p.class.clone());
        let chevron_cls = if self.open { "transition-transform duration-150 rotate-180" } else { "transition-transform duration-150" };

        html! {
            <div class="relative" onmouseenter={on_enter} onmouseleave={on_leave}>
                <button class={trigger_cls} aria-expanded={self.open.to_string()}>
                    { &p.title }
                    <span class={chevron_cls}>{ icon_chevron_down() }</span>
                </button>
                if self.open {
                    <div class="absolute top-full left-0 pt-1 z-50">
                        <div
                            class="min-w-56 rounded-lg border border-border bg-background shadow-lg p-2"
                            role="region"
                        >
                            <ContextProvider<NavMenuContext> context={child_ctx}>
                                { for p.children.iter() }
                            </ContextProvider<NavMenuContext>>
                        </div>
                    </div>
                }
            </div>
        }
    }

    fn render_top_leaf(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let base = "inline-flex items-center gap-1 px-3 py-2 text-sm rounded-md transition-colors select-none text-foreground/70 hover:bg-muted hover:text-foreground";
        let cls = classes!(base, p.class.clone());
        html! {
            <a class={cls} href={p.href.clone()} onclick={p.onclick.clone()}>
                { &p.title }
            </a>
        }
    }

    fn render_sub_trigger(&self, ctx: &Context<Self>, nav_ctx: &NavMenuContext) -> Html {
        let p = ctx.props();
        let on_enter  = ctx.link().callback(|_: MouseEvent| NavMenuItemMsg::Open);
        let on_leave  = ctx.link().callback(|_: MouseEvent| NavMenuItemMsg::Close);
        let child_ctx = NavMenuContext { depth: nav_ctx.depth + 1 };

        let row_base     = "flex items-center justify-between gap-6 rounded-md p-3 cursor-pointer select-none transition-colors";
        let row_active   = "bg-muted";
        let row_inactive = "hover:bg-muted";
        let row_cls   = classes!(row_base, if self.open { row_active } else { row_inactive }, p.class.clone());
        let arrow_cls = if self.open { "text-foreground shrink-0" } else { "text-foreground/40 shrink-0" };

        html! {
            <div class="relative" onmouseenter={on_enter} onmouseleave={on_leave}>
                <div class={row_cls}>
                    <div>
                        <div class="text-sm font-medium text-foreground leading-snug">{ &p.title }</div>
                        if let Some(desc) = &p.description {
                            <div class="text-xs text-foreground/60 mt-0.5 leading-snug">{ desc }</div>
                        }
                    </div>
                    <span class={arrow_cls}>{ icon_chevron_right() }</span>
                </div>
                if self.open {
                    <div class="absolute left-full top-0 pl-1 z-50">
                        <div
                            class="min-w-48 rounded-lg border border-border bg-background shadow-lg p-2"
                            role="region"
                        >
                            <ContextProvider<NavMenuContext> context={child_ctx}>
                                { for p.children.iter() }
                            </ContextProvider<NavMenuContext>>
                        </div>
                    </div>
                }
            </div>
        }
    }

    fn render_sub_leaf(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let base = "block rounded-md p-3 hover:bg-muted transition-colors cursor-pointer select-none";
        let cls = classes!(base, p.class.clone());
        html! {
            <a class={cls} href={p.href.clone()} onclick={p.onclick.clone()}>
                <div class="text-sm font-medium text-foreground leading-snug">{ &p.title }</div>
                if let Some(desc) = &p.description {
                    <div class="text-xs text-foreground/60 mt-0.5 leading-snug">{ desc }</div>
                }
            </a>
        }
    }
}

// --- NavMenuLabel ---

#[derive(Properties, PartialEq)]
pub struct NavMenuLabelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

pub enum NavMenuLabelMsg {}
pub struct NavMenuLabel;

impl Component for NavMenuLabel {
    type Message = NavMenuLabelMsg;
    type Properties = NavMenuLabelProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let base = "px-3 py-1.5 text-xs font-semibold text-foreground/50 select-none";
        let cls = classes!(base, p.class.clone());
        html! {
            <div class={cls}>{ for p.children.iter() }</div>
        }
    }
}
