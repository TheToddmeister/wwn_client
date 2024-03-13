#[allow(warnings)]
use itertools::Itertools;
use leptos::{component, create_local_resource, IntoView, SignalGet, view};
use leptos::*;
use leptos_struct_table::*;
use wwn_shared_utils::location::{Location, LocationFilter};
use wwn_shared_utils::location::LocationType::MeasuringStation;
use wwn_shared_utils::mapping::Origin::CANADA;
use wwn_shared_utils::mapping::Regulation::NOTDOWNLOADED;
use wwn_shared_utils::station::Station;
use crate::style::tables::TableDecorationPreset;

use crate::data::{fetch_locations, fetch_stations};

/// This generates the component MinimalStationTable
#[derive(TableRow, Clone)]
#[table(
sortable,
impl_vec_data_provider,
classes_provider ="TableDecorationPreset")]
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
            id: s.station_id.to_string(),
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
pub fn MinimalStationList() -> impl IntoView {
    let stations_resource = create_local_resource(|| (), |_| async move { fetch_stations().await });
    let location_resource = create_local_resource(|| (), |_| async move {
        fetch_locations(
            LocationFilter {
                origin: vec![CANADA],
                nation: vec![],
                id: vec![],
                loc_type: vec![MeasuringStation],
            }).await
    });

    let data = move || match (stations_resource.get(), location_resource.get()) {
        (Some(Ok(vs)), Some(Ok(vl))) => {
            let mut srows = vs.iter()
                .map(|s| StationList::from_station(s)).collect_vec();
            let lrows = vl.iter()
                .map(|l| StationList::from_location(l)).collect_vec();
           srows.extend(lrows);
            let view = view! {
                    <table>
                        <TableContent rows=srows/>
                     </table>
                };
            view.into_view()
        }
        (None, _) => {
            let waiting_message = "Waiting ".to_string();
            let view = view! {<p> {waiting_message} </p>};
            view.into_view()
        }
        (_, None) => {
            let waiting_message = "Waiting ".to_string();
            let view = view! {<p> {waiting_message} </p>};
            view.into_view()
        }
        (Some(Err(e)), _) => {
            let error_message = "ErrorMessage: ".to_string() + &e;
            let view = view! {<p> {error_message} </p>};
            view.into_view()
        }

        (_, Some(Err(e))) => {
            let error_message = "ErrorMessage: ".to_string() + &e;
            let view = view! {<p> {error_message} </p>};
            view.into_view()
        }
    };


    view! {
            <div class="rounded-md overflow-clip m-10 border dark:border-gray-700 float-left".to_string()>
        <h1> "something" </h1>
            <table class="text-sm text-left text-gray-500 dark:text-gray-400 mb-[-1px]">
                {data}
            </table>
        </div>
        }
}
