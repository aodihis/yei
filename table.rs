use yew::prelude::*;

// --- Table ---

#[derive(Properties, PartialEq)]
pub struct TableProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

pub enum TableMsg {}
pub struct Table;

impl Component for Table {
    type Message = TableMsg;
    type Properties = TableProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let outer = "w-full overflow-auto rounded-lg border border-border";
        let inner = classes!("w-full", "text-sm", p.class.clone());
        html! {
            <div class={outer}>
                <table class={inner}>
                    { for p.children.iter() }
                </table>
            </div>
        }
    }
}

// --- TableHeader (thead) ---

#[derive(Properties, PartialEq)]
pub struct TableHeaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

pub enum TableHeaderMsg {}
pub struct TableHeader;

impl Component for TableHeader {
    type Message = TableHeaderMsg;
    type Properties = TableHeaderProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let cls = classes!("border-b", "border-border", "bg-muted/50", p.class.clone());
        html! { <thead class={cls}>{ for p.children.iter() }</thead> }
    }
}

// --- TableBody (tbody) ---

#[derive(Properties, PartialEq)]
pub struct TableBodyProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

pub enum TableBodyMsg {}
pub struct TableBody;

impl Component for TableBody {
    type Message = TableBodyMsg;
    type Properties = TableBodyProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let cls = classes!("divide-y", "divide-border", p.class.clone());
        html! { <tbody class={cls}>{ for p.children.iter() }</tbody> }
    }
}

// --- TableRow (tr) ---

#[derive(Properties, PartialEq)]
pub struct TableRowProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

pub enum TableRowMsg {}
pub struct TableRow;

impl Component for TableRow {
    type Message = TableRowMsg;
    type Properties = TableRowProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let cls = classes!("hover:bg-muted/50", "transition-colors", p.class.clone());
        html! { <tr class={cls}>{ for p.children.iter() }</tr> }
    }
}

// --- TableHead (th) ---

#[derive(Properties, PartialEq)]
pub struct TableHeadProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

pub enum TableHeadMsg {}
pub struct TableHead;

impl Component for TableHead {
    type Message = TableHeadMsg;
    type Properties = TableHeadProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let base = "px-4 py-3 text-left text-xs font-semibold text-foreground/60 uppercase tracking-wide";
        let cls = classes!(base, p.class.clone());
        html! { <th class={cls}>{ for p.children.iter() }</th> }
    }
}

// --- TableCell (td) ---

#[derive(Properties, PartialEq)]
pub struct TableCellProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

pub enum TableCellMsg {}
pub struct TableCell;

impl Component for TableCell {
    type Message = TableCellMsg;
    type Properties = TableCellProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let cls = classes!("px-4", "py-3", "text-foreground", p.class.clone());
        html! { <td class={cls}>{ for p.children.iter() }</td> }
    }
}
