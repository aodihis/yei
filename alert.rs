use yew::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub enum AlertVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
    Muted,
}

#[derive(Properties, PartialEq)]
pub struct AlertProps {
    // --- Content ---
    #[prop_or_default]
    pub children: Children,

    // --- Style ---
    #[prop_or_default]
    pub variant: AlertVariant,
    #[prop_or_default]
    pub class: Classes,
}

pub enum AlertMsg {}

pub struct Alert;

impl Component for Alert {
    type Message = AlertMsg;
    type Properties = AlertProps;

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
        let cls = alert_classes(&p.variant, &p.class);
        html! {
            <div role="alert" class={cls}>
                { for p.children.iter() }
            </div>
        }
    }
}

fn alert_classes(variant: &AlertVariant, extra: &Classes) -> Classes {
    let base = "rounded-lg border px-4 py-3 text-sm";
    let variant_cls = match variant {
        AlertVariant::Primary   => "bg-primary/10 border-primary/20 text-foreground",
        AlertVariant::Secondary => "bg-secondary border-border text-foreground",
        AlertVariant::Danger    => "bg-danger/10 border-danger/30 text-danger",
        AlertVariant::Muted     => "bg-muted border-border text-muted-foreground",
    };
    classes!(base, variant_cls, extra.clone())
}
