use yew::prelude::*;
use crate::components::icons::icon_x;

// --- Modal (controlled) ---

#[derive(Properties, PartialEq)]
pub struct ModalProps {
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
    pub class: Classes,

    // --- Callbacks ---
    pub on_close: Callback<()>,
}

pub enum ModalMsg {}
pub struct Modal;

impl Component for Modal {
    type Message = ModalMsg;
    type Properties = ModalProps;

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
        // Stop clicks inside the panel from bubbling to the backdrop
        let stop_prop = Callback::from(|e: MouseEvent| e.stop_propagation());

        let panel_base = "yei-modal-panel relative w-full max-w-md mx-4 bg-background rounded-lg shadow-sm";
        let panel_cls = classes!(panel_base, p.class.clone());

        html! {
            // Overlay — flex-centers the panel
            <div
                class="yei-modal-overlay fixed inset-0 z-50 flex items-center justify-center bg-black/50"
                onclick={on_backdrop}
            >
                // Panel
                <div class={panel_cls} onclick={stop_prop}>
                    // Close button (optional)
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

// --- ModalHeader ---

#[derive(Properties, PartialEq)]
pub struct ModalHeaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

pub enum ModalHeaderMsg {}
pub struct ModalHeader;

impl Component for ModalHeader {
    type Message = ModalHeaderMsg;
    type Properties = ModalHeaderProps;

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

// --- ModalContent ---

#[derive(Properties, PartialEq)]
pub struct ModalContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

pub enum ModalContentMsg {}
pub struct ModalContent;

impl Component for ModalContent {
    type Message = ModalContentMsg;
    type Properties = ModalContentProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let base = "px-6 pb-4 text-sm text-foreground/80";
        let cls = classes!(base, p.class.clone());
        html! {
            <div class={cls}>{ for p.children.iter() }</div>
        }
    }
}

// --- ModalFooter ---

#[derive(Properties, PartialEq)]
pub struct ModalFooterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

pub enum ModalFooterMsg {}
pub struct ModalFooter;

impl Component for ModalFooter {
    type Message = ModalFooterMsg;
    type Properties = ModalFooterProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let base = "flex items-center justify-end gap-2 px-6 pb-6 pt-2";
        let cls = classes!(base, p.class.clone());
        html! {
            <div class={cls}>{ for p.children.iter() }</div>
        }
    }
}
