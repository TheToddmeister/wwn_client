use leptos::*;
use leptos_meta::Html;
use leptos_theme::{Theme, use_theme};

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let current_theme = use_theme();
    let is_dark = Signal::derive(move || current_theme.get() == Theme::Dark);

    view! {
        <Html class=move || if is_dark.get() { "dark" } else { "" }/>
        <label
            for="dark-mode-toggle"
            class="text-xs font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
        >
            Dark Mode
        </label>
        <Switch
            enabled=is_dark
            on_click=Callback::new(move |_| {
                let new_theme = if is_dark.get() { Theme::Light } else { Theme::Dark };
                current_theme.set(new_theme);
            })

            attr:id="dark-mode-toggle"
        />
    }
}

#[component]
pub fn Switch(
    #[prop(into)] enabled: Signal<bool>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    #[prop(into)] on_click: Callback<()>,
) -> impl IntoView {
    let button_class = move || {
        let button_class = if enabled.get() {
            "bg-primary"
        } else {
            "bg-gray-200"
        };
        format!("{} {}", "bg-gray-200 relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2", button_class)
    };

    let span_class = move || {
        let span_class = if enabled.get() {
            "translate-x-5"
        } else {
            "translate-x-0"
        };
        format!("{} {}", "translate-x-0 pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out", span_class)
    };

    view! {
        <button type="button" {..attributes} class=button_class on:click=move |_| { on_click(()) }>
            <span aria-hidden="true" class=span_class></span>
        </button>
    }
}
