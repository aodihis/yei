use yew::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub enum SpinnerSize {
    Sm,
    #[default]
    Default,
    Lg,
}

#[derive(Properties, PartialEq)]
pub struct SpinnerProps {
    // --- Style ---
    #[prop_or_default]
    pub size: SpinnerSize,
    #[prop_or_default]
    pub class: Classes,
}

pub enum SpinnerMsg {}
pub struct Spinner;

impl Component for Spinner {
    type Message = SpinnerMsg;
    type Properties = SpinnerProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let size_cls = match p.size {
            SpinnerSize::Sm      => "size-4",
            SpinnerSize::Default => "size-6",
            SpinnerSize::Lg      => "size-8",
        };
        let cls = classes!("animate-[spin_0.75s_linear_infinite]", size_cls, p.class.clone());
        html! {
            <svg
                class={cls}
                viewBox="0 0 24 24"
                fill="none"
                aria-label="Loading"
                role="status"
            >
                // Track ring
                <circle
                    cx="12" cy="12" r="10"
                    stroke="currentColor"
                    stroke-width="3"
                    class="opacity-20"
                />
                // Spinning arc
                <path
                    d="M12 2a10 10 0 0 1 10 10"
                    stroke="currentColor"
                    stroke-width="3"
                    stroke-linecap="round"
                />
            </svg>
        }
    }
}
