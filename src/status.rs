use leptos::*;
use leptos_struct_table::*;
use leptos_router::*;

#[component]
pub fn AppStatus()-> impl IntoView{
    view! {
        <div hx-get="127:0.0.1:1337/status/octagon">
        </div>
    }
}