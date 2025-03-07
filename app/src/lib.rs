use sycamore::prelude::*;

mod api;
mod assets;
mod components;
mod context;
mod error;
mod future;
mod meta;
mod pages;
mod pob;
mod response_context;
mod router;
mod utils;

#[cfg(feature = "ssr")]
mod head;

pub use context::Context;
pub use error::{Error, Result};
pub use meta::Meta;
pub use response_context::ResponseContext;
pub use router::Route;

use components::ThemeToggle;

#[cfg(feature = "ssr")]
pub fn render_to_string(context: Context) -> (String, ResponseContext) {
    ResponseContext::with(|| sycamore::render_to_string(|| view! { App(Some(context)) }))
}

#[cfg(feature = "ssr")]
pub fn render_head(meta: Meta) -> String {
    let mut result = sycamore::render_to_string(|| view! { head::Head(meta) });

    // workaround to replace data-hk with data-xx to not interfer with hydration
    let bytes = unsafe { result.as_bytes_mut() };
    static DATA_HK: &[u8] = b"data-hk";
    for i in 0..(bytes.len() - DATA_HK.len()) {
        if &bytes[i..i + DATA_HK.len()] == DATA_HK {
            bytes[i + 5] = b'x';
            bytes[i + 6] = b'x';
        }
    }

    result
}

#[component(App<G>)]
pub fn app(ctx: Option<Context>) -> View<G> {
    // we need to manually handle clicking here, since the nav isn't wrapped in a router
    let navigate_index = |ev: web_sys::Event| {
        sycamore_router::navigate("/");
        ev.prevent_default();
    };

    view! {
        div {
            nav(class="flex p-4 lg:px-8 mb-10 bg-slate-200 dark:bg-slate-900 dark:drop-shadow-lg") {
                a(class="flex-auto", href="/", on:click=navigate_index) { "POB B.in" }
                ThemeToggle()
            }
            div(class="max-w-screen-xl mx-auto px-5 xl:px-0") {
                router::Router(ctx)
            }
        }
    }
}
