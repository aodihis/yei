use yew::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub enum InputType {
    #[default]
    Text,
    Email,
    Password,
    Number,
    Tel,
    Url,
}

impl InputType {
    fn as_str(&self) -> &'static str {
        match self {
            InputType::Text     => "text",
            InputType::Email    => "email",
            InputType::Password => "password",
            InputType::Number   => "number",
            InputType::Tel      => "tel",
            InputType::Url      => "url",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct InputProps {
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
    #[prop_or_default]
    pub r#type: InputType,

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
    #[prop_or_default]
    pub onfocus: Callback<FocusEvent>,
    #[prop_or_default]
    pub onblur: Callback<FocusEvent>,
}

pub enum InputMsg {}

pub struct Input;

impl Component for Input {
    type Message = InputMsg;
    type Properties = InputProps;

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
        let cls = input_classes(p.error, &p.class);
        html! {
            <input
                id={p.id.clone()}
                type={p.r#type.as_str()}
                class={cls}
                value={p.value.clone()}
                placeholder={p.placeholder.clone()}
                disabled={p.disabled}
                oninput={p.oninput.clone()}
                onchange={p.onchange.clone()}
                onfocus={p.onfocus.clone()}
                onblur={p.onblur.clone()}
            />
        }
    }
}

fn input_classes(error: bool, extra: &Classes) -> Classes {
    let base = "flex h-10 w-full rounded-lg border border-transparent bg-field px-3 py-2 text-sm \
                placeholder:text-foreground/40 \
                focus-visible:outline-none focus-visible:border-primary \
                focus-visible:ring-1 focus-visible:ring-focus \
                disabled:cursor-not-allowed disabled:opacity-50 transition-colors";

    let state_cls = if error {
        "border-danger/60 focus-visible:border-danger focus-visible:ring-danger"
    } else {
        ""
    };

    classes!(base, state_cls, extra.clone())
}
