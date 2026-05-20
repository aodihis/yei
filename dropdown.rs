use std::rc::Rc;
use yew::prelude::*;

// --- Item variant ---

#[derive(Clone, PartialEq, Default)]
pub enum DropdownItemVariant {
    #[default]
    Default,
    Danger,
}

// --- Position ---

#[derive(Clone, PartialEq, Default)]
pub enum DropdownPosition {
    #[default]
    Bottom,
    Top,
    Left,
    Right,
}

// --- DropdownContent (Rc<Html> newtype for PartialEq) ---

#[derive(Clone)]
pub struct DropdownContent(pub Rc<Html>);

impl DropdownContent {
    pub fn new(content: Html) -> Self {
        Self(Rc::new(content))
    }
}

impl PartialEq for DropdownContent {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

// --- Dropdown ---

#[derive(Properties, PartialEq)]
pub struct DropdownProps {
    // --- Content ---
    #[prop_or_default]
    pub children: Children,   // trigger element
    pub content: DropdownContent, // menu panel body

    // --- Style ---
    #[prop_or_default]
    pub position: DropdownPosition,
    #[prop_or_default]
    pub class: Classes,
}

pub enum DropdownMsg {
    Toggle,
    Close,
}

pub struct Dropdown {
    open: bool,
}

impl Component for Dropdown {
    type Message = DropdownMsg;
    type Properties = DropdownProps;

    fn create(_ctx: &Context<Self>) -> Self { Self { open: false } }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DropdownMsg::Toggle => { self.open = !self.open; true }
            DropdownMsg::Close  => { self.open = false; true }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let on_toggle = ctx.link().callback(|_: MouseEvent| DropdownMsg::Toggle);
        let on_close  = ctx.link().callback(|_: MouseEvent| DropdownMsg::Close);

        // Stop propagation so clicks don't re-trigger the toggle div;
        // also close the menu so item clicks dismiss it automatically.
        let on_panel_click = {
            let link = ctx.link().clone();
            Callback::from(move |e: MouseEvent| {
                e.stop_propagation();
                link.send_message(DropdownMsg::Close);
            })
        };

        let wrapper_base = "relative inline-block";
        let wrapper = classes!(wrapper_base, p.class.clone());
        let panel_cls = dropdown_panel_classes(&p.position);

        html! {
            <div class={wrapper}>
                <div onclick={on_toggle}>
                    { for p.children.iter() }
                </div>
                if self.open {
                    // Transparent backdrop — outside clicks close the menu
                    <div class="fixed inset-0 z-40" onclick={on_close} />
                    // Panel
                    <div class={panel_cls} role="menu" onclick={on_panel_click}>
                        { (*p.content.0).clone() }
                    </div>
                }
            </div>
        }
    }
}

fn dropdown_panel_classes(pos: &DropdownPosition) -> Classes {
    let base = "absolute z-50 min-w-40 rounded-lg border border-border bg-background shadow-lg py-1";
    let pos_cls = match pos {
        DropdownPosition::Bottom => "top-full mt-1 left-0",
        DropdownPosition::Top    => "bottom-full mb-1 left-0",
        DropdownPosition::Left   => "right-full mr-1 top-0",
        DropdownPosition::Right  => "left-full ml-1 top-0",
    };
    classes!(base, pos_cls)
}

// --- DropdownItem ---

#[derive(Properties, PartialEq)]
pub struct DropdownItemProps {
    // --- Content ---
    #[prop_or_default]
    pub children: Children,

    // --- State ---
    #[prop_or_default]
    pub disabled: bool,

    // --- Style ---
    #[prop_or_default]
    pub variant: DropdownItemVariant,
    #[prop_or_default]
    pub class: Classes,

    // --- Callbacks ---
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

pub enum DropdownItemMsg {}
pub struct DropdownItem;

impl Component for DropdownItem {
    type Message = DropdownItemMsg;
    type Properties = DropdownItemProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let cls = dropdown_item_classes(&p.variant, p.disabled, &p.class);
        if p.disabled {
            html! {
                <div class={cls} aria-disabled="true" role="menuitem">
                    { for p.children.iter() }
                </div>
            }
        } else {
            html! {
                <div class={cls} role="menuitem" onclick={p.onclick.clone()}>
                    { for p.children.iter() }
                </div>
            }
        }
    }
}

fn dropdown_item_classes(variant: &DropdownItemVariant, disabled: bool, extra: &Classes) -> Classes {
    let base = "flex items-center gap-2 px-3 py-1.5 text-sm cursor-pointer select-none";
    let variant_cls = match variant {
        DropdownItemVariant::Default => "text-foreground hover:bg-muted",
        DropdownItemVariant::Danger  => "text-danger hover:bg-danger/10",
    };
    let disabled_cls = if disabled { "opacity-50 pointer-events-none" } else { "" };
    classes!(base, variant_cls, disabled_cls, extra.clone())
}

// --- DropdownLabel ---

#[derive(Properties, PartialEq)]
pub struct DropdownLabelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

pub enum DropdownLabelMsg {}
pub struct DropdownLabel;

impl Component for DropdownLabel {
    type Message = DropdownLabelMsg;
    type Properties = DropdownLabelProps;

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

// --- DropdownSeparator ---

pub struct DropdownSeparator;
pub enum DropdownSeparatorMsg {}

#[derive(Properties, PartialEq)]
pub struct DropdownSeparatorProps {
    #[prop_or_default]
    pub class: Classes,
}

impl Component for DropdownSeparator {
    type Message = DropdownSeparatorMsg;
    type Properties = DropdownSeparatorProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let base = "my-1 h-px bg-border";
        let cls = classes!(base, ctx.props().class.clone());
        html! { <div class={cls} role="separator" /> }
    }
}
