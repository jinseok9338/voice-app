use crate::{
    components::{
        Buttons::buttons::{Button, ClassString},
        Navigation::navigation::TopNaivation,
    },
    i18n::*,
};
use leptos::*;
/// Renders the home page of your application.

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <TopNaivation title={"Leptos".to_string()} />
        <img src="/favicon.ico"/>
    }
}
