//! quang-hub-app — QuangHub application binary entry point.
//!
//! Initializes the Dioxus web application, configures tracing for Wasm,
//! and launches the QuangHub UI. This crate ties together all feature
//! crates (Workplace, Meet, Repo) with the shared quang-web layer.

// Module declarations
pub mod pages;
pub mod router;

use dioxus::prelude::*;
use router::AppRoute;

use quang_web::auth::{restore_session, AuthProvider, UserSession};
use quang_web::client::QuangHubClient;
use quang_web::event::EventBusSignal;

/// Inject CSS and font link into the document head.
fn inject_styles() {
    let document = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };
    let head = match document.head() {
        Some(h) => h,
        None => return,
    };

    // Font link
    let font_link = document.create_element("link").unwrap();
    let _ = font_link.set_attribute("rel", "stylesheet");
    let _ = font_link.set_attribute(
        "href",
        "https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800&display=swap",
    );
    let _ = head.append_child(&font_link);

    // Style element
    let style = document.create_element("style").unwrap();
    style.set_text_content(Some(include_str!("../../../public/styles.css")));
    let _ = head.append_child(&style);
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    tracing_wasm::set_as_global_default();

    dioxus::launch(App);
}

/// Root app component — provides global context + routing.
fn App() -> Element {
    use_effect(move || {
        inject_styles();
    });

    let auth_session: Signal<Option<UserSession>> = use_signal(|| restore_session());
    let event_bus = EventBusSignal::new();
    let api_client = use_signal(|| {
        let client = QuangHubClient::new("/");
        let session = auth_session.read();
        if let Some(user) = session.as_ref() {
            client.with_token(&user.token)
        } else {
            client
        }
    });

    use_context_provider(|| auth_session);
    use_context_provider(|| event_bus);
    use_context_provider(|| api_client);

    rsx! {
        div {
            class: "quang-app",
            AuthProvider {
                Router::<AppRoute> {}
            }
        }
    }
}
