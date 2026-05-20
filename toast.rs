use yew::prelude::*;
use crate::components::icons::icon_x;

// --- Variant ---

#[derive(Clone, PartialEq, Default)]
pub enum ToastVariant {
    #[default]
    Default,
    Success,
    Warning,
    Danger,
}

// --- Item ---

#[derive(Clone, PartialEq)]
pub struct ToastItem {
    pub id: u32,
    pub title: AttrValue,
    pub description: Option<AttrValue>,
    pub variant: ToastVariant,
    pub duration_ms: u32,
}

// --- Context ---

#[derive(Clone, PartialEq)]
pub struct ToastContext {
    pub push: Callback<ToastItem>,
}

#[hook]
pub fn use_toast() -> ToastContext {
    use_context::<ToastContext>().expect("use_toast must be called inside ToastProvider")
}

// --- ToastProvider (function component — needs hooks for state) ---

#[derive(Properties, PartialEq)]
pub struct ToastProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ToastProvider)]
pub fn toast_provider(props: &ToastProviderProps) -> Html {
    let toasts  = use_state(Vec::<ToastItem>::new);
    let counter = use_mut_ref(|| 0u32);

    let push = {
        let toasts  = toasts.clone();
        let counter = counter.clone();
        Callback::from(move |mut item: ToastItem| {
            let id = {
                let mut n = counter.borrow_mut();
                let id = *n;
                *n = n.wrapping_add(1);
                id
            };
            item.id = id;
            let mut list = (*toasts).clone();
            list.push(item);
            toasts.set(list);
        })
    };

    let dismiss = {
        let toasts = toasts.clone();
        Callback::from(move |id: u32| {
            let list: Vec<ToastItem> = (*toasts).iter().filter(|t| t.id != id).cloned().collect();
            toasts.set(list);
        })
    };

    let ctx = ToastContext { push };

    html! {
        <ContextProvider<ToastContext> context={ctx}>
            { for props.children.iter() }
            <ToastContainer toasts={(*toasts).clone()} on_dismiss={dismiss} />
        </ContextProvider<ToastContext>>
    }
}

// --- ToastContainer ---

#[derive(Properties, PartialEq)]
pub struct ToastContainerProps {
    pub toasts: Vec<ToastItem>,
    pub on_dismiss: Callback<u32>,
}

pub enum ToastContainerMsg {}
pub struct ToastContainer;

impl Component for ToastContainer {
    type Message = ToastContainerMsg;
    type Properties = ToastContainerProps;

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        if p.toasts.is_empty() {
            return html! {};
        }
        let container = "fixed top-4 right-4 z-[200] flex flex-col gap-2 w-80";
        html! {
            <div class={container} aria-live="polite" aria-label="Notifications">
                { for p.toasts.iter().map(|t| html! {
                    <ToastComp
                        key={t.id}
                        item={t.clone()}
                        on_dismiss={p.on_dismiss.clone()}
                    />
                }) }
            </div>
        }
    }
}

// --- Individual Toast (struct component — holds timeout handle) ---

pub struct ToastComp {
    _timeout: gloo_timers::callback::Timeout,
}

#[derive(Properties, PartialEq)]
pub struct ToastCompProps {
    pub item: ToastItem,
    pub on_dismiss: Callback<u32>,
}

impl Component for ToastComp {
    type Message = ();
    type Properties = ToastCompProps;

    fn create(ctx: &Context<Self>) -> Self {
        let p = ctx.props();
        let cb = p.on_dismiss.clone();
        let id = p.item.id;
        let timeout = gloo_timers::callback::Timeout::new(p.item.duration_ms, move || {
            cb.emit(id);
        });
        Self { _timeout: timeout }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: ()) -> bool { false }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let cls = toast_item_classes(&p.item.variant);
        let on_close = {
            let cb = p.on_dismiss.clone();
            let id = p.item.id;
            Callback::from(move |_: MouseEvent| cb.emit(id))
        };
        html! {
            <div class={cls} role="status">
                <div class="flex-1 min-w-0">
                    <p class="font-medium leading-snug">{ &p.item.title }</p>
                    if let Some(desc) = &p.item.description {
                        <p class="text-xs mt-0.5 opacity-75">{ desc }</p>
                    }
                </div>
                <button
                    onclick={on_close}
                    aria-label="Dismiss"
                    class="shrink-0 mt-0.5 p-0.5 rounded opacity-50 hover:opacity-100 transition-opacity"
                >
                    { icon_x() }
                </button>
            </div>
        }
    }
}

fn toast_item_classes(variant: &ToastVariant) -> Classes {
    let base = "animate-toast-in flex items-start gap-3 p-4 rounded-lg border shadow-lg text-sm";
    let variant_cls = match variant {
        ToastVariant::Default => "bg-background border-border text-foreground",
        ToastVariant::Success => "bg-success/10 border-success/30 text-success",
        ToastVariant::Warning => "bg-warning/10 border-warning/30 text-warning",
        ToastVariant::Danger  => "bg-danger/10 border-danger/30 text-danger",
    };
    classes!(base, variant_cls)
}
