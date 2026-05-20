use yew::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub enum BadgeVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
    Muted,
    Outline,
}

#[derive(Properties, PartialEq)]
pub struct BadgeProps {
    // --- Content ---
    #[prop_or_default]
    pub children: Children,

    // --- Style ---
    #[prop_or_default]
    pub variant: BadgeVariant,
    #[prop_or_default]
    pub class: Classes,
}

pub enum BadgeMsg {}

pub struct Badge;

impl Component for Badge {
    type Message = BadgeMsg;
    type Properties = BadgeProps;

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
        let cls = badge_classes(&p.variant, &p.class);
        html! {
            <span class={cls}>
                { for p.children.iter() }
            </span>
        }
    }
}

fn badge_classes(variant: &BadgeVariant, extra: &Classes) -> Classes {
    let base = "inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium";
    let variant_cls = match variant {
        BadgeVariant::Primary   => "bg-primary text-primary-foreground",
        BadgeVariant::Secondary => "bg-secondary text-secondary-foreground border border-border",
        BadgeVariant::Danger    => "bg-danger text-on-danger",
        BadgeVariant::Muted     => "bg-muted text-muted-foreground",
        BadgeVariant::Outline   => "border border-border text-foreground bg-transparent",
    };
    classes!(base, variant_cls, extra.clone())
}
