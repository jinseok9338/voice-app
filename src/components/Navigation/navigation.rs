use leptos::{ev::MouseEvent, *};

use crate::components::Gutter::gutter::Gutter;

#[component]
pub fn TopNaivation(
    title: String,

    #[prop(optional)] mut on_click: Option<Box<dyn FnMut(MouseEvent) + 'static>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] style: String,
) -> impl IntoView {
    view! {
        <Gutter space=20/>
        <Gutter space=14/>
        <div class="top_navigation">
            <div class="top_navigation_title subheading-1">{title}</div>
            <div class="top_navigation_menu">
                <div class="top_navigation_menu_item">{"Home"}</div>
                <div class="top_navigation_menu_item">{"About"}</div>
                <div class="top_navigation_menu_item">{"Contact"}</div>
            </div>
        </div>
    }
}
