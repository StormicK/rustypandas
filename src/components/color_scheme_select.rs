use leptos::ev::Event;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct ColorSchemeArgs {
    color_scheme: String,
}

fn load_data() -> Vec<String> {
    vec![
        String::from("Campbell"),
        String::from("Campbell Powershell"),
        String::from("One Half Dark"),
        String::from("One Half Light"),
        String::from("Solarized Dark"),
        String::from("Solarized Light"),
        String::from("Tango Dark"),
        String::from("Tango Light"),
        String::from("Vintage"),
        String::from("RedPanda"),
    ]
}

#[component]
pub fn ColorSchemeSelectComponent(cx: Scope) -> impl IntoView {
    let color_schemes = load_data(); //create_resource(cx, || (), |_| async move { load_data().await })
                                     // .read(cx)
                                     // .unwrap();

    let color_scheme_signals = color_schemes.iter().map(|color_scheme| {
        let color_scheme = color_scheme.clone();
        create_signal(cx, color_scheme)
    });

    let select_options = color_scheme_signals
        .map(|(color_scheme, _)| {
            view! { cx,
                <option value={move || color_scheme.get()}>{move || color_scheme.get()}</option>
            }
        })
        .collect::<Vec<_>>();

    let update_color_scheme = move |ev: Event| {
        ev.prevent_default();
        spawn_local(async move {
            let v = event_target_value(&ev);

            let args = to_value(&ColorSchemeArgs { color_scheme: v }).unwrap();

            invoke("update_color_scheme", args).await;
        });
    };

    view! { cx,
        <select id="color-scheme-input" on:input=update_color_scheme>
            { select_options }
        </select>
    }
}
