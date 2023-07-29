use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn Configuration(cx: Scope) -> impl IntoView {
    let refresh_panda = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            invoke("refresh_panda", JsValue::UNDEFINED)
                .await;
        });
    };

    view! { cx,
        <main class="container">
            <head>
                <title>Red Panda Theme Refresh</title>
            </head>
            <body>
                <h1>Red Panda Theme Refresh</h1>
                <form on:submit=refresh_panda>
                    <button type="submit">"Refresh"</button>
                </form>
            </body>
        </main>
    }
}