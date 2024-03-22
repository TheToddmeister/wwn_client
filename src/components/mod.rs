use leptos::{Children, component, IntoView, Signal, view};
use leptos::*;
pub mod not_found;
pub mod layout;
pub mod header;
pub mod sidebar;


#[component]
pub fn Loud(children: Children) -> impl IntoView {
    view! { <span class="font-semibold text-amber-600 dark:text-amber-400">{children()}</span> }
}

#[component]
pub fn Skeleton(#[prop(optional, into)] class: String) -> impl IntoView {
    view! { <div class=format!("animate-pulse rounded-md bg-muted-foreground/10 {class}")></div> }
}

#[component]
pub fn Spinner(#[prop(into)] fetching: Signal<bool>) -> impl IntoView {
    view! {
        <Show when=fetching>
            <svg
                class="h-5 w-5 stroke-foreground text-foreground animate-spin"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <line x1="12" x2="12" y1="2" y2="6"></line>
                <line x1="12" x2="12" y1="18" y2="22"></line>
                <line x1="4.93" x2="7.76" y1="4.93" y2="7.76"></line>
                <line x1="16.24" x2="19.07" y1="16.24" y2="19.07"></line>
                <line x1="2" x2="6" y1="12" y2="12"></line>
                <line x1="18" x2="22" y1="12" y2="12"></line>
                <line x1="4.93" x2="7.76" y1="19.07" y2="16.24"></line>
                <line x1="16.24" x2="19.07" y1="7.76" y2="4.93"></line>
            </svg>
        </Show>
    }
}