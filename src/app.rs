use crate::{
    components::Buttons::buttons::{GhostButton, SecondaryButton},
    i18n::*,
};
use leptos::*;

use crate::components::Buttons::buttons::PrimaryButton;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
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
                    <Route path="" view=HomePage ssr=SsrMode::OutOfOrder   />
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);

    // let on_click = move |_| {
    //     set_count(count.get() + 1);
    // };

    let i18n = use_i18n();

    let on_switch = move |_| {
        let new_lang = match i18n.get_locale() {
            Locale::ko => Locale::en,
            Locale::en => Locale::ko,
        };
        i18n.set_locale(new_lang);
    };
    logging::log!("where do I run?");

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_switch>{t!(i18n, hello_world, count)}</button>
        <PrimaryButton text={"Button".to_string()} disabled={false}/>
        <SecondaryButton text={"Button".to_string()} disabled={true}/>
        <GhostButton text={"Button".to_string()} disabled={true}/>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

#[component]
fn Example() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server

    view! {
        <h1>"This part in written in ssr mode"</h1>
    }
}
