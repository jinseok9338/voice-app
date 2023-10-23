use crate::{
    i18n::*,
    pages::{home_page::HomePage, login_page::index::LoginPage, not_found::NotFound},
};
use leptos::*;

use leptos_image::provide_image_context;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_image_context();
    provide_i18n_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=LoginPage ssr=SsrMode::OutOfOrder   />
                    // <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
