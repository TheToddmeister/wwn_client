use leptos::*;
use crate::components::sidebar::SidebarLink;
use crate::style::state::ThemeToggle;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="relative flex h-screen flex-col bg-background">
            <div class="flex-1 items-start grid grid-cols-[180px_minmax(0,1fr)]  lg:grid-cols-[200px_minmax(0,1fr)]">
                <aside class="h-screen w-full shrink-0 border-r">
                    <div class="relative overflow-hidden h-full py-6 pr-6 lg:py-8 px-2 md:px-4">
                        <SidebarLink href="/">Home</SidebarLink>
                        <div class="text-sm">
                            <SidebarLink href="/intro">Why Leptos Query</SidebarLink>
                        </div>
                        <div class="py-2"></div>
                        <h4 class="rounded-md px-2 py-1 text-base font-semibold">Examples</h4>
                        <div class="grid grid-flow-row auto-rows-max text-sm">
                            <SidebarLink href="/single">Single Query</SidebarLink>
                            <SidebarLink href="/todos">Optimistic Update</SidebarLink>
                        </div>
                        <div class="absolute bottom-4 flex flex-col items-start gap-2">
                            <ThemeToggle/>
                        </div>
                    </div>
                </aside>
                <main class="container relative py-6 lg:py-8 overflow-y-auto max-h-screen">
                    {children()}
                </main>
            </div>
        </div>
    }
}