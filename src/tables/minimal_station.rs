#[allow(warnings)]
use itertools::Itertools;
use leptos::{component, IntoView, SignalGet, view};
use leptos::*;
use leptos_query::*;
use leptos_struct_table::*;
use wwn_shared_utils::location::Location;
use wwn_shared_utils::mapping::Regulation::NOTDOWNLOADED;
use wwn_shared_utils::station::{Station, StationFilter};

use crate::components::{Loud, Spinner};
use crate::style::definitions::HEADER_CLASS;
use crate::style::tables::TableDecorationPreset;
use crate::table_query;

/// This generates the component MinimalStationTable
#[derive(TableRow, Clone)]
#[table(
sortable,
impl_vec_data_provider,
classes_provider = "TableDecorationPreset")]
pub struct StationList {
    pub id: String,
    pub river: Vec<String>,
    pub parameters: Vec<String>,
    pub regulation: String,
    pub active: bool,
    pub hierarchy: Vec<String>,
    pub classification: Option<String>,
    pub origin: String,
    pub coordinates: String,
    pub location_name: String,
    pub nation: String,
    pub description: String,
}

impl StationList {
    pub fn from_station(s: &Station) -> Self {
        let parameters = s.station_parameters.iter()
            .map(|a| a.parameter.to_string())
            .collect_vec();
        let l = &s.location;
        Self {
            id: s.source_id.to_string(),
            river: s.river_name.to_vec(),
            parameters,
            regulation: s.regulation_status.to_string(),
            active: s.status,
            hierarchy: s.parental_hierarchy.to_vec(),
            classification: s.station_type.to_owned(),
            origin: s.origin.to_string(),
            coordinates: format!("{},{}", l.coordinates.0, l.coordinates.1),
            location_name: l.name.to_string(),
            nation: l.nation.to_string(),
            description: l.description.to_string(),
        }
    }
    pub fn from_location(l: &Location) -> Self {
        Self {
            id: l.id.1.to_string(),
            river: vec![],
            parameters: vec![],
            regulation: NOTDOWNLOADED.to_string(),
            active: true,
            hierarchy: vec![],
            classification: None,
            origin: l.origin.to_string(),
            coordinates: format!("{},{}", l.coordinates.0, l.coordinates.1),
            location_name: l.name.to_string(),
            nation: l.nation.to_string(),
            description: l.description.to_string(),
        }
    }
}

#[component]
pub fn StationListApp() -> impl IntoView {
    let query = table_query::<Station>().use_query(move || StationFilter::test_default());
    let data = query.data;
    let fetching = query.is_fetching;
    view! {
       <div>
           <Transition
               fallback=move || {
                   view! { <h2>"Loading..."</h2> }
               }>
               {move || {data.get().map(|result| {
                   match result {Ok(v)=>{
                       let table = v.iter().map(|q| StationList::from_station(q)).collect_vec();
                       view! {
                           <div>
                           <h2>{v.len()}</h2>
                           <table>
                                <TableContent rows=table/>
                           </table>
                           <h2>{v.len()}</h2>
                           </div>
                       }},
                       Err(e) => view! { <div><h2>e</h2></div> } }

                        })
               }}
           </Transition>
       </div>
    }
}