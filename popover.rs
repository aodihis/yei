use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub enum PopoverPosition {
    #[default]
    Bottom,
    Top,
    Left,
    Right,
}

// Rc<Html> newtype so Properties can derive PartialEq via ptr_eq.
#[derive(Clone)]
pub struct PopoverContent(pub Rc<Html>);

impl PopoverContent {
    pub fn new(content: Html) -> Self {
        Self(Rc::new(content))
    }
}

impl PartialEq for PopoverContent {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Properties, PartialEq)]
pub struct PopoverProps {
    // --- Content ---
    #[prop_or_default]
    pub children: Children,   // trigger element
    pub content: PopoverContent, // panel body

    // --- Style ---
    #[prop_or_default]
    pub position: PopoverPosition,
    #[prop_or_default]
    pub class: Classes,
}

pub enum PopoverMsg {
    Toggle,
    Close,
}

pub struct Popover {
    open: bool,
}

impl Component for Popover {
    type Message = PopoverMsg;
    type Properties = PopoverProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { open: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PopoverMsg::Toggle => { self.open = !self.open; true }
            PopoverMsg::Close  => { self.open = false; true }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let on_toggle = ctx.link().callback(|_: MouseEvent| PopoverMsg::Toggle);
        let on_close  = ctx.link().callback(|_: MouseEvent| PopoverMsg::Close);
        let wrapper_base = "relative inline-block";
        let wrapper = classes!(wrapper_base, p.class.clone());
        let panel_cls = popover_panel_classes(&p.position);

        html! {
            <div class={wrapper}>
                <div onclick={on_toggle}>
                    { for p.children.iter() }
                </div>

                if self.open {
                    // Transparent backdrop — closes panel on outside click
                    <div class="fixed inset-0 z-40" onclick={on_close} />
                    // Panel
                    <div class={panel_cls} role="dialog">
                        { (*p.content.0).clone() }
                    </div>
                }
            </div>
        }
    }
}

fn popover_panel_classes(pos: &PopoverPosition) -> Classes {
    let base = "absolute z-50 min-w-48 rounded-lg border border-border bg-background shadow-lg p-4";
    let pos_cls = match pos {
        PopoverPosition::Bottom => "top-full mt-2 left-0",
        PopoverPosition::Top    => "bottom-full mb-2 left-0",
        PopoverPosition::Left   => "right-full mr-2 top-0",
        PopoverPosition::Right  => "left-full ml-2 top-0",
    };
    classes!(base, pos_cls)
}
