use std::rc::Rc;
use yew::prelude::*;
use crate::components::icons::icon_chevron_down;

#[derive(Clone, PartialEq)]
pub struct AccordionItem {
    pub title: AttrValue,
    pub content: AttrValue,
}

#[derive(Properties, PartialEq)]
pub struct AccordionProps {
    // --- Content ---
    #[prop_or_default]
    pub items: Rc<Vec<AccordionItem>>,

    // --- State ---
    #[prop_or_default]
    pub multiple: bool,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,
}

pub enum AccordionMsg {
    Toggle(usize),
}

pub struct Accordion {
    open: Vec<bool>,
}

impl Component for Accordion {
    type Message = AccordionMsg;
    type Properties = AccordionProps;

    fn create(ctx: &Context<Self>) -> Self {
        let open = vec![false; ctx.props().items.len()];
        Self { open }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AccordionMsg::Toggle(idx) => {
                let is_open = self.open.get(idx).copied().unwrap_or(false);
                if ctx.props().multiple {
                    if let Some(v) = self.open.get_mut(idx) {
                        *v = !is_open;
                    }
                } else {
                    for v in self.open.iter_mut() {
                        *v = false;
                    }
                    if let Some(v) = self.open.get_mut(idx) {
                        *v = !is_open;
                    }
                }
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old: &Self::Properties) -> bool {
        if ctx.props().items.len() != old.items.len() {
            self.open = vec![false; ctx.props().items.len()];
        }
        !Rc::ptr_eq(&ctx.props().items, &old.items)
            || ctx.props().multiple != old.multiple
            || ctx.props().class    != old.class
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let cls = accordion_classes(&p.class);

        html! {
            <div class={cls}>
                { for p.items.iter().enumerate().map(|(i, item)| {
                    let is_open = self.open.get(i).copied().unwrap_or(false);
                    view_item(ctx, i, item, is_open)
                }) }
            </div>
        }
    }
}

fn view_item(ctx: &Context<Accordion>, i: usize, item: &AccordionItem, is_open: bool) -> Html {
    let on_toggle = ctx.link().callback(move |_| AccordionMsg::Toggle(i));
    let trigger_cls = trigger_classes(is_open);
    let chevron_cls = chevron_classes(is_open);

    html! {
        <div class="border-b border-border last:border-b-0">
            <button
                type="button"
                class={trigger_cls}
                onclick={on_toggle}
            >
                <span>{ item.title.clone() }</span>
                <span class={chevron_cls}>{ icon_chevron_down() }</span>
            </button>
            if is_open {
                <div class="px-4 pb-4 pt-0 text-sm text-foreground/80">
                    { item.content.clone() }
                </div>
            }
        </div>
    }
}

fn accordion_classes(extra: &Classes) -> Classes {
    let base = "w-full rounded-md border border-border";
    classes!(base, extra.clone())
}

fn trigger_classes(open: bool) -> Classes {
    let base = "flex w-full items-center justify-between px-4 py-4 \
                text-sm font-medium transition-all \
                focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus";
    let open_cls = if open { "text-primary" } else { "" };
    classes!(base, open_cls)
}

fn chevron_classes(open: bool) -> Classes {
    let base = "inline-flex transition-transform duration-200";
    let rotated = if open { "" } else { "-rotate-90" };
    classes!(base, rotated)
}
