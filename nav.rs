use yew::prelude::*;
use crate::components::icons::icon_chevron_right;

// --- Nav ---

#[derive(Properties, PartialEq)]
pub struct NavProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Nav)]
pub fn nav(props: &NavProps) -> Html {
    let base = "flex flex-col space-y-0.5";
    let cls  = classes!(base, props.class.clone());
    html! { <ul class={cls}>{ for props.children.iter() }</ul> }
}

// --- NavItem ---

#[derive(Properties, PartialEq)]
pub struct NavItemProps {
    pub title: AttrValue,
    #[prop_or(AttrValue::Static("#"))]
    pub href: AttrValue,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

pub enum NavItemMsg { Toggle }

pub struct NavItem {
    open: bool,
}

impl Component for NavItem {
    type Message = NavItemMsg;
    type Properties = NavItemProps;

    fn create(_ctx: &Context<Self>) -> Self { Self { open: false } }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NavItemMsg::Toggle => { self.open = !self.open; true }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        if p.children.iter().next().is_some() {
            self.render_parent(ctx)
        } else {
            self.render_leaf(ctx)
        }
    }
}

impl NavItem {
    fn render_parent(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();

        let row_base  = "flex items-center w-full px-3 py-2.5 rounded-md cursor-pointer select-none text-sm font-medium transition-colors";
        let row_state = if self.open { "bg-muted text-foreground" } else { "text-foreground/80 hover:bg-muted hover:text-foreground" };
        let row_cls   = classes!(row_base, row_state, p.class.clone());

        let chevron_rot  = if self.open { "rotate-90" } else { "" };
        let chevron_base = "ml-auto shrink-0 transition-transform duration-200";
        let chevron_cls  = classes!(chevron_base, chevron_rot);

        let body_base = "overflow-hidden transition-all duration-200";
        let body_cls  = classes!(body_base, if self.open { "max-h-[60rem]" } else { "max-h-0" });

        let on_toggle = ctx.link().callback(|_: MouseEvent| NavItemMsg::Toggle);

        html! {
            <li>
                <button class={row_cls} onclick={on_toggle} aria-expanded={self.open.to_string()}>
                    <span class="flex-1 text-left truncate">{ &p.title }</span>
                    <span class={chevron_cls}>{ icon_chevron_right() }</span>
                </button>
                <div class={body_cls}>
                    <ul class="pl-3 mt-0.5 space-y-0.5">
                        { for p.children.iter() }
                    </ul>
                </div>
            </li>
        }
    }

    fn render_leaf(&self, ctx: &Context<Self>) -> Html {
        let p    = ctx.props();
        let base = "flex items-center w-full px-3 py-2 rounded-md text-sm text-foreground/70 hover:bg-muted hover:text-foreground transition-colors select-none";
        let cls  = classes!(base, p.class.clone());

        html! {
            <li>
                <a class={cls} href={p.href.clone()} onclick={p.onclick.clone()}>
                    <span class="truncate">{ &p.title }</span>
                </a>
            </li>
        }
    }
}

// --- NavLabel ---

#[derive(Properties, PartialEq)]
pub struct NavLabelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(NavLabel)]
pub fn nav_label(props: &NavLabelProps) -> Html {
    let base = "px-3 py-1.5 text-xs font-semibold text-foreground/50 select-none";
    let cls  = classes!(base, props.class.clone());
    html! { <li><div class={cls}>{ for props.children.iter() }</div></li> }
}
