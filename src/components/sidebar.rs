use leptos::{Children, component, IntoView, view};

#[component]
pub fn SidebarLink(#[prop(into)] href: String, children: Children) -> impl IntoView {
    view! {
        <a
            href=href
            class="group flex w-full items-center rounded-md border border-transparent px-2 py-1 hover:underline font-medium text-foreground/80"
        >
            {children()}
        </a>
    }
}
