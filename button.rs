use yew::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
    Muted,
    Outline,
    Link,
}

#[derive(Clone, PartialEq, Default)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    // --- Content ---
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub href: Option<AttrValue>,

    // --- State ---
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or(AttrValue::from("button"))]
    pub r#type: AttrValue,

    // --- Style ---
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: ButtonSize,
    #[prop_or_default]
    pub class: Classes,

    // --- Callbacks ---
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

pub enum ButtonMsg {}

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

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let cls = button_classes(&p.variant, &p.size, &p.class);

        match &p.href {
            Some(href) if !p.disabled => html! {
                <a href={href.clone()} class={cls} onclick={p.onclick.clone()}>
                    { for p.children.iter() }
                </a>
            },
            Some(_) => {
                let disabled_cls = classes!(cls, "pointer-events-none", "opacity-50");
                html! {
                    <a class={disabled_cls} aria-disabled="true">
                        { for p.children.iter() }
                    </a>
                }
            },
            None => html! {
                <button
                    type={p.r#type.clone()}
                    class={cls}
                    disabled={p.disabled}
                    onclick={p.onclick.clone()}
                >
                    { for p.children.iter() }
                </button>
            },
        }
    }
}

fn button_classes(variant: &ButtonVariant, size: &ButtonSize, extra: &Classes) -> Classes {
    let base = "inline-flex items-center justify-center gap-2 rounded-lg text-sm font-medium \
                transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus \
                disabled:pointer-events-none disabled:opacity-50";

    let variant_cls = match variant {
        ButtonVariant::Primary   => "bg-primary text-primary-foreground hover:bg-primary/90",
        ButtonVariant::Danger    => "bg-danger text-on-danger hover:bg-danger/90",
        ButtonVariant::Outline   => "border border-border bg-background hover:bg-muted hover:text-muted-foreground",
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        ButtonVariant::Muted     => "hover:bg-muted hover:text-muted-foreground",
        ButtonVariant::Link      => "text-primary underline-offset-4 hover:underline",
    };

    let size_cls = match size {
        ButtonSize::Default => "h-10 px-4 py-2",
        ButtonSize::Sm      => "h-9 px-3 text-xs",
        ButtonSize::Lg      => "h-11 px-8",
        ButtonSize::Icon    => "size-10",
    };

    classes!(base, variant_cls, size_cls, extra.clone())
}
