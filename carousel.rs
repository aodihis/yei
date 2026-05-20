use std::rc::Rc;
use gloo_timers::callback::Interval;
use yew::prelude::*;
use crate::components::icons::{icon_chevron_left, icon_chevron_right};

// Wrap Html in Rc so we can derive PartialEq via ptr_eq.
#[derive(Clone)]
pub struct CarouselSlide {
    pub content: Rc<Html>,
}

impl PartialEq for CarouselSlide {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.content, &other.content)
    }
}

impl CarouselSlide {
    pub fn new(content: Html) -> Self {
        Self { content: Rc::new(content) }
    }
}

#[derive(Clone, PartialEq, Default)]
enum SlideDir {
    #[default]
    Right, // Next  → new slide enters from the right
    Left,  // Prev  → new slide enters from the left
}

#[derive(Properties, PartialEq)]
pub struct CarouselProps {
    // --- Content ---
    pub slides: Rc<Vec<CarouselSlide>>,

    // --- State ---
    #[prop_or(true)]
    pub show_controls: bool,
    #[prop_or(true)]
    pub show_indicators: bool,
    #[prop_or_default]
    pub autoplay: bool,
    #[prop_or(3000u16)]
    pub interval_ms: u16,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,
}

pub enum CarouselMsg {
    Next,    // user action — resets autoplay timer
    Prev,    // user action — resets autoplay timer
    GoTo(usize), // user action — resets autoplay timer
    Tick,    // autoplay action — does NOT reset timer
}

pub struct Carousel {
    current: usize,
    dir: SlideDir,
    _interval: Option<Interval>,
}

impl Carousel {
    fn make_interval(ctx: &Context<Self>) -> Option<Interval> {
        let p = ctx.props();
        if p.autoplay && !p.slides.is_empty() {
            let link = ctx.link().clone();
            Some(Interval::new(p.interval_ms as u32, move || {
                link.send_message(CarouselMsg::Tick);
            }))
        } else {
            None
        }
    }
}

impl Component for Carousel {
    type Message = CarouselMsg;
    type Properties = CarouselProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            current: 0,
            dir: SlideDir::Right,
            _interval: Self::make_interval(ctx),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let len = ctx.props().slides.len();
        if len == 0 {
            return false;
        }
        match msg {
            CarouselMsg::Next => {
                self.dir = SlideDir::Right;
                self.current = (self.current + 1) % len;
                self._interval = Self::make_interval(ctx);
            }
            CarouselMsg::Prev => {
                self.dir = SlideDir::Left;
                self.current = (self.current + len - 1) % len;
                self._interval = Self::make_interval(ctx);
            }
            CarouselMsg::GoTo(i) => {
                if i == self.current {
                    return false;
                }
                self.dir = if i > self.current { SlideDir::Right } else { SlideDir::Left };
                self.current = i;
                self._interval = Self::make_interval(ctx);
            }
            CarouselMsg::Tick => {
                self.dir = SlideDir::Right;
                self.current = (self.current + 1) % len;
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let p = ctx.props();
        if p.autoplay != old_props.autoplay || p.interval_ms != old_props.interval_ms {
            self._interval = Self::make_interval(ctx);
        }
        let len = p.slides.len();
        if len > 0 && self.current >= len {
            self.current = len - 1;
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let len = p.slides.len();

        if len == 0 {
            return html! {};
        }

        let slide_html = (*p.slides[self.current].content).clone();
        let current = self.current;

        // key= forces Yew to remount the element on each slide change,
        // which re-triggers the CSS animation from the start.
        let anim_cls = match self.dir {
            SlideDir::Right => "yei-carousel-enter-right",
            SlideDir::Left  => "yei-carousel-enter-left",
        };

        let on_prev = ctx.link().callback(|_: MouseEvent| CarouselMsg::Prev);
        let on_next = ctx.link().callback(|_: MouseEvent| CarouselMsg::Next);

        let base = "relative overflow-hidden rounded-lg";
        let outer = classes!(base, p.class.clone());

        html! {
            <div class={outer}>
                <div key={current} class={anim_cls}>
                    { slide_html }
                </div>

                if p.show_controls && len > 1 {
                    <button
                        onclick={on_prev}
                        aria-label="Previous slide"
                        class="absolute left-3 top-1/2 -translate-y-1/2 flex items-center justify-center w-9 h-9 rounded-full bg-background/80 border border-border text-foreground hover:bg-background transition-colors shadow-sm"
                    >
                        { icon_chevron_left() }
                    </button>

                    <button
                        onclick={on_next}
                        aria-label="Next slide"
                        class="absolute right-3 top-1/2 -translate-y-1/2 flex items-center justify-center w-9 h-9 rounded-full bg-background/80 border border-border text-foreground hover:bg-background transition-colors shadow-sm"
                    >
                        { icon_chevron_right() }
                    </button>
                }

                if p.show_indicators && len > 1 {
                    <div class="absolute bottom-3 left-1/2 -translate-x-1/2 flex gap-1.5">
                        { for (0..len).map(|i| {
                            let on_dot = ctx.link().callback(move |_: MouseEvent| CarouselMsg::GoTo(i));
                            let dot_cls = if i == current {
                                "w-2.5 h-2.5 rounded-full bg-primary transition-all"
                            } else {
                                "w-2 h-2 rounded-full bg-primary/30 hover:bg-primary/60 transition-all cursor-pointer"
                            };
                            html! {
                                <button
                                    key={i}
                                    onclick={on_dot}
                                    aria-label={format!("Go to slide {}", i + 1)}
                                    class={dot_cls}
                                />
                            }
                        }) }
                    </div>
                }
            </div>
        }
    }
}
