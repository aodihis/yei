use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SeparatorProps {
    // --- State ---
    #[prop_or_default]
    pub vertical: bool,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,
}

pub enum SeparatorMsg {}
pub struct Separator;

impl Component for Separator {
    type Message = SeparatorMsg;
    type Properties = SeparatorProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let cls = separator_classes(p.vertical, &p.class);
        let orientation = if p.vertical { "vertical" } else { "horizontal" };
        html! {
            <div role="separator" aria-orientation={orientation} class={cls} />
        }
    }
}

fn separator_classes(vertical: bool, extra: &Classes) -> Classes {
    let base = if vertical {
        "inline-block self-stretch w-px bg-border"
    } else {
        "block w-full h-px bg-border"
    };
    classes!(base, extra.clone())
}
