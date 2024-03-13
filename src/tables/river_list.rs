use itertools::Itertools;
use leptos::{component, create_local_resource, IntoView, SignalGet, view};
use leptos_struct_table::*;

use wwn_shared_utils::location::LocationFilter;
use wwn_shared_utils::location::LocationType::MeasuringStation;
use wwn_shared_utils::mapping::Origin::CANADA;
use wwn_shared_utils::river::River;
use crate::data::{fetch_locations, fetch_stations};
use crate::tables::minimal_station::StationList;


pub struct RiverList{
    pub river: String,
    pub alias: Vec<String>,
    pub tributary_hierarchy: Vec<String>,
    pub drainage_baisin: Option<String>,
    pub catchment_area: Option<u64>,
    pub nation: String,

}
impl RiverList {
    pub fn from_river(r: &River)->Self{
        Self{
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
pub fn RiverList() -> impl IntoView {
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
            let rows = srows.extend(lrows);
            let view = view! {
                    <table>
                        <TableContent rows/>
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
        (Some(Err(_)), _) => {
            let error_message = "ErrorMessage: ".to_string();
            let view = view! {<p> {error_message} </p>};
            view.into_view()
        }

        (_, Some(Err(_))) => {
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
