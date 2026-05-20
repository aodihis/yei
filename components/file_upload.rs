use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{DataTransfer, File, HtmlInputElement};
use crate::components::icons::icon_upload;

// --- Variant ---

#[derive(Clone, PartialEq, Default)]
pub enum FileUploadVariant {
    #[default]
    DropZone,
    Button,
}

// --- Props ---

#[derive(Properties, PartialEq)]
pub struct FileUploadProps {
    // --- State ---
    #[prop_or_default]
    pub multiple: bool,
    #[prop_or_default]
    pub accept: AttrValue,
    #[prop_or_default]
    pub disabled: bool,

    // --- Style ---
    #[prop_or_default]
    pub variant: FileUploadVariant,
    #[prop_or_default]
    pub class: Classes,

    // --- Callbacks ---
    #[prop_or_default]
    pub onchange: Callback<Vec<File>>,
}

// --- Component ---

#[function_component(FileUpload)]
pub fn file_upload(props: &FileUploadProps) -> Html {
    let dragging  = use_state(|| false);
    let input_ref = use_node_ref();

    // Shared: fires when files are selected via input element
    let on_input_change = {
        let onchange = props.onchange.clone();
        Callback::from(move |e: Event| {
            if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                if let Some(list) = input.files() {
                    onchange.emit(collect_files(list));
                }
            }
        })
    };

    match props.variant {
        FileUploadVariant::Button   => render_button(props, &input_ref, &on_input_change),
        FileUploadVariant::DropZone => render_dropzone(props, &dragging, &input_ref, &on_input_change),
    }
}

// --- Button variant ---

fn render_button(
    props: &FileUploadProps,
    input_ref: &NodeRef,
    on_input_change: &Callback<Event>,
) -> Html {
    let input_ref2 = input_ref.clone();
    let onclick = Callback::from(move |_: MouseEvent| {
        if let Some(input) = input_ref2.cast::<HtmlInputElement>() {
            input.click();
        }
    });

    let btn_base = "inline-flex items-center gap-2 px-4 py-2 rounded-md border border-border bg-background text-sm font-medium text-foreground hover:bg-muted transition-colors cursor-pointer";
    let btn_cls  = classes!(btn_base, if props.disabled { "opacity-50 pointer-events-none" } else { "" }, props.class.clone());

    html! {
        <>
            <button type="button" class={btn_cls} onclick={onclick} disabled={props.disabled}>
                { icon_upload() }
                {"Upload file"}
            </button>
            { hidden_input(props, input_ref, on_input_change) }
        </>
    }
}

// --- DropZone variant ---

fn render_dropzone(
    props: &FileUploadProps,
    dragging: &UseStateHandle<bool>,
    input_ref: &NodeRef,
    on_input_change: &Callback<Event>,
) -> Html {
    let ondragover = {
        let dragging = dragging.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            dragging.set(true);
        })
    };
    let ondragleave = {
        let dragging = dragging.clone();
        Callback::from(move |_: DragEvent| dragging.set(false))
    };
    let ondrop = {
        let dragging  = dragging.clone();
        let onchange  = props.onchange.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            dragging.set(false);
            if let Some(dt) = e.data_transfer() {
                if let Some(list) = DataTransfer::files(&dt) {
                    onchange.emit(collect_files(list));
                }
            }
        })
    };

    let input_ref2 = input_ref.clone();
    let onclick = Callback::from(move |_: MouseEvent| {
        if let Some(input) = input_ref2.cast::<HtmlInputElement>() {
            input.click();
        }
    });

    let zone_base   = "flex flex-col items-center justify-center gap-3 w-full rounded-lg border-2 border-dashed p-8 text-center transition-colors cursor-pointer";
    let zone_state  = if **dragging {
        "border-primary bg-primary/5 text-foreground"
    } else {
        "border-border bg-muted/30 text-foreground/60 hover:border-primary/50 hover:bg-muted/50"
    };
    let zone_dis    = if props.disabled { "opacity-50 pointer-events-none" } else { "" };
    let zone_cls    = classes!(zone_base, zone_state, zone_dis, props.class.clone());

    html! {
        <div
            class={zone_cls}
            {ondragover}
            {ondragleave}
            {ondrop}
            {onclick}
        >
            <span class="text-2xl text-foreground/40">{ icon_upload() }</span>
            <div>
                <p class="text-sm font-medium text-foreground">{"Drop files here"}</p>
                <p class="text-xs text-foreground/50 mt-0.5">{"or click to browse"}</p>
            </div>
            if !props.accept.is_empty() {
                <p class="text-xs text-foreground/40">{ format!("Accepted: {}", props.accept) }</p>
            }
            { hidden_input(props, input_ref, on_input_change) }
        </div>
    }
}

// --- Shared helpers ---

fn hidden_input(props: &FileUploadProps, input_ref: &NodeRef, onchange: &Callback<Event>) -> Html {
    html! {
        <input
            ref={input_ref.clone()}
            type="file"
            class="hidden"
            multiple={props.multiple}
            accept={props.accept.clone()}
            disabled={props.disabled}
            onchange={onchange.clone()}
        />
    }
}

fn collect_files(list: web_sys::FileList) -> Vec<File> {
    (0..list.length()).filter_map(|i| list.get(i)).collect()
}
