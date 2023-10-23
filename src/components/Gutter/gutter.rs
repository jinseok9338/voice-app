use leptos::*;

#[component]
pub fn Gutter(space: i32) -> impl IntoView {
    view! {
        <div
        style=format!("height: {}px", space)
        style=format!("width: {}%", 100)
        ></div>
    }
}
