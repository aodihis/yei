use yew::prelude::*;
use crate::components::calendar::Calendar;
use crate::components::icons::icon_calendar;

#[derive(Properties, PartialEq)]
pub struct DateInputProps {
    // --- Content ---
    #[prop_or_default]
    pub selected: Option<AttrValue>,
    #[prop_or_default]
    pub placeholder: AttrValue,

    // --- State ---
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub disabled: bool,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub error: bool,

    // --- Callbacks ---
    #[prop_or_default]
    pub onselect: Callback<AttrValue>,
}

pub enum DateInputMsg {
    Toggle,
    SelectDate(AttrValue),
}

pub struct DateInput {
    open: bool,
}

impl Component for DateInput {
    type Message = DateInputMsg;
    type Properties = DateInputProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { open: false }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DateInputMsg::Toggle => {
                if !ctx.props().disabled {
                    self.open = !self.open;
                    true
                } else {
                    false
                }
            }
            DateInputMsg::SelectDate(date) => {
                ctx.props().onselect.emit(date);
                self.open = false;
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let link = ctx.link();

        let display = p.selected.as_ref().map(|s| format_date(s.as_str()))
            .unwrap_or_default();

        let input_cls = date_input_classes(p.error, p.disabled, &p.class);

        let on_select = link.callback(DateInputMsg::SelectDate);
        let on_toggle = link.callback(|_| DateInputMsg::Toggle);

        html! {
            <div class="relative inline-block w-full">
                <button
                    id={p.id.clone()}
                    type="button"
                    class={input_cls}
                    disabled={p.disabled}
                    onclick={on_toggle}
                >
                    <span class={if display.is_empty() { "text-foreground/40" } else { "text-foreground" }}>
                        { if display.is_empty() {
                            let ph = if p.placeholder.is_empty() {
                                AttrValue::from("Pick a date")
                            } else {
                                p.placeholder.clone()
                            };
                            html! { {ph} }
                        } else {
                            html! { {display} }
                        }}
                    </span>
                    <span class="ml-auto text-foreground/50">{ icon_calendar() }</span>
                </button>

                if self.open {
                    <div class="absolute z-50 mt-1 left-0">
                        <Calendar
                            selected={p.selected.clone()}
                            onselect={on_select}
                        />
                    </div>
                }
            </div>
        }
    }
}

fn date_input_classes(error: bool, disabled: bool, extra: &Classes) -> Classes {
    let base = "flex h-10 w-full items-center rounded-lg border border-transparent bg-field \
                px-3 py-2 text-sm text-left \
                focus-visible:outline-none focus-visible:border-primary \
                focus-visible:ring-1 focus-visible:ring-focus \
                transition-colors cursor-pointer";

    let state_cls = if error {
        "border-danger/60 focus-visible:border-danger focus-visible:ring-danger"
    } else {
        ""
    };

    let disabled_cls = if disabled {
        "opacity-50 cursor-not-allowed pointer-events-none"
    } else {
        ""
    };

    classes!(base, state_cls, disabled_cls, extra.clone())
}

fn format_date(s: &str) -> String {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return s.to_string();
    }
    let month_names = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December",
    ];
    let month_idx: usize = parts[1].parse::<usize>().unwrap_or(1).saturating_sub(1);
    let month = month_names.get(month_idx).copied().unwrap_or("");
    let day: u32 = parts[2].parse().unwrap_or(0);
    format!("{} {}, {}", month, day, parts[0])
}
