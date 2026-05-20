use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RadioProps {
    // --- State ---
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or_default]
    pub id: AttrValue,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,

    // --- Callbacks ---
    #[prop_or_default]
    pub onchange: Callback<Event>,
}

pub enum RadioMsg {}

pub struct Radio;

impl Component for Radio {
    type Message = RadioMsg;
    type Properties = RadioProps;

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
        let cls = radio_classes(&p.class);
        html! {
            <input
                type="radio"
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

fn radio_classes(extra: &Classes) -> Classes {
    let base = "h-4 w-4 border border-primary accent-primary \
                cursor-pointer disabled:cursor-not-allowed disabled:opacity-50 \
                focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus";
    classes!(base, extra.clone())
}

// ---------------------------------------------------------------------------

#[derive(Properties, PartialEq)]
pub struct RadioGroupProps {
    // --- Content ---
    #[prop_or_default]
    pub children: Children,

    // --- State ---
    #[prop_or_default]
    pub name: AttrValue,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,
}

pub enum RadioGroupMsg {}

pub struct RadioGroup;

impl Component for RadioGroup {
    type Message = RadioGroupMsg;
    type Properties = RadioGroupProps;

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
        let cls = radio_group_classes(&p.class);
        html! {
            <div role="radiogroup" class={cls}>
                { for p.children.iter() }
            </div>
        }
    }
}

fn radio_group_classes(extra: &Classes) -> Classes {
    let base = "flex flex-col gap-2";
    classes!(base, extra.clone())
}
