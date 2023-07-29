mod configuration;

use configuration::*;
use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <Configuration/>
        }
    })
}
