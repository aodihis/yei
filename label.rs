use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LabelProps {
    // --- Content ---
    #[prop_or_default]
    pub children: Children,

    // --- State ---
    #[prop_or_default]
    pub r#for: AttrValue,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,
}

pub enum LabelMsg {}

pub struct Label;

impl Component for Label {
    type Message = LabelMsg;
    type Properties = LabelProps;

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
        let cls = label_classes(&p.class);
        html! {
            <label for={p.r#for.clone()} class={cls}>
                { for p.children.iter() }
            </label>
        }
    }
}

fn label_classes(extra: &Classes) -> Classes {
    let base = "text-sm font-medium leading-none \
                peer-disabled:cursor-not-allowed peer-disabled:opacity-70";
    classes!(base, extra.clone())
}
