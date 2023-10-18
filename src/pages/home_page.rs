use crate::{
    components::Buttons::buttons::{GhostButton, PrimaryButton, SecondaryButton},
    i18n::*,
};
use leptos::*;
/// Renders the home page of your application.

#[component]
pub fn HomePage() -> impl IntoView {
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
