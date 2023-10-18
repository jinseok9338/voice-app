use leptos::{ev::MouseEvent, *};

#[component]
pub fn PrimaryButton(
    text: String,
    #[prop(optional)] mut on_click: Option<Box<dyn FnMut(MouseEvent) + 'static>>,
    #[prop(optional)] disabled: bool,
) -> impl IntoView {
    view! {
        <button class="primary_button subheading-2"
        disabled=disabled
         on:click= move |ev| {
            if let Some(ref mut f) = on_click {
                f(ev);
            }
        }>{text}</button>
    }
}

#[component]
pub fn SecondaryButton(
    text: String,
    #[prop(optional)] mut on_click: Option<Box<dyn FnMut(MouseEvent) + 'static>>,
    #[prop(optional)] disabled: bool,
) -> impl IntoView {
    view! {
        <button class="secondary_button subheading-2"
        disabled=disabled
         on:click= move |ev| {
            if let Some(ref mut f) = on_click {
                f(ev);
            }
        }>{text}</button>
    }
}

#[component]
pub fn GhostButton(
    text: String,
    #[prop(optional)] mut on_click: Option<Box<dyn FnMut(MouseEvent) + 'static>>,
    #[prop(optional)] disabled: bool,
) -> impl IntoView {
    view! {
        <button class="ghost_button subheading-2"
        disabled=disabled
         on:click= move |ev| {
            if let Some(ref mut f) = on_click {
                f(ev);
            }
        }>{text}</button>
    }
}
