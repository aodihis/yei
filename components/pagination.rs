use yew::prelude::*;

// --- Pagination (nav wrapper) ---

#[derive(Properties, PartialEq)]
pub struct PaginationProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

pub enum PaginationMsg {}
pub struct Pagination;

impl Component for Pagination {
    type Message = PaginationMsg;
    type Properties = PaginationProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let base = "flex items-center gap-1";
        let cls = classes!(base, p.class.clone());
        html! {
            <nav class={cls} aria-label="Pagination">
                { for p.children.iter() }
            </nav>
        }
    }
}

// --- PaginationItem ---

#[derive(Properties, PartialEq)]
pub struct PaginationItemProps {
    // --- Content ---
    #[prop_or_default]
    pub children: Children,

    // --- State ---
    #[prop_or_default]
    pub active: bool,
    #[prop_or_default]
    pub disabled: bool,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,

    // --- Callbacks ---
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

pub enum PaginationItemMsg {}
pub struct PaginationItem;

impl Component for PaginationItem {
    type Message = PaginationItemMsg;
    type Properties = PaginationItemProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let cls = pagination_item_classes(p.active, p.disabled, &p.class);
        html! {
            <a
                class={cls}
                onclick={p.onclick.clone()}
                aria-current={if p.active { "page" } else { "false" }}
                aria-disabled={if p.disabled { "true" } else { "false" }}
            >
                { for p.children.iter() }
            </a>
        }
    }
}

fn pagination_item_classes(active: bool, disabled: bool, extra: &Classes) -> Classes {
    let base = "inline-flex items-center justify-center size-9 rounded-lg text-sm border transition-colors cursor-pointer select-none";
    let state_cls = if disabled {
        "border-border text-foreground/30 pointer-events-none cursor-default"
    } else if active {
        "border-transparent bg-primary text-primary-foreground cursor-default"
    } else {
        "border-border text-foreground hover:bg-muted"
    };
    classes!(base, state_cls, extra.clone())
}

// --- PaginationEllipsis ---

#[derive(Properties, PartialEq)]
pub struct PaginationEllipsisProps {
    #[prop_or_default]
    pub class: Classes,
}

pub enum PaginationEllipsisMsg {}
pub struct PaginationEllipsis;

impl Component for PaginationEllipsis {
    type Message = PaginationEllipsisMsg;
    type Properties = PaginationEllipsisProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let base = "inline-flex items-center justify-center size-9 text-sm text-foreground/40 select-none";
        let cls = classes!(base, ctx.props().class.clone());
        html! {
            <span class={cls} aria-hidden="true">{"…"}</span>
        }
    }
}
