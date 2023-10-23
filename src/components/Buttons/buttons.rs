use leptos::{ev::MouseEvent, *};

pub enum ClassString {
    PrimaryButton,
    SecondaryButton,
    GhostButton,
}

impl From<ClassString> for String {
    fn from(class: ClassString) -> Self {
        match class {
            ClassString::PrimaryButton => "primary_button".to_string(),
            ClassString::SecondaryButton => "secondary_button".to_string(),
            ClassString::GhostButton => "ghost_button".to_string(),
        }
    }
}

#[component]
pub fn Button(
    text: String,
    class: ClassString,
    #[prop(optional)] mut on_click: Option<Box<dyn FnMut(MouseEvent) + 'static>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] style: String,
) -> impl IntoView {
    let class = String::from(class);
    view! {
        <button class=(class + " subheading-2")
        disabled=disabled
        style=style.clone()
         on:click= move |ev| {
            if let Some(ref mut f) = on_click {
                f(ev);
            }
        }>{text}</button>
    }
}
