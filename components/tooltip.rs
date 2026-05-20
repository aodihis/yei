use yew::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub enum TooltipPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Properties, PartialEq)]
pub struct TooltipProps {
    // --- Content ---
    #[prop_or_default]
    pub children: Children,
    pub tip: AttrValue,

    // --- Style ---
    #[prop_or_default]
    pub position: TooltipPosition,
    #[prop_or_default]
    pub class: Classes,
}

pub enum TooltipMsg {}
pub struct Tooltip;

impl Component for Tooltip {
    type Message = TooltipMsg;
    type Properties = TooltipProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let wrapper_base = "relative inline-block group";
        let wrapper = classes!(wrapper_base, p.class.clone());
        let tip_cls = tooltip_tip_classes(&p.position);
        html! {
            <span class={wrapper}>
                { for p.children.iter() }
                <span class={tip_cls} role="tooltip">
                    { p.tip.clone() }
                </span>
            </span>
        }
    }
}

fn tooltip_tip_classes(pos: &TooltipPosition) -> Classes {
    let base = "pointer-events-none absolute z-50 px-2 py-1 text-xs rounded-md \
                bg-foreground text-background whitespace-nowrap \
                opacity-0 group-hover:opacity-100 transition-opacity duration-150";
    let pos_cls = match pos {
        TooltipPosition::Top    => "bottom-full left-1/2 -translate-x-1/2 mb-1.5",
        TooltipPosition::Bottom => "top-full left-1/2 -translate-x-1/2 mt-1.5",
        TooltipPosition::Left   => "right-full top-1/2 -translate-y-1/2 mr-1.5",
        TooltipPosition::Right  => "left-full top-1/2 -translate-y-1/2 ml-1.5",
    };
    classes!(base, pos_cls)
}
