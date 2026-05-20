use yew::prelude::*;
use crate::components::icons::{icon_chevron_left, icon_chevron_right};

#[derive(Clone, PartialEq)]
enum ViewMode {
    Days,
    Months,
    Years,
}

#[derive(Properties, PartialEq)]
pub struct CalendarProps {
    // --- Content ---
    #[prop_or_default]
    pub selected: Option<AttrValue>,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,

    // --- Callbacks ---
    #[prop_or_default]
    pub onselect: Callback<AttrValue>,
}

pub enum CalendarMsg {
    PrevMonth,
    NextMonth,
    Prev,
    Next,
    SelectDay(u32),
    SelectMonth(u32),
    SelectYear(i32),
    ShowMonthPicker,
    ShowYearPicker,
}

pub struct Calendar {
    year:      i32,
    month:     u32,
    mode:      ViewMode,
    year_page: i32,
}

impl Component for Calendar {
    type Message = CalendarMsg;
    type Properties = CalendarProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (year, month) = ctx
            .props()
            .selected
            .as_ref()
            .and_then(|s| parse_year_month(s.as_str()))
            .unwrap_or((2025, 1));
        Self { year, month, mode: ViewMode::Days, year_page: year_page_start(year) }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CalendarMsg::PrevMonth => {
                if self.month == 1 { self.month = 12; self.year -= 1; }
                else               { self.month -= 1; }
                true
            }
            CalendarMsg::NextMonth => {
                if self.month == 12 { self.month = 1; self.year += 1; }
                else                { self.month += 1; }
                true
            }
            // Prev/Next meaning depends on the active view
            CalendarMsg::Prev => {
                match self.mode {
                    ViewMode::Days   => unreachable!(),
                    ViewMode::Months => { self.year -= 1; true }
                    ViewMode::Years  => { self.year_page -= 12; true }
                }
            }
            CalendarMsg::Next => {
                match self.mode {
                    ViewMode::Days   => unreachable!(),
                    ViewMode::Months => { self.year += 1; true }
                    ViewMode::Years  => { self.year_page += 12; true }
                }
            }
            CalendarMsg::SelectDay(day) => {
                let date = format!("{:04}-{:02}-{:02}", self.year, self.month, day);
                ctx.props().onselect.emit(AttrValue::from(date));
                false
            }
            CalendarMsg::SelectMonth(m) => {
                self.month = m;
                self.mode  = ViewMode::Days;
                true
            }
            CalendarMsg::SelectYear(y) => {
                self.year      = y;
                self.year_page = year_page_start(y);
                self.mode      = ViewMode::Months;
                true
            }
            CalendarMsg::ShowMonthPicker => {
                self.mode = ViewMode::Months;
                true
            }
            CalendarMsg::ShowYearPicker => {
                self.year_page = year_page_start(self.year);
                self.mode      = ViewMode::Years;
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let cls = calendar_classes(&p.class);
        html! {
            <div class={cls}>
                { self.view_header(ctx) }
                { match self.mode {
                    ViewMode::Days   => self.view_days(ctx),
                    ViewMode::Months => view_month_grid(ctx, self.month),
                    ViewMode::Years  => view_year_grid(ctx, self.year, self.year_page),
                }}
            </div>
        }
    }
}

impl Calendar {
    fn view_header(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let nav_cls    = "p-1 rounded hover:bg-muted transition-colors text-sm";
        let picker_cls = "text-sm font-semibold py-0.5 rounded hover:bg-muted transition-colors";

        match self.mode {
            ViewMode::Days => html! {
                <div class="flex items-center justify-between mb-4">
                    <button type="button" class={nav_cls}
                        onclick={link.callback(|_| CalendarMsg::PrevMonth)}>
                        { icon_chevron_left() }
                    </button>
                    <div class="flex items-center gap-1">
                        <button type="button" class={picker_cls}
                            onclick={link.callback(|_| CalendarMsg::ShowMonthPicker)}>
                            { month_name(self.month) }
                        </button>
                        <button type="button" class={picker_cls}
                            onclick={link.callback(|_| CalendarMsg::ShowYearPicker)}>
                            { self.year }
                        </button>
                    </div>
                    <button type="button" class={nav_cls}
                        onclick={link.callback(|_| CalendarMsg::NextMonth)}>
                        { icon_chevron_right() }
                    </button>
                </div>
            },
            ViewMode::Months => html! {
                <div class="flex items-center justify-between mb-4">
                    <button type="button" class={nav_cls}
                        onclick={link.callback(|_| CalendarMsg::Prev)}>
                        { icon_chevron_left() }
                    </button>
                    <button type="button" class={picker_cls}
                        onclick={link.callback(|_| CalendarMsg::ShowYearPicker)}>
                        { self.year }
                    </button>
                    <button type="button" class={nav_cls}
                        onclick={link.callback(|_| CalendarMsg::Next)}>
                        { icon_chevron_right() }
                    </button>
                </div>
            },
            ViewMode::Years => {
                let end = self.year_page + 11;
                html! {
                    <div class="flex items-center justify-between mb-4">
                        <button type="button" class={nav_cls}
                            onclick={link.callback(|_| CalendarMsg::Prev)}>
                            { icon_chevron_left() }
                        </button>
                        <span class="text-sm font-semibold">
                            { format!("{} – {}", self.year_page, end) }
                        </span>
                        <button type="button" class={nav_cls}
                            onclick={link.callback(|_| CalendarMsg::Next)}>
                            { icon_chevron_right() }
                        </button>
                    </div>
                }
            },
        }
    }

    fn view_days(&self, ctx: &Context<Self>) -> Html {
        let p        = ctx.props();
        let days     = days_in_month(self.year, self.month);
        let first_wd = first_weekday(self.year, self.month);

        let selected_day: Option<u32> = p.selected.as_ref().and_then(|s| {
            let parts: Vec<&str> = s.split('-').collect();
            if parts.len() == 3 {
                let y: i32 = parts[0].parse().ok()?;
                let m: u32 = parts[1].parse().ok()?;
                let d: u32 = parts[2].parse().ok()?;
                if y == self.year && m == self.month { Some(d) } else { None }
            } else {
                None
            }
        });

        html! {
            <>
                { view_week_headers() }
                { view_day_grid(ctx, days, first_wd, selected_day) }
            </>
        }
    }
}

fn view_week_headers() -> Html {
    html! {
        <div class="grid grid-cols-7 mb-2">
            { for ["Su","Mo","Tu","We","Th","Fr","Sa"].iter().map(|d| html! {
                <div class="text-center text-xs text-foreground/50 font-medium py-1">{ d }</div>
            }) }
        </div>
    }
}

fn view_day_grid(ctx: &Context<Calendar>, days: u32, first_wd: u32, selected_day: Option<u32>) -> Html {
    let link = ctx.link();
    html! {
        <div class="grid grid-cols-7 gap-1">
            { for (0..first_wd).map(|_| html! { <div /> }) }
            { for (1..=days).map(|day| {
                let cls = day_classes(selected_day == Some(day));
                let cb  = link.callback(move |_| CalendarMsg::SelectDay(day));
                html! { <button type="button" class={cls} onclick={cb}>{ day }</button> }
            }) }
        </div>
    }
}

fn view_month_grid(ctx: &Context<Calendar>, current_month: u32) -> Html {
    let link   = ctx.link();
    let months = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];
    html! {
        <div class="grid grid-cols-3 gap-1">
            { for months.iter().enumerate().map(|(i, &name)| {
                let m   = (i as u32) + 1;
                let cls = picker_cell_classes(m == current_month);
                let cb  = link.callback(move |_| CalendarMsg::SelectMonth(m));
                html! { <button type="button" class={cls} onclick={cb}>{ name }</button> }
            }) }
        </div>
    }
}

fn view_year_grid(ctx: &Context<Calendar>, current_year: i32, page_start: i32) -> Html {
    let link = ctx.link();
    html! {
        <div class="grid grid-cols-3 gap-1">
            { for (0..12).map(|i| {
                let y   = page_start + i;
                let cls = picker_cell_classes(y == current_year);
                let cb  = link.callback(move |_| CalendarMsg::SelectYear(y));
                html! { <button type="button" class={cls} onclick={cb}>{ y }</button> }
            }) }
        </div>
    }
}

fn calendar_classes(extra: &Classes) -> Classes {
    let base = "inline-block w-72 rounded-lg border border-border bg-background p-4 shadow-sm";
    classes!(base, extra.clone())
}

fn day_classes(selected: bool) -> Classes {
    let base = "h-8 w-8 rounded-md text-sm flex items-center justify-center transition-colors cursor-pointer hover:bg-muted";
    let sel  = "bg-primary text-primary-foreground hover:bg-primary/90";
    if selected { classes!(base, sel) } else { classes!(base) }
}

fn picker_cell_classes(active: bool) -> Classes {
    let base = "rounded-md py-2 text-sm transition-colors cursor-pointer";
    let sel  = "bg-primary text-primary-foreground hover:bg-primary/90";
    let idle = "hover:bg-muted";
    if active { classes!(base, sel) } else { classes!(base, idle) }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { 29 } else { 28 },
        _ => 30,
    }
}

// Tomohiko Sakamoto's algorithm — returns 0=Sun … 6=Sat
fn first_weekday(year: i32, month: u32) -> u32 {
    let t: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if month < 3 { year - 1 } else { year };
    ((y + y/4 - y/100 + y/400 + t[month as usize - 1] + 1) % 7) as u32
}

fn month_name(month: u32) -> &'static str {
    match month {
        1  => "January",   2  => "February", 3  => "March",
        4  => "April",     5  => "May",       6  => "June",
        7  => "July",      8  => "August",    9  => "September",
        10 => "October",   11 => "November",  12 => "December",
        _  => "",
    }
}

fn year_page_start(year: i32) -> i32 {
    year - (year % 12)
}

fn parse_year_month(s: &str) -> Option<(i32, u32)> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() >= 2 {
        let y = parts[0].parse().ok()?;
        let m = parts[1].parse().ok()?;
        if (1..=12).contains(&m) { Some((y, m)) } else { None }
    } else {
        None
    }
}
