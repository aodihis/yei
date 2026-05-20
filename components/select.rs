use std::rc::Rc;
use yew::prelude::*;
use crate::components::icons::{icon_chevron_up, icon_chevron_down};

#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub value: AttrValue,
    pub label: AttrValue,
}

#[derive(Properties, PartialEq)]
pub struct SelectProps {
    // --- Content ---
    #[prop_or_default]
    pub options: Rc<Vec<SelectOption>>,
    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or_default]
    pub placeholder: AttrValue,

    // --- State ---
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub disabled: bool,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,

    // --- Callbacks ---
    #[prop_or_default]
    pub onchange: Callback<Event>,
}

pub enum SelectMsg {
    SetOpen(bool),
    Change(Event),
}

pub struct Select {
    open: bool,
    node_ref: NodeRef,
}

impl Component for Select {
    type Message = SelectMsg;
    type Properties = SelectProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { open: false, node_ref: NodeRef::default() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SelectMsg::SetOpen(v) => {
                self.open = v;
                true
            }
            SelectMsg::Change(e) => {
                self.open = false;
                if let Some(el) = self.node_ref.cast::<web_sys::HtmlSelectElement>() {
                    let _ = el.blur();
                }
                ctx.props().onchange.emit(e);
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old: &Self::Properties) -> bool {
        !Rc::ptr_eq(&ctx.props().options, &old.options)
            || ctx.props().value       != old.value
            || ctx.props().placeholder != old.placeholder
            || ctx.props().id          != old.id
            || ctx.props().disabled    != old.disabled
            || ctx.props().class       != old.class
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let link = ctx.link();
        let (select_cls, wrapper_cls) = select_classes(&p.class);
        let on_focus  = link.callback(|_| SelectMsg::SetOpen(true));
        let on_blur   = link.callback(|_| SelectMsg::SetOpen(false));
        let on_change = link.callback(SelectMsg::Change);

        html! {
            <div class={wrapper_cls}>
                <select
                    ref={self.node_ref.clone()}
                    id={p.id.clone()}
                    class={select_cls}
                    disabled={p.disabled}
                    onchange={on_change}
                    onfocus={on_focus}
                    onblur={on_blur}
                >
                    if !p.placeholder.is_empty() {
                        <option value="" disabled=true selected={p.value.is_empty()}>
                            { p.placeholder.clone() }
                        </option>
                    }
                    { for p.options.iter().map(|opt| {
                        let selected = opt.value == p.value;
                        html! {
                            <option value={opt.value.clone()} selected={selected}>
                                { opt.label.clone() }
                            </option>
                        }
                    }) }
                </select>
                <span class="pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 text-foreground/50">
                    { icon_chevron(self.open) }
                </span>
            </div>
        }
    }
}

fn icon_chevron(open: bool) -> Html {
    if open { icon_chevron_up() } else { icon_chevron_down() }
}

fn select_classes(extra: &Classes) -> (Classes, Classes) {
    let select_base = "h-10 w-full rounded-lg border border-transparent bg-field pl-3 pr-8 py-2 text-sm \
                       appearance-none focus-visible:outline-none focus-visible:border-primary \
                       focus-visible:ring-1 focus-visible:ring-focus \
                       disabled:cursor-not-allowed disabled:opacity-50 cursor-pointer transition-colors";
    let wrapper_base = "relative w-full";
    (classes!(select_base), classes!(wrapper_base, extra.clone()))
}
