use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProgressProps {
    // --- State ---
    pub value: f64,          // 0.0 – 100.0
    #[prop_or(100.0f64)]
    pub max: f64,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,
}

pub enum ProgressMsg {}
pub struct Progress;

impl Component for Progress {
    type Message = ProgressMsg;
    type Properties = ProgressProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let pct = (p.value / p.max * 100.0).clamp(0.0, 100.0);
        let fill_style = format!("width: {pct:.2}%");

        let track_base = "w-full h-2 rounded-full bg-muted overflow-hidden";
        let track_cls = classes!(track_base, p.class.clone());

        html! {
            <div
                role="progressbar"
                aria-valuenow={format!("{:.0}", p.value)}
                aria-valuemin="0"
                aria-valuemax={format!("{:.0}", p.max)}
                class={track_cls}
            >
                <div
                    class="h-full bg-primary rounded-full transition-all duration-300"
                    style={fill_style}
                />
            </div>
        }
    }
}
