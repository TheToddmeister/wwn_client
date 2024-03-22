use leptos::{Signal, SignalGet, SignalSet, WriteSignal};
use leptos_use::storage::use_local_storage;
use leptos_use::utils::JsonCodec;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use wwn_shared_utils::{DataTable, Filter};
use wwn_shared_utils::location::LocationFilter;
use wwn_shared_utils::river::RiverFilter;
use wwn_shared_utils::station::{Station, StationFilter};
use wwn_shared_utils::timeseries::TimeSeriesFilter;

pub struct Filters{
    pub station: (Signal<StationFilter>, WriteSignal<StationFilter>),
    pub river: (Signal<RiverFilter>, WriteSignal<RiverFilter>),
    pub location: (Signal<LocationFilter>, WriteSignal<LocationFilter>),
    pub timeseries: (Signal<TimeSeriesFilter>, WriteSignal<TimeSeriesFilter>),
}

pub struct StaticLocalData{
    station: Signal<Result<Vec<Station>, String>>,
}
impl Filters{
    pub fn get_local()->Self{
        let (r,w,_) = use_local_storage::<StationFilter, JsonCodec>("StationFilter");
        let station = (r,w);
        let (r,w,_) = use_local_storage::<RiverFilter, JsonCodec>("RiverFilter");
        let river = (r,w);
        let (r,w,_) = use_local_storage::<LocationFilter, JsonCodec>("LocationFilter");
        let location = (r,w);
        let (r,w,_) = use_local_storage::<TimeSeriesFilter, JsonCodec>("TimeSeriesFilter");
        let timeseries = (r,w);
        Self{
            station,
            river,
            location,
            timeseries,
        }
    }
}
pub fn store_local_filter<F:Filter + Serialize + DeserializeOwned + Eq + Default + Clone>(filter: F, storage_location: String)-> Result<Signal<F>, wwn_shared_utils::Error>{
    let filter_name = storage_location;
    let (r, w, _) = use_local_storage::<F, JsonCodec>(filter_name);
    w.set(filter);
    Ok(r)
}
