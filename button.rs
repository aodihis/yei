use yew::prelude::*;

// --- Variants ---

#[derive(Clone, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

// --- Sizes ---

#[derive(Clone, PartialEq, Default)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
}

// --- Props ---

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    // Content
    #[prop_or_default]
    pub children: Children,

    // Behavior
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or(AttrValue::from("button"))]
    pub r#type: AttrValue,

    // Appearance
    #[prop_or_default]
    pub variant: ButtonVariant,

    #[prop_or_default]
    pub size: ButtonSize,

    #[prop_or_default]
    pub class: Classes,

    // Callbacks
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

// --- Message ---

pub enum ButtonMsg {}

// --- Component ---

pub struct Button;

impl Component for Button {
    type Message = ButtonMsg;
    type Properties = ButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let classes = button_classes(&props.variant, &props.size, &props.class);
        let onclick = props.onclick.clone();

        html! {
            <button
                type={props.r#type.clone()}
                class={classes}
                disabled={props.disabled}
                onclick={onclick}
            >
                { for props.children.iter() }
            </button>
        }
    }
}

// --- Class resolver ---

fn button_classes(variant: &ButtonVariant, size: &ButtonSize, extra: &Classes) -> Classes {
    let base = "inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium \
                transition-colors focus-visible:outline-none focus-visible:ring-2 \
                focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

    let variant_cls = match variant {
        ButtonVariant::Default => {
            "bg-primary text-primary-foreground hover:bg-primary/90"
        }
        ButtonVariant::Destructive => {
            "bg-destructive text-destructive-foreground hover:bg-destructive/90"
        }
        ButtonVariant::Outline => {
            "border border-input bg-background hover:bg-accent hover:text-accent-foreground"
        }
        ButtonVariant::Secondary => {
            "bg-secondary text-secondary-foreground hover:bg-secondary/80"
        }
        ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
    };

    let size_cls = match size {
        ButtonSize::Default => "h-10 px-4 py-2",
        ButtonSize::Sm => "h-9 px-3 text-xs",
        ButtonSize::Lg => "h-11 px-8",
        ButtonSize::Icon => "size-10",
    };

    classes!(base, variant_cls, size_cls, extra.clone())
}
