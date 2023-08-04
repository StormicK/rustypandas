mod configuration;
mod components;

use configuration::*;
use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <MainComponent/>
        }
    })
}
