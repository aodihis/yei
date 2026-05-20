use yew::prelude::*;
use crate::components::label::Label;

#[derive(Properties, PartialEq)]
pub struct FormFieldProps {
    // --- Content ---
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub label: AttrValue,
    #[prop_or_default]
    pub description: AttrValue,
    #[prop_or_default]
    pub error: AttrValue,
    #[prop_or_default]
    pub r#for: AttrValue,

    // --- State ---
    #[prop_or_default]
    pub horizontal: bool,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub label_class: Classes,
}

pub enum FormFieldMsg {}

pub struct FormField;

impl Component for FormField {
    type Message = FormFieldMsg;
    type Properties = FormFieldProps;

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

        let description = if !p.description.is_empty() {
            html! { <p class="text-sm text-foreground/60">{ p.description.clone() }</p> }
        } else {
            html! {}
        };

        let error = if !p.error.is_empty() {
            html! { <p class="text-sm text-danger">{ p.error.clone() }</p> }
        } else {
            html! {}
        };

        if p.horizontal {
            let cls = classes!("flex", "items-start", "gap-4", p.class.clone());
            let lbl_cls = classes!("w-32", "shrink-0", "pt-2", p.label_class.clone());
            html! {
                <div class={cls}>
                    if !p.label.is_empty() {
                        <Label r#for={p.r#for.clone()} class={lbl_cls}>
                            { p.label.clone() }
                        </Label>
                    }
                    <div class="flex-1 space-y-1">
                        { for p.children.iter() }
                        { description }
                        { error }
                    </div>
                </div>
            }
        } else {
            let cls = classes!("space-y-2", p.class.clone());
            html! {
                <div class={cls}>
                    if !p.label.is_empty() {
                        <Label r#for={p.r#for.clone()} class={p.label_class.clone()}>
                            { p.label.clone() }
                        </Label>
                    }
                    { for p.children.iter() }
                    { description }
                    { error }
                </div>
            }
        }
    }
}
