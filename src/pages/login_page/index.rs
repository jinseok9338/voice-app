use crate::components::Gutter::gutter::Gutter;
use leptos::*;
use leptos_image::Image;
use twust::tw;
#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <div class=tw!("w-full")>
        <Gutter space=90/>
        <img  src="/welcome.svg"/>
        </div>
    }
}
