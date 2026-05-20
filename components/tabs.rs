use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone)]
pub struct TabItem {
    pub label: AttrValue,
    pub content: Rc<Html>,
}

impl PartialEq for TabItem {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label && Rc::ptr_eq(&self.content, &other.content)
    }
}

impl TabItem {
    pub fn new(label: impl Into<AttrValue>, content: Html) -> Self {
        Self { label: label.into(), content: Rc::new(content) }
    }
}

#[derive(Properties, PartialEq)]
pub struct TabsProps {
    // --- Content ---
    pub items: Rc<Vec<TabItem>>,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,
}

pub enum TabsMsg {
    Select(usize),
}

pub struct Tabs {
    active: usize,
}

impl Component for Tabs {
    type Message = TabsMsg;
    type Properties = TabsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { active: 0 }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TabsMsg::Select(i) => {
                if i == self.active || i >= ctx.props().items.len() {
                    return false;
                }
                self.active = i;
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        // clamp active index if items shrank
        let len = ctx.props().items.len();
        if len > 0 && self.active >= len {
            self.active = len - 1;
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let active = self.active;

        let outer_base = "w-full";
        let outer = classes!(outer_base, p.class.clone());

        html! {
            <div class={outer}>
                // Tab bar
                <div class="flex border-b border-border" role="tablist">
                    { for p.items.iter().enumerate().map(|(i, item)| {
                        let on_click = ctx.link().callback(move |_: MouseEvent| TabsMsg::Select(i));
                        let is_active = i == active;
                        let tab_cls = tab_classes(is_active);
                        html! {
                            <button
                                key={i}
                                role="tab"
                                aria-selected={is_active.to_string()}
                                onclick={on_click}
                                class={tab_cls}
                            >
                                { item.label.clone() }
                            </button>
                        }
                    }) }
                </div>

                // Active panel
                if let Some(item) = p.items.get(active) {
                    <div role="tabpanel" class="pt-4">
                        { (*item.content).clone() }
                    </div>
                }
            </div>
        }
    }
}

fn tab_classes(active: bool) -> &'static str {
    if active {
        "px-4 py-2 text-sm font-medium border-b-2 border-primary text-primary -mb-px"
    } else {
        "px-4 py-2 text-sm font-medium text-foreground/50 hover:text-foreground border-b-2 border-transparent -mb-px transition-colors"
    }
}
