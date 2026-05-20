use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct SliderProps {
    // --- State ---
    pub value: f64,
    #[prop_or(0.0f64)]
    pub min: f64,
    #[prop_or(100.0f64)]
    pub max: f64,
    #[prop_or(1.0f64)]
    pub step: f64,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub id: AttrValue,

    // --- Style ---
    #[prop_or_default]
    pub class: Classes,

    // --- Callbacks ---
    #[prop_or_default]
    pub onchange: Callback<f64>,
}

#[function_component(Slider)]
pub fn slider(props: &SliderProps) -> Html {
    let onchange = props.onchange.clone();
    let oninput = Callback::from(move |e: InputEvent| {
        if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
            if let Ok(v) = input.value().parse::<f64>() {
                onchange.emit(v);
            }
        }
    });

    let pct   = ((props.value - props.min) / (props.max - props.min) * 100.0).clamp(0.0, 100.0);
    let style = format!("--yei-slider-fill: {pct:.2}%");

    let outer_base = "relative w-full flex items-center py-1";
    let outer_cls  = classes!(outer_base, props.class.clone());

    html! {
        <div class={outer_cls}>
            <input
                type="range"
                class="yei-slider"
                id={props.id.clone()}
                min={props.min.to_string()}
                max={props.max.to_string()}
                step={props.step.to_string()}
                value={props.value.to_string()}
                disabled={props.disabled}
                {oninput}
                style={style}
                aria-valuemin={props.min.to_string()}
                aria-valuemax={props.max.to_string()}
                aria-valuenow={props.value.to_string()}
            />
        </div>
    }
}
