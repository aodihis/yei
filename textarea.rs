use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextareaProps {
    // --- Content ---
    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or_default]
    pub placeholder: AttrValue,

    // --- State ---
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or(4)]
    pub rows: u32,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub error: bool,

    // --- Callbacks ---
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub onchange: Callback<Event>,
}

pub enum TextareaMsg {}

pub struct Textarea;

impl Component for Textarea {
    type Message = TextareaMsg;
    type Properties = TextareaProps;

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
        let cls = textarea_classes(p.error, &p.class);
        html! {
            <textarea
                id={p.id.clone()}
                class={cls}
                rows={p.rows.to_string()}
                placeholder={p.placeholder.clone()}
                disabled={p.disabled}
                oninput={p.oninput.clone()}
                onchange={p.onchange.clone()}
            >
                { p.value.clone() }
            </textarea>
        }
    }
}

fn textarea_classes(error: bool, extra: &Classes) -> Classes {
    let base = "flex w-full rounded-lg border border-transparent bg-field px-3 py-2 text-sm \
                placeholder:text-foreground/40 \
                focus-visible:outline-none focus-visible:border-primary \
                focus-visible:ring-1 focus-visible:ring-focus \
                disabled:cursor-not-allowed disabled:opacity-50 resize-y transition-colors";

    let state_cls = if error {
        "border-danger/60 focus-visible:border-danger focus-visible:ring-danger"
    } else {
        ""
    };

    classes!(base, state_cls, extra.clone())
}
