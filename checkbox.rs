use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CheckboxProps {
    // --- State ---
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub value: AttrValue,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,

    // --- Callbacks ---
    #[prop_or_default]
    pub onchange: Callback<Event>,
}

pub enum CheckboxMsg {}

pub struct Checkbox;

impl Component for Checkbox {
    type Message = CheckboxMsg;
    type Properties = CheckboxProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let cls = checkbox_classes(&p.class);
        html! {
            <input
                type="checkbox"
                id={p.id.clone()}
                name={p.name.clone()}
                value={p.value.clone()}
                class={cls}
                checked={p.checked}
                disabled={p.disabled}
                onchange={p.onchange.clone()}
            />
        }
    }
}

fn checkbox_classes(extra: &Classes) -> Classes {
    let base = "h-4 w-4 rounded border border-primary accent-primary \
                cursor-pointer disabled:cursor-not-allowed disabled:opacity-50 \
                focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus";
    classes!(base, extra.clone())
}
