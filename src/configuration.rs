use leptos::leptos_dom::ev::SubmitEvent;
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
struct SearchArgs {
    search_query: String,
}

#[component]
pub fn Configuration(cx: Scope) -> impl IntoView {
    let (search_query, set_search_query) = create_signal(cx, String::from("red panda "));
    let update_search_query = move |ev| {
        let v = event_target_value(&ev);
        set_search_query.set(v);
    };

    let refresh_panda = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if search_query.get().is_empty() {
                return;
            }

            let args = to_value(&SearchArgs {
                search_query: search_query.get(),
            }).unwrap();

            invoke("refresh_panda", args)
                .await;
        });
    };

    view! { cx,
        <main class="container">
            <head>
                <title>Red Panda Theme Refresh</title>
            </head>
            <body>
                <h1>Theme Refresher</h1>
                <form on:submit=refresh_panda>
                    <input
                        id="search-input"
                        placeholder="Enter a name..."
                        autocomplete="off"
                        on:input=update_search_query
                    />
                    <button type="submit">"Refresh"</button>
                </form>
            </body>
        </main>
    }
}