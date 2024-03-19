use itertools::Itertools;
use leptos::*;
use leptos::{component, create_local_resource, IntoView, SignalGet, view};
use leptos_struct_table::*;
use wwn_shared_utils::river::River;

use crate::data::fetch_rivers;
use crate::style::tables::TableDecorationPreset;

#[derive(TableRow, Clone)]
#[table(
sortable,
impl_vec_data_provider,
classes_provider ="TableDecorationPreset")]
pub struct RiverList {
    pub river: String,
    pub alias: Vec<String>,
    pub tributary_hierarchy: Vec<String>,
    pub drainage_baisin: Option<String>,
    pub catchment_area: Option<u64>,
    pub nation: String,

}

impl RiverList {
    pub fn from_river(r: &River) -> Self {
        Self {
            river: r.name.to_string(),
            alias: r.alias.to_owned(),
            tributary_hierarchy: r.tributary_hierarchy.to_owned(),
            drainage_baisin: r.drainage_basin.to_owned(),
            nation: r.origin.to_nation().to_string(),
            catchment_area: r.catchment_area.to_owned(),
        }
    }
}

#[component]
pub fn RiverListTable() -> impl IntoView {
    let river_resource = create_local_resource(|| (), |_| async move { fetch_rivers().await });

    let data = move || match river_resource.get() {
        Some(Ok(vs)) => {
            let rows = vs.iter()
                .map(|r| RiverList::from_river(r))
                .collect_vec();
            let view = view! {
                    <table>
                        <TableContent rows/>
                     </table>
                };
            view.into_view()
        }
        None => {
            let waiting_message = "Waiting ".to_string();
            let view = view! {<p> {waiting_message} </p>};
            view.into_view()
        }
        Some(Err(_)) => {
            let error_message = "ErrorMessage: ".to_string();
            let view = view! {<p> {error_message} </p>};
            view.into_view()
        }
    };
    view! {
        <div>
                <div>
                    <h1> "something" </h1>
                    {data}
                </div>
        </div>
        }
}
