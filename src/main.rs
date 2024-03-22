#![allow(warnings)]


use leptos::*;
use leptos_router::*;
use leptos::tracing::*;
use leptos_theme::ThemeProvider;
use leptos_struct_table::*;
use leptos_meta::*;
use wwn_shared_utils;
use gloo_net;
use leptos_query::provide_query_client;
use wwn_shared_utils::station::Station;
use frontend::components::layout::Layout;

use frontend::components::not_found::NotFound;
use frontend::filter::Filters;
use frontend::home::Home;
use frontend::table_query;
use frontend::tables::minimal_station::StationListApp;

fn main() {
    // set up logging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
pub fn App() -> impl IntoView {
    provide_query_client();
    let filters = Filters::get_local();
    let stations = create_effect(move |_| {
        let filter = filters.station.0.get();
        provide_context(filters.station.0);
        let binding = table_query::<Station>();
       let fut = binding.prefetch_query(filter);
    });
    view! {
        <Stylesheet id="leptos" href="/output.css"/>
        <ThemeProvider>
            <Layout>
                <Router>
                    <nav>
                      <A href="home">"Home"</A>
                      <A href="not_found" class="my-class">"NotFound"</A>
                    </nav>
                     <main>
                        <Routes>
                            <Route path="/" view=NotFound/>
                            <Route path="/home" view=Home/>
                            <Route path="/stations" view=StationListApp/>
                            <Route path="/not_found" view=NotFound/>
                            <Route path="/*" view=NotFound/>
                        </Routes>
                     </main>
                </Router>
            </Layout>
        </ThemeProvider>
    }
}

