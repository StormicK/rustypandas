use leptos::{leptos_dom::ev::SubmitEvent, ev::Event};
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use chrono::prelude::*;


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
pub fn GifSearchComponent(cx: Scope) -> impl IntoView {
    let (search_query, set_search_query) = create_signal(cx, String::from("red panda "));
    let (last_updated, set_last_updated) = create_signal(cx, String::from(""));

    let update_search_query = move |ev: Event| {
        let v = event_target_value(&ev);
        set_search_query.set(v);
    };

    let update_gif = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if search_query.get().is_empty() {
                return;
            }

            let args = to_value(&SearchArgs {
                search_query: search_query.get(),
            })
            .unwrap();
            
            invoke("update_gif", args).await;
        });
    };

    view! { cx,
        <form on:submit=update_gif>
            <input
                id="search-input"
                placeholder="Enter a name..."
                autocomplete="off"
                on:input=update_search_query
            />
            <button type="submit">"Refresh"</button>
            <p>{ move || last_updated.get() }</p>
        </form>
    }
}
