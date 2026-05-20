use yew::prelude::*;

// --- Card ---

#[derive(Properties, PartialEq)]
pub struct CardProps {
    // --- Content ---
    #[prop_or_default]
    pub children: Children,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,
}

pub enum CardMsg {}
pub struct Card;

impl Component for Card {
    type Message = CardMsg;
    type Properties = CardProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let base = "rounded-lg border border-border bg-background shadow-sm";
        let cls = classes!(base, p.class.clone());
        html! {
            <div class={cls}>
                { for p.children.iter() }
            </div>
        }
    }
}

// --- CardHeader ---

#[derive(Properties, PartialEq)]
pub struct CardHeaderProps {
    // --- Content ---
    #[prop_or_default]
    pub children: Children,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,
}

pub enum CardHeaderMsg {}
pub struct CardHeader;

impl Component for CardHeader {
    type Message = CardHeaderMsg;
    type Properties = CardHeaderProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let base = "flex flex-col gap-1 px-6 pt-6 pb-4";
        let cls = classes!(base, p.class.clone());
        html! {
            <div class={cls}>
                { for p.children.iter() }
            </div>
        }
    }
}

// --- CardContent ---

#[derive(Properties, PartialEq)]
pub struct CardContentProps {
    // --- Content ---
    #[prop_or_default]
    pub children: Children,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,
}

pub enum CardContentMsg {}
pub struct CardContent;

impl Component for CardContent {
    type Message = CardContentMsg;
    type Properties = CardContentProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let base = "px-6 pb-6";
        let cls = classes!(base, p.class.clone());
        html! {
            <div class={cls}>
                { for p.children.iter() }
            </div>
        }
    }
}

// --- CardFooter ---

#[derive(Properties, PartialEq)]
pub struct CardFooterProps {
    // --- Content ---
    #[prop_or_default]
    pub children: Children,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,
}

pub enum CardFooterMsg {}
pub struct CardFooter;

impl Component for CardFooter {
    type Message = CardFooterMsg;
    type Properties = CardFooterProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let base = "flex items-center gap-2 px-6 pb-6";
        let cls = classes!(base, p.class.clone());
        html! {
            <div class={cls}>
                { for p.children.iter() }
            </div>
        }
    }
}
