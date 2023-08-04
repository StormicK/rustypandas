use leptos::*;
use wasm_bindgen::prelude::*;

use crate::components::color_scheme_select::ColorSchemeSelectComponent;
use crate::components::gif_search::GifSearchComponent;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn MainComponent(cx: Scope) -> impl IntoView {
    view! { cx,
        <main class="container">
            <head>
                <title>Theme Refresh</title>
            </head>
            <body>
                <h1>Gif Picker</h1>
                <GifSearchComponent/>
                <h1>Color Scheme Picker</h1>
                <ColorSchemeSelectComponent/>
            </body>
        </main>
    }
}