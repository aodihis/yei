use yew::prelude::*;
use crate::components::icons::icon_x;

// --- Side ---

#[derive(Clone, PartialEq, Default)]
pub enum DrawerSide {
    #[default]
    Right,
    Left,
    Top,
    Bottom,
}

// --- Props ---

#[derive(Properties, PartialEq)]
pub struct DrawerProps {
    // --- Content ---
    #[prop_or_default]
    pub children: Children,

    // --- State ---
    pub open: bool,
    #[prop_or(true)]
    pub show_close: bool,
    #[prop_or(true)]
    pub close_on_backdrop: bool,

    // --- Style ---
    #[prop_or_default]
    pub side: DrawerSide,
    #[prop_or_default]
    pub class: Classes,

    // --- Callbacks ---
    pub on_close: Callback<()>,
}

pub enum DrawerMsg {}
pub struct Drawer;

impl Component for Drawer {
    type Message = DrawerMsg;
    type Properties = DrawerProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();

        if !p.open {
            return html! {};
        }

        let on_backdrop = if p.close_on_backdrop {
            let cb = p.on_close.clone();
            Callback::from(move |_: MouseEvent| cb.emit(()))
        } else {
            Callback::from(|_: MouseEvent| ())
        };
        let on_close_btn = {
            let cb = p.on_close.clone();
            Callback::from(move |_: MouseEvent| cb.emit(()))
        };
        let stop_prop = Callback::from(|e: MouseEvent| e.stop_propagation());

        let panel_cls = drawer_panel_classes(&p.side, &p.class);

        html! {
            <div
                class="fixed inset-0 z-50 bg-black/50"
                onclick={on_backdrop}
            >
                <div class={panel_cls} onclick={stop_prop}>
                    if p.show_close {
                        <button
                            onclick={on_close_btn}
                            aria-label="Close"
                            class="absolute top-3 right-3 p-1 rounded-md text-foreground/50 hover:text-foreground hover:bg-muted transition-colors"
                        >
                            { icon_x() }
                        </button>
                    }
                    { for p.children.iter() }
                </div>
            </div>
        }
    }
}

fn drawer_panel_classes(side: &DrawerSide, extra: &Classes) -> Classes {
    let base = "fixed z-50 bg-background flex flex-col shadow-lg";
    let side_cls = match side {
        DrawerSide::Left   => "animate-drawer-in-left inset-y-0 left-0 h-full w-80 border-r border-border",
        DrawerSide::Right  => "animate-drawer-in-right inset-y-0 right-0 h-full w-80 border-l border-border",
        DrawerSide::Top    => "animate-drawer-in-top inset-x-0 top-0 w-full border-b border-border",
        DrawerSide::Bottom => "animate-drawer-in-bottom inset-x-0 bottom-0 w-full border-t border-border",
    };
    classes!(base, side_cls, extra.clone())
}

// --- DrawerHeader ---

#[derive(Properties, PartialEq)]
pub struct DrawerHeaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

pub enum DrawerHeaderMsg {}
pub struct DrawerHeader;

impl Component for DrawerHeader {
    type Message = DrawerHeaderMsg;
    type Properties = DrawerHeaderProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let base = "px-6 pt-6 pb-4 pr-12";
        let cls = classes!(base, p.class.clone());
        html! {
            <div class={cls}>{ for p.children.iter() }</div>
        }
    }
}

// --- DrawerContent ---

#[derive(Properties, PartialEq)]
pub struct DrawerContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

pub enum DrawerContentMsg {}
pub struct DrawerContent;

impl Component for DrawerContent {
    type Message = DrawerContentMsg;
    type Properties = DrawerContentProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let base = "flex-1 overflow-y-auto px-6 pb-4 text-sm text-foreground/80";
        let cls = classes!(base, p.class.clone());
        html! {
            <div class={cls}>{ for p.children.iter() }</div>
        }
    }
}

// --- DrawerFooter ---

#[derive(Properties, PartialEq)]
pub struct DrawerFooterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

pub enum DrawerFooterMsg {}
pub struct DrawerFooter;

impl Component for DrawerFooter {
    type Message = DrawerFooterMsg;
    type Properties = DrawerFooterProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let base = "flex items-center justify-end gap-2 px-6 py-4 border-t border-border";
        let cls = classes!(base, p.class.clone());
        html! {
            <div class={cls}>{ for p.children.iter() }</div>
        }
    }
}
